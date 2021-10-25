use std::collections::VecDeque;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Species {
    Goblin,
    Elf,
}

#[derive(Debug, Clone, PartialEq)]
struct Combatant {
    species: Species,
    hit_points: u8,
    attack_power: u8,
}

impl Combatant {
    fn new(species: Species) -> Self {
        Combatant {
            species,
            hit_points: 200,
            attack_power: 3,
        }
    }
}

#[derive(Debug, PartialEq)]
enum BoardPiece {
    Space,
    Wall,
    Combatant(Combatant),
}

struct Board {
    pieces: Vec<Vec<BoardPiece>>,
    dimensions: (usize, usize),
}

#[derive(PartialEq, Eq, PartialOrd)]
struct PathNode {
    coords: (usize, usize),
    prev: Option<Rc<PathNode>>,
}

impl PathNode {
    fn iter(&self) -> Iter {
        Iter { next: Some(self) }
    }

    /// Turns the PathNode into a Vec
    /// PathNodes are collected step-by-step with the first step being the last one in the chain; a
    /// PathNode references the step that came *before* in its `.prev` attribute. This method
    /// consumes a PathNode and returns a Vec of its coordinates, reversing it so the first element
    /// of the Vec should be the piece that is moving and the last element should be the enemy
    /// piece it is approaching.
    fn to_vec(&self) -> Vec<(usize, usize)> {
        self.iter()
            .map(|p| p.coords)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }
}

impl std::fmt::Debug for PathNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let points = self.iter().map(|p| p.coords).collect::<Vec<_>>();
        write!(f, "{:?}", points)
    }
}

impl Board {
    fn game_is_over(&self) -> bool {
        ![Species::Goblin, Species::Elf].iter().all(|species| {
            self.pieces.iter().flatten().any(|piece| match piece {
                BoardPiece::Combatant(Combatant { species: s, .. }) if s == species => true,
                _ => false,
            })
        })
    }

    /// Get the coordinates of squares adjacent to `coords`, minding the edges of the board
    fn jiggle(&self, coords: (usize, usize)) -> Vec<(usize, usize)> {
        let mut adjacent = Vec::new();
        for adj in [(0, 1), (1, 0)].iter() {
            if let (Some(x), Some(y)) = (coords.0.checked_sub(adj.0), coords.1.checked_sub(adj.1)) {
                adjacent.push((x, y));
            }
        }
        for adj in [(1, 0), (0, 1)].iter() {
            if let (Some(_), Some(_)) = (
                self.dimensions.0.checked_sub(coords.0 + adj.0),
                self.dimensions.1.checked_sub(coords.1 + adj.1),
            ) {
                adjacent.push((coords.0 + adj.0, coords.1 + adj.1));
            }
        }
        adjacent
    }

    /// Returns coordinates of best adjacent target (if any), preferring those with lowest hit
    /// points left, and then in reading order
    fn find_adjacent_enemy(
        &self,
        enemy_species: &Species,
        from: (usize, usize),
    ) -> Option<(usize, usize)> {
        let adjacent_enemies: Vec<_> = self
            .jiggle(from)
            .into_iter()
            .filter_map(|coord| match self.get(coord) {
                Some(BoardPiece::Combatant(c)) if c.species == *enemy_species => {
                    Some((c.hit_points, coord))
                }
                _ => None,
            })
            .collect();
        if let Some(min_hitpoints) = adjacent_enemies.iter().map(|(hp, _)| hp).min() {
            let mut low_points = adjacent_enemies
                .iter()
                .filter_map(|(hp, coords)| {
                    if hp == min_hitpoints {
                        Some(vec![*coords, *coords])
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            sort_reading_order(&mut low_points);
            return low_points.into_iter().next().unwrap().into_iter().next();
        }
        None
    }

    fn get(&self, coords: (usize, usize)) -> Option<&BoardPiece> {
        self.pieces.get(coords.1).and_then(|row| row.get(coords.0))
    }

    fn get_mut(&mut self, coords: (usize, usize)) -> Option<&mut BoardPiece> {
        self.pieces
            .get_mut(coords.1)
            .and_then(|row| row.get_mut(coords.0))
    }

    /// Attacks the piece at `enemy_coords` with the hitpower from the piece at `coords`. Returns
    /// `true` if the enemy piece was killed.
    fn attack(&mut self, coords: (usize, usize), enemy_coords: (usize, usize)) {
        let attack_power = match self.get(coords).unwrap() {
            BoardPiece::Combatant(Combatant {
                attack_power: p, ..
            }) => *p,
            _ => {
                panic!(
                    "Tried to attack from a non-combatant board piece at {:?}",
                    coords
                );
            }
        };
        let other_piece = self.get_mut(enemy_coords).unwrap();
        if let BoardPiece::Combatant(enemy) = other_piece {
            // Use +1 to adjust for case of `0`
            if let Some(p) = enemy.hit_points.checked_sub(attack_power + 1) {
                enemy.hit_points = p + 1;
            } else {
                *other_piece = BoardPiece::Space;
            }
        }
    }

    fn find_enemy_path(
        &self,
        enemy_species: &Species,
        from: (usize, usize),
    ) -> Option<(usize, usize)> {
        let mut paths = VecDeque::from(vec![Rc::new(PathNode {
            coords: from,
            prev: None,
        })]);
        let mut should_end = false;
        loop {
            if paths.is_empty() {
                return None;
            }
            for _ in 0..paths.len() {
                if let Some(path) = paths.pop_front() {
                    for pos in self.jiggle(path.coords) {
                        if paths
                            .iter()
                            .any(|path| path.iter().any(|p| p.coords == pos))
                            || path.iter().any(|p| p.coords == pos)
                        {
                            continue;
                        };
                        match self.get(pos) {
                            Some(BoardPiece::Combatant(Combatant { species: s, .. }))
                                if s == enemy_species =>
                            {
                                should_end = true;
                                paths.push_back(Rc::new(PathNode {
                                    coords: pos,
                                    prev: Some(Rc::clone(&path)),
                                }));
                            }
                            Some(BoardPiece::Space) => {
                                paths.push_back(Rc::new(PathNode {
                                    coords: pos,
                                    prev: Some(Rc::clone(&path)),
                                }));
                            }
                            _ => (),
                        }
                    }
                }
            }
            if should_end {
                break;
            }
        }

        // Only keep paths whose first element (the last added) is an enemy
        let mut paths: Vec<_> = paths
            .into_iter()
            .filter(|p| match self.get(p.coords) {
                Some(BoardPiece::Combatant(Combatant { species: s, .. })) if s == enemy_species => {
                    true
                }
                _ => false,
            })
            .collect();

        if let Some(min_length) = paths.iter().map(|p| p.iter().count()).min() {
            // Only consider paths with the minimum number of steps
            paths.retain(|p| p.iter().count() == min_length);

            let mut paths: Vec<_> = paths.iter().map(|p| p.to_vec()).collect();
            sort_reading_order(&mut paths);

            // Return the 2nd element of the 1st vec, the 1st element being the coordinates of the
            // piece that is moving
            return paths.into_iter().next().unwrap().into_iter().nth(1);
        }
        None
    }

    /// If piece hasn't had its turn yet it should still be in `pieces_in_order` (not yet popped
    /// off), and it may get killed before its turn, so need to update value in pieces_in_order by
    /// cloning from value in board, which should be updated
    fn update_pieces(
        &self,
        enemy_coords: (usize, usize),
        pieces_in_order: &mut VecDeque<((usize, usize), Combatant)>,
    ) {
        if let Some(BoardPiece::Combatant(c)) = self.get(enemy_coords) {
            if let Some(mut p) = pieces_in_order
                .iter_mut()
                .find(|(coords, _)| coords == &enemy_coords)
            {
                p.1 = c.clone();
            }
        } else {
            // Those coords on board not a combatant, likely died and now a space
            pieces_in_order.retain(|&(coords, _)| coords != enemy_coords);
        }
    }
}

struct Iter<'a> {
    next: Option<&'a PathNode>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a PathNode;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|n| {
            self.next = n.prev.as_deref();
            n
        })
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter()
            .count()
            .cmp(&other.iter().count())
            .then_with(|| {
                self.coords.1.cmp(&other.coords.1).then_with(|| {
                    self.coords
                        .0
                        .cmp(&other.coords.0)
                        .then_with(|| self.prev.cmp(&other.prev))
                })
            })
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lines = self
            .pieces
            .iter()
            .map(|y| {
                y.iter()
                    .map(|p| {
                        use BoardPiece::*;
                        match p {
                            Space => '.',
                            Wall => '#',
                            Combatant(crate::Combatant {
                                species: Species::Goblin,
                                ..
                            }) => 'G',
                            Combatant(crate::Combatant {
                                species: Species::Elf,
                                ..
                            }) => 'E',
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>();
        write!(f, "{}", lines.join("\n"))
    }
}

impl FromStr for Board {
    type Err = Box<dyn std::error::Error>;
    fn from_str(input: &str) -> Result<Board, Self::Err> {
        let mut rows = Vec::new();
        for line in input.lines() {
            let mut columns = Vec::new();
            for c in line.chars() {
                use BoardPiece::*;
                use Species::*;
                let piece = match c {
                    '#' => Wall,
                    '.' => Space,
                    'G' => Combatant(crate::Combatant::new(Goblin)),
                    'E' => Combatant(crate::Combatant::new(Elf)),
                    _ => {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("invalid character found in board: {}", c),
                        )))
                    }
                };
                columns.push(piece);
            }
            rows.push(columns);
        }
        Ok(Board {
            pieces: rows,
            dimensions: (
                input.lines().next().unwrap().chars().count() - 1,
                input.lines().count() - 1,
            ),
        })
    }
}

fn sort_reading_order<T, U>(paths: &mut [T])
where
    T: AsRef<[(U, U)]>,
    U: Ord,
{
    paths.sort_unstable_by(|first, second| {
        let a = first.as_ref().iter().rev().nth(1).unwrap();
        let b = second.as_ref().iter().rev().nth(1).unwrap();
        a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0))
    });
}

fn part1(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut board: Board = input.parse()?;
    for round in 0.. {
        // println!("Round: {}\n{}", round, board.to_string());
        let mut pieces_in_order = board
            .pieces
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, piece)| match piece {
                        BoardPiece::Wall => None,
                        BoardPiece::Space => None,
                        BoardPiece::Combatant(c) => Some(((x, y), c.clone())),
                    })
            })
            .collect::<VecDeque<_>>();

        while let Some((coords, piece)) = pieces_in_order.pop_front() {
            // Still a piece that needs to move, so this round isn't over, but need to verify the
            // game isn't finished
            if board.game_is_over() {
                let hit_points = board
                    .pieces
                    .iter()
                    .flatten()
                    .filter_map(|piece| {
                        if let BoardPiece::Combatant(c) = piece {
                            Some(c)
                        } else {
                            None
                        }
                    })
                    .map(|p| u32::from(p.hit_points))
                    .sum::<u32>();
                return Ok(hit_points * round);
            }

            let enemy_species = match piece.species {
                Species::Elf => Species::Goblin,
                Species::Goblin => Species::Elf,
            };
            if let Some(enemy_coords) = board.find_adjacent_enemy(&enemy_species, coords) {
                board.attack(coords, enemy_coords);
                board.update_pieces(enemy_coords, &mut pieces_in_order);

                if let Some(BoardPiece::Combatant(c)) = board.get(enemy_coords) {
                    // If piece hasn't gone yet it should still be in pieces_in_order (not yet
                    // popped off), and it may get killed before its turn, so need to update value
                    // in pieces_in_order by cloning from value in board, which should be updated
                    if let Some(mut p) = pieces_in_order
                        .iter_mut()
                        .find(|(coords, _)| coords == &enemy_coords)
                    {
                        p.1 = c.clone();
                    }
                } else {
                    // Those coords on board not a combatant, likely died and now a space
                    pieces_in_order.retain(|&(coords, _)| coords != enemy_coords);
                }
            } else if let Some(next_step) = board.find_enemy_path(&enemy_species, coords) {
                *board.get_mut(next_step).unwrap() = BoardPiece::Combatant(piece);
                *board.get_mut(coords).unwrap() = BoardPiece::Space;
                if let Some(enemy_coords) = board.find_adjacent_enemy(&enemy_species, next_step) {
                    board.attack(next_step, enemy_coords);
                    board.update_pieces(enemy_coords, &mut pieces_in_order);
                }
            }
        }
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "This should be unreachable code",
    )))
}

fn part2(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    'outer: for power in 3..255 {
        let mut board: Board = input.parse()?;
        let mut elf_count = 0;
        board
            .pieces
            .iter_mut()
            .flatten()
            .for_each(|piece| match piece {
                BoardPiece::Combatant(c) if c.species == Species::Elf => {
                    c.attack_power = power;
                    elf_count += 1;
                }
                _ => (),
            });
        for round in 0.. {
            // println!("Round: {}\n{}", round, board.to_string());
            let mut pieces_in_order = board
                .pieces
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .filter_map(move |(x, piece)| match piece {
                            BoardPiece::Wall => None,
                            BoardPiece::Space => None,
                            BoardPiece::Combatant(c) => Some(((x, y), c.clone())),
                        })
                })
                .collect::<VecDeque<_>>();
            if pieces_in_order
                .iter()
                .filter(|(_, combatant)| combatant.species == Species::Elf)
                .count()
                != elf_count
            {
                continue 'outer;
            };

            while let Some((coords, piece)) = pieces_in_order.pop_front() {
                // Still a piece that needs to move, so this round isn't over, but need to verify the
                // game isn't finished
                if board.game_is_over() {
                    let hit_points = board
                        .pieces
                        .iter()
                        .flatten()
                        .filter_map(|piece| {
                            if let BoardPiece::Combatant(c) = piece {
                                Some(c)
                            } else {
                                None
                            }
                        })
                        .map(|p| u32::from(p.hit_points))
                        .sum::<u32>();
                    return Ok(hit_points * round);
                }

                let enemy_species = match piece.species {
                    Species::Elf => Species::Goblin,
                    Species::Goblin => Species::Elf,
                };
                if let Some(enemy_coords) = board.find_adjacent_enemy(&enemy_species, coords) {
                    board.attack(coords, enemy_coords);
                    board.update_pieces(enemy_coords, &mut pieces_in_order);

                    if let Some(BoardPiece::Combatant(c)) = board.get(enemy_coords) {
                        // If piece hasn't gone yet it should still be in pieces_in_order (not yet
                        // popped off), and it may get killed before its turn, so need to update value
                        // in pieces_in_order by cloning from value in board, which should be updated
                        if let Some(mut p) = pieces_in_order
                            .iter_mut()
                            .find(|(coords, _)| coords == &enemy_coords)
                        {
                            p.1 = c.clone();
                        }
                    } else {
                        // Those coords on board not a combatant, likely died and now a space
                        pieces_in_order.retain(|&(coords, _)| coords != enemy_coords);
                    }
                } else if let Some(next_step) = board.find_enemy_path(&enemy_species, coords) {
                    *board.get_mut(next_step).unwrap() = BoardPiece::Combatant(piece);
                    *board.get_mut(coords).unwrap() = BoardPiece::Space;
                    if let Some(enemy_coords) = board.find_adjacent_enemy(&enemy_species, next_step)
                    {
                        board.attack(next_step, enemy_coords);
                        board.update_pieces(enemy_coords, &mut pieces_in_order);
                    }
                }
            }
        }
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "This should be unreachable code",
    )))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let input = std::fs::read_to_string("day15/input.txt")?;
    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_pairs = vec![
            (
                "
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
",
                27730,
            ),
            (
                "
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
",
                36334,
            ),
            (
                "
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
",
                39514,
            ),
            (
                "
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
",
                27755,
            ),
            (
                "
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
",
                28944,
            ),
            (
                "
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
",
                18740,
            ),
        ];

        for (board, score) in test_pairs {
            let board = board
                .lines()
                .filter(|line| !line.trim().is_empty())
                .collect::<Vec<&str>>()
                .join("\n");
            assert_eq!(part1(&board).unwrap(), score);
        }
    }

    #[test]
    fn test_parse_input() {
        let input = "
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
"
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n");

        let output = "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

        let board: Board = input.parse().unwrap();
        assert_eq!(board.to_string(), output);
    }

    #[test]
    fn test_jiggle() {
        let input = "
#########
#G......#
#.E.#...#
"
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n");

        let board: Board = input.parse().unwrap();
        assert_eq!(board.jiggle((1, 1)), vec![(1, 0), (0, 1), (2, 1), (1, 2)]);
        assert_eq!(board.jiggle((8, 1)), vec![(8, 0), (7, 1), (8, 2)]);
        assert_eq!(board.jiggle((4, 2)), vec![(4, 1), (3, 2), (5, 2)]);
        assert_eq!(board.jiggle((0, 0)), vec![(1, 0), (0, 1)]);
    }

    #[test]
    fn test_get() {
        let input = "
#########
#.G.....#
#.E.#...#
"
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n");

        let mut board: Board = input.parse().unwrap();
        assert_eq!(*board.get((8, 1)).unwrap(), BoardPiece::Wall);
        assert_eq!(*board.get((1, 2)).unwrap(), BoardPiece::Space);
        assert_eq!(
            *board.get((2, 1)).unwrap(),
            BoardPiece::Combatant(Combatant {
                species: Species::Goblin,
                hit_points: 200,
                attack_power: 3,
            })
        );
        assert_eq!(board.get_mut((1, 1)), Some(&mut BoardPiece::Space));
    }
}
