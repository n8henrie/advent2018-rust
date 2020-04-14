use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, PartialEq, Clone)]
enum BoardSpace {
    Space,
    FallingWater,
    SittingWater,
    Clay,
}

#[derive(Debug, PartialEq)]
struct Board {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    spaces: Vec<BoardSpace>,
}

impl Board {
    fn get(&self, coords: (usize, usize)) -> Option<&BoardSpace> {
        self.spaces.get(coords.0 + (self.max_x + 1) * coords.1)
    }

    fn get_mut(&mut self, coords: (usize, usize)) -> Option<&mut BoardSpace> {
        self.spaces.get_mut(coords.0 + (self.max_x + 1) * coords.1)
    }
}

impl From<Vec<ClayBuilder>> for Board {
    fn from(cbs: Vec<ClayBuilder>) -> Self {
        let min_x = *cbs.iter().flat_map(|cb| &cb.xs).min().unwrap();
        let max_x = *cbs.iter().flat_map(|cb| &cb.xs).max().unwrap();
        let min_y = *cbs.iter().flat_map(|cb| &cb.ys).min().unwrap();
        let max_y = *cbs.iter().flat_map(|cb| &cb.ys).max().unwrap();

        let mut spaces = vec![BoardSpace::Space; (max_x + 1) * (max_y + 1)];

        for cb in cbs {
            for x in &cb.xs {
                for y in &cb.ys {
                    *spaces.get_mut(x + (max_x + 1) * y).unwrap() = BoardSpace::Clay;
                }
            }
        }
        Board {
            min_x,
            max_x,
            min_y,
            max_y,
            spaces,
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spring_loc = 500;
        println!(
            "x: {}-{}, y: {}-{}",
            self.min_x, self.max_x, self.min_y, self.max_y
        );
        println!("{0: >1$}", spring_loc, spring_loc - self.min_x + 2);
        println!("{0: >1$}", "|", spring_loc - self.min_x + 1);
        for (idx, space) in self.spaces.iter().enumerate() {
            let first_clay = self.min_x + (self.max_x * self.min_y);
            let x_idx = idx % (self.max_x + 1);
            let y_idx = idx / (self.max_x + 1);

            if idx > first_clay && x_idx == 0 && y_idx >= self.min_y {
                // write!(f, "\n{}", y_idx)?;
                writeln!(f)?;
            }

            if x_idx >= self.min_x && y_idx >= self.min_y {
                let c = match space {
                    BoardSpace::Clay => "#",
                    BoardSpace::Space => ".",
                    BoardSpace::FallingWater => "|",
                    BoardSpace::SittingWater => "~",
                };
                write!(f, "{}", c)?;
            }
        }
        Ok(())
    }
}

struct ClayBuilder {
    xs: Vec<usize>,
    ys: Vec<usize>,
}

enum StopReason {
    HitWall,
    TooFar,
    StartFalling((usize, usize)),
}

fn propagate(board: Arc<Mutex<Board>>, pos: (usize, usize)) {
    let mut pos = pos;

    {
        // Assume we are here because we are falling
        *board.lock().unwrap().get_mut(pos).unwrap() = BoardSpace::FallingWater;
    }

    // If you can go down, go down as far as you can (should only matter on startup)
    match board.lock().unwrap().get_mut((pos.0, pos.1 + 1)) {
        Some(space @ BoardSpace::Space) => {
            *space = BoardSpace::FallingWater;
            pos.1 += 1;
        }
        Some(BoardSpace::Clay) | Some(BoardSpace::SittingWater) => (),
        Some(BoardSpace::FallingWater) | None => return,
    }

    let b = Arc::clone(&board);
    let left = thread::spawn(move || {
        loop {
            // If you can go down, then it's time to recurse the function
            match b.lock().unwrap().get((pos.0, pos.1 + 1)) {
                Some(BoardSpace::Space) | Some(BoardSpace::FallingWater) => {
                    return StopReason::StartFalling(pos)
                }
                None => return StopReason::TooFar,
                _ => (),
            };

            match b.lock().unwrap().get_mut((pos.0 - 1, pos.1)).unwrap() {
                space @ BoardSpace::Space | space @ BoardSpace::FallingWater => {
                    *space = BoardSpace::FallingWater;
                    pos.0 -= 1;
                }
                _ => return StopReason::HitWall,
            };
        }
    });

    let b = Arc::clone(&board);
    let right = thread::spawn(move || {
        loop {
            match b.lock().unwrap().get((pos.0, pos.1 + 1)) {
                // If you can go down, then it's time to recurse the function
                Some(BoardSpace::Space) | Some(BoardSpace::FallingWater) => {
                    return StopReason::StartFalling(pos)
                }
                None => return StopReason::TooFar,
                _ => (),
            };

            // If you couldn't go down, then go right
            match b.lock().unwrap().get_mut((pos.0 + 1, pos.1)).unwrap() {
                space @ BoardSpace::Space | space @ BoardSpace::FallingWater => {
                    *space = BoardSpace::FallingWater;
                    pos.0 += 1;
                }
                _ => return StopReason::HitWall,
            };
        }
    });
    let (left, right) = (left.join(), right.join());

    // If we hit a wall on both sides, turn both sides into sitting water and take pos up by 1
    match (&left, &right) {
        (Ok(StopReason::HitWall), Ok(StopReason::HitWall)) => {
            let start_pos = pos;

            {
                let mut guard = board.lock().unwrap();
                while let Some(space @ BoardSpace::FallingWater) = guard.get_mut(pos) {
                    *space = BoardSpace::SittingWater;
                    pos.0 -= 1;
                }
                pos = start_pos;
                pos.0 += 1;
                while let Some(space @ BoardSpace::FallingWater) = guard.get_mut(pos) {
                    *space = BoardSpace::SittingWater;
                    pos.0 += 1;
                }
                pos = start_pos;
            };
            pos.1 -= 1;
            propagate(board, pos)
        }
        (Ok(StopReason::TooFar), Ok(StopReason::TooFar)) => {}

        (l, r) => {
            for dir in &[l, r] {
                if let Ok(StopReason::StartFalling(pos)) = dir {
                    propagate(Arc::clone(&board), *pos);
                }
            }
        }
    }
}

fn part1(input: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let board: Board = input.parse()?;

    let pos = (500, board.min_y);
    let b = Arc::new(Mutex::new(board));
    propagate(Arc::clone(&b), pos);

    let water_count = b
        .lock()
        .unwrap()
        .spaces
        .iter()
        .filter(|&space| *space == BoardSpace::FallingWater || *space == BoardSpace::SittingWater)
        .count();
    let standing_water_count = b
        .lock()
        .unwrap()
        .spaces
        .iter()
        .filter(|&space| *space == BoardSpace::SittingWater)
        .count();
    Ok((water_count, standing_water_count))
}

impl FromStr for Board {
    type Err = Box<dyn std::error::Error>;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut cbs: Vec<ClayBuilder> = Vec::new();
        for line in input.lines() {
            let mut cb = ClayBuilder {
                xs: Vec::new(),
                ys: Vec::new(),
            };
            for axis in line.split(", ") {
                let mut parts = axis.split('=');
                if let (Some(axis), Some(coords)) = (parts.next(), parts.next()) {
                    let mut coords = coords.split("..");
                    let coords = match (coords.next(), coords.next()) {
                        (Some(start), Some(end)) => {
                            let (start, end): (usize, usize) = (start.parse()?, end.parse()?);
                            (start..=end).collect::<Vec<_>>()
                        }
                        (Some(start), None) => vec![start.parse::<usize>().unwrap()],
                        _ => {
                            return Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "unable to parse coordinates",
                            )))
                        }
                    };
                    match axis {
                        "y" => cb.ys = coords,
                        "x" => cb.xs = coords,
                        _ => {
                            return Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "unable to parse coordinates",
                            )))
                        }
                    }
                }
            }
            cbs.push(cb)
        }
        Ok(Board::from(cbs))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let input = std::fs::read_to_string("day17/input.txt")?;
    let output = part1(&input)?;
    println!("part1: {}", output.0);
    println!("part2: {}", output.1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_board() {
        let input = "x=4, y=2..7
x=5..8, y=3";
        use BoardSpace::*;
        let board: Board = input.parse().unwrap();
        assert_eq!(
            board,
            Board {
                min_x: 4,
                max_x: 8,
                min_y: 2,
                max_y: 7,
                spaces: vec![
                    Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space,
                    Space, Space, Space, Space, Space, Space, Space, Space, Space, Space, Space,
                    Clay, Space, Space, Space, Space, Space, Space, Space, Space, Clay, Clay, Clay,
                    Clay, Clay, Space, Space, Space, Space, Clay, Space, Space, Space, Space,
                    Space, Space, Space, Space, Clay, Space, Space, Space, Space, Space, Space,
                    Space, Space, Clay, Space, Space, Space, Space, Space, Space, Space, Space,
                    Clay, Space, Space, Space, Space,
                ]
            }
        )
    }

    #[test]
    fn test_print_board() {
        let input = "x=4, y=2..7
x=5..8, y=3";
        let board: Board = input.parse().unwrap();
        assert_eq!(
            board.to_string(),
            "#....
#####
#....
#....
#....
#...."
        );
    }

    #[test]
    fn test_get() {
        /*
         45678
        2#....
        3#####
        4#....
        5#....
        6#....
        7#....

         012345678
        0.........
        1.........
        2....#....
        3....#####
        4....#....
        5....#....
        6....#....
        7....#....
        */

        let input = "x=4, y=2..7
x=5..8, y=3";
        let board: Board = input.parse().unwrap();
        assert_eq!(board.get((3, 2)), Some(&BoardSpace::Space));
        assert_eq!(board.get((5, 2)), Some(&BoardSpace::Space));
        assert_eq!(board.get((4, 1)), Some(&BoardSpace::Space));
        assert_eq!(board.get((4, 2)), Some(&BoardSpace::Clay));
        assert_eq!(board.get((4, 3)), Some(&BoardSpace::Clay));
        assert_eq!(board.get((4, 7)), Some(&BoardSpace::Clay));
        assert_eq!(board.get((8, 3)), Some(&BoardSpace::Clay));
        assert_eq!(board.get((4, 8)), None);
    }

    #[test]
    fn test_get_mut() {
        let input = "x=4, y=2..7
x=5..8, y=3";
        let mut board: Board = input.parse().unwrap();
        assert_eq!(board.get((5, 2)), Some(&BoardSpace::Space));
        if let Some(space) = board.get_mut((5, 2)) {
            *space = BoardSpace::Clay;
        }
        assert_eq!(board.get((5, 2)), Some(&BoardSpace::Clay));
    }

    #[test]
    fn test_both_spill() {
        /*
         012345678910
        0#..........
        1...........
        2...#...#...
        3...#####...
        4...........
        5..........#
        */
        let input = "x=3, y=2..3
x=4..6, y=3
x=7, y=2..3
x=0, y=0
x=10, y=5";
        let board: Board = input.parse().unwrap();
        assert_eq!(
            board.to_string(),
            "#..........
...........
...#...#...
...#####...
...........
..........#"
        );
        let b = Arc::new(Mutex::new(board));
        propagate(Arc::clone(&b), (5, 0));
        println!("{}", b.lock().unwrap().to_string());
        assert_eq!(
            b.lock().unwrap().to_string(),
            "#....|.....
..|||||||..
..|#~~~#|..
..|#####|..
..|.....|..
..|.....|.#"
        );
    }
}
