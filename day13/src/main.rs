// Part 1: 57,104
// Part 2: 67,74
use std::cmp::Ordering;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Clone, Debug)]
struct Cart {
    position: (usize, usize),
    direction: Direction,
    intersection_handler: std::iter::Cycle<std::vec::IntoIter<Turn>>,
}

impl Eq for Cart {}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_position = (self.position.1, self.position.0);
        self_position.cmp(&(other.position.1, other.position.0))
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Cart {
    fn new(position: (usize, usize), direction: char) -> Self {
        use Turn::*;
        Cart {
            position,
            direction: match direction {
                '^' => Direction::Up,
                '<' => Direction::Left,
                '>' => Direction::Right,
                'v' => Direction::Down,
                _ => panic!("Invalid contructor for cart!"),
            },
            intersection_handler: vec![Left, Straight, Right].into_iter().cycle(),
        }
    }
    fn proceed(&mut self, grid: &Grid) {
        match self.direction {
            Direction::Up => self.position.1 -= 1,
            Direction::Down => self.position.1 += 1,
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
        };
        match grid.get(&self.position) {
            Some(GridPoint::Corner(corner)) => match (corner, &self.direction) {
                ('/', Direction::Up) | ('\\', Direction::Down) => self.direction = Direction::Right,
                ('/', Direction::Down) | ('\\', Direction::Up) => self.direction = Direction::Left,
                ('/', Direction::Right) | ('\\', Direction::Left) => self.direction = Direction::Up,
                ('/', Direction::Left) | ('\\', Direction::Right) => {
                    self.direction = Direction::Down
                }
                _ => panic!("Weird corner."),
            },
            Some(GridPoint::Intersection) => {
                match (self.intersection_handler.next().unwrap(), &self.direction) {
                    (Turn::Straight, _) => (),
                    (Turn::Right, Direction::Up) | (Turn::Left, Direction::Down) => {
                        self.direction = Direction::Right
                    }
                    (Turn::Left, Direction::Up) | (Turn::Right, Direction::Down) => {
                        self.direction = Direction::Left
                    }
                    (Turn::Left, Direction::Right) | (Turn::Right, Direction::Left) => {
                        self.direction = Direction::Up
                    }
                    (Turn::Left, Direction::Left) | (Turn::Right, Direction::Right) => {
                        self.direction = Direction::Down
                    }
                }
            }
            None => (),
        }
    }
}

#[derive(Clone)]
enum GridPoint {
    Corner(char),
    Intersection,
}

type Grid = HashMap<(usize, usize), GridPoint>;

fn parse_input(input: &str) -> (Grid, Vec<Cart>) {
    let mut grid: Grid = HashMap::new();
    let mut carts: Vec<Cart> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' | '<' | '>' | 'v' => {
                    carts.push(Cart::new((x, y), c));
                }
                '+' => {
                    grid.insert((x, y), GridPoint::Intersection);
                }
                '/' | '\\' => {
                    grid.insert((x, y), GridPoint::Corner(c));
                }
                _ => (),
            }
        }
    }
    (grid, carts)
}

fn part1(grid: Grid, carts: &mut Vec<Cart>) -> (usize, usize) {
    loop {
        carts.sort();
        for _ in 0..carts.len() {
            let mut cart = carts.remove(0);
            cart.proceed(&grid);
            if carts.iter().any(|c| c == &cart) {
                carts.retain(|c| *c != cart);
                return cart.position;
            } else {
                carts.push(cart);
            }
        }
    }
}

fn part2(grid: Grid, carts: &mut Vec<Cart>) -> (usize, usize) {
    while carts.len() > 1 {
        carts.sort();
        let mut counter = 0;
        let len = carts.len();
        while counter < len {
            let mut cart = carts.remove(0);
            cart.proceed(&grid);
            if let Some(idx) = carts
                .iter()
                .enumerate()
                .filter_map(|(i, c)| if c == &cart { Some(i) } else { None })
                .next()
            {
                carts.remove(idx);

                // Carts at the beginning of the list have not gotten to move yet, but if they are
                // removed, the counter needs to account for them in terms of when to reset the
                // loop. This determines whether or not the cart has moved so far this loop.
                if idx < (len - counter - 1) {
                    counter += 1;
                }
            } else {
                carts.push(cart);
            }
            counter += 1;
        }
    }
    carts[0].position
}

fn main() -> Result<()> {

    let input = std::fs::read_to_string("day13/input.txt")?;
    let (grid, mut carts) = parse_input(&input);
    let (part1x, part1y) = part1(grid.clone(), &mut carts.clone());
    let (part2x, part2y) = part2(grid, &mut carts);
    println!("part 1: {},{}", part1x, part1y);
    println!("part 2: {},{}", part2x, part2y);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let track = r#"
/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
"#;
        let input = track
            .lines()
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        let (grid, mut carts) = parse_input(&input);
        assert_eq!(part1(grid, &mut carts), (7, 3));
    }

    #[test]
    fn test_part2() {
        let track = r#"
/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
"#;
        let input = track
            .lines()
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        let (grid, mut carts) = parse_input(&input);
        assert_eq!(part2(grid, &mut carts), (6, 4));
    }
}
