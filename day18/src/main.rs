use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{Error, ErrorKind::InvalidInput};
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(PartialEq, Clone, Debug, Hash, Eq)]
enum Acre {
    Open,
    Lumberyard,
    Tree,
}

#[derive(Clone)]
struct Forest {
    acres: Vec<Vec<Acre>>,
}

struct ForestIterator(Forest);

impl Forest {
    fn resource_value(&self) -> u32 {
        let trees: u32 = self
            .acres
            .iter()
            .flatten()
            .filter(|&acre| *acre == Acre::Tree)
            .count() as u32;
        let lumbaryards: u32 = self
            .acres
            .iter()
            .flatten()
            .filter(|&acre| *acre == Acre::Lumberyard)
            .count() as u32;
        trees * lumbaryards
    }

    fn get<T>(&self, (x, y): (T, T)) -> Option<&Acre>
    where
        T: TryInto<usize>,
        T::Error: std::fmt::Debug,
    {
        let (x, y) = if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
            (x, y)
        } else {
            return None;
        };
        self.acres.get(y).and_then(|rows| rows.get(x))
    }

    fn into_iter(self) -> ForestIterator {
        ForestIterator(self)
    }

    fn count<T>(&self, pos: (T, T), acre: Acre) -> u8
    where
        T: TryInto<isize>,
        T::Error: std::fmt::Debug,
    {
        let (x, y): (isize, isize) = (pos.0.try_into().unwrap(), pos.1.try_into().unwrap());
        let deltas = &[-1_isize, 0_isize, 1_isize];
        let adjustments = deltas
            .iter()
            .flat_map(|x| {
                deltas.iter().filter_map(move |y| {
                    if let (0, 0) = (x, y) {
                        None
                    } else {
                        Some((x, y))
                    }
                })
            })
            .collect::<Vec<_>>();

        adjustments
            .iter()
            .filter(|(&adj_x, &adj_y)| self.get(((x + adj_x), (y + adj_y))) == Some(&acre))
            .count()
            .try_into()
            .unwrap()
    }
}

impl Iterator for ForestIterator {
    type Item = Forest;
    fn next(&mut self) -> Option<Self::Item> {
        use Acre::*;
        let mut rows = Vec::new();
        for (y, row) in self.0.acres.iter().enumerate() {
            let mut cols = Vec::new();
            for (x, c) in row.iter().enumerate() {
                let acre = match c {
                    Open => {
                        let tree_count = self.0.count((x, y), Acre::Tree);
                        if tree_count >= 3 {
                            Tree
                        } else {
                            Open
                        }
                    }
                    Tree => {
                        let lumbaryard_count = self.0.count((x, y), Acre::Lumberyard);
                        if lumbaryard_count >= 3 {
                            Lumberyard
                        } else {
                            Tree
                        }
                    }
                    Lumberyard => {
                        let lumbaryard_count = self.0.count((x, y), Acre::Lumberyard);
                        let tree_count = self.0.count((x, y), Acre::Tree);
                        if lumbaryard_count >= 1 && tree_count >= 1 {
                            Lumberyard
                        } else {
                            Open
                        }
                    }
                };
                cols.push(acre);
            }
            rows.push(cols);
        }
        self.0 = Forest { acres: rows };
        Some(self.0.clone())
    }
}

impl FromStr for Forest {
    type Err = Box<std::error::Error>;
    fn from_str(input: &str) -> Result<Self> {
        use Acre::*;
        let mut rows = Vec::new();
        for line in input.lines() {
            let mut cols = Vec::new();
            for c in line.chars() {
                match c {
                    '.' => cols.push(Open),
                    '|' => cols.push(Tree),
                    '#' => cols.push(Lumberyard),
                    e => return Err(Box::new(Error::new(InvalidInput, format!("got {}", e)))),
                }
            }
            rows.push(cols);
        }
        Ok(Forest { acres: rows })
    }
}

impl std::fmt::Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Acre::*;
        for row in self.acres.iter() {
            for col in row.iter() {
                write!(
                    f,
                    "{}",
                    match col {
                        Lumberyard => '#',
                        Tree => '|',
                        Open => '.',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn run_simulation(input: &str, iterations: usize) -> Result<u32> {
    let forest: Forest = input.parse()?;
    let mut seen = HashMap::new();
    let mut iter = forest.into_iter();
    for idx in 1..iterations {
        let f = iter.next().unwrap();
        if seen.contains_key(&f.acres) {
            let old_idx = seen[&f.acres];
            let diff = idx - old_idx;

            if (iterations - idx) % diff == 0 {
                return Ok(f.resource_value());
            }
        }
        seen.insert(f.acres, idx);
    }
    Ok(iter.next().unwrap().resource_value())
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    println!("part1: {}", run_simulation(&input, 10)?);
    println!("part2: {}", run_simulation(&input, 1_000_000_000)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = ".#.#...|#.\n\
                     .....#|##|\n\
                     .|..|...#.\n\
                     ..|#.....#\n\
                     #.#|||#|#|\n\
                     ...#.||...\n\
                     .|....|...\n\
                     ||...#|.#|\n\
                     |.||||..|.\n\
                     ...#.|..|.";
        let forest: Forest = input.parse().unwrap();
        assert_eq!(forest.get((0, 0)), Some(&Acre::Open));
        assert_eq!(forest.get((8, 1)), Some(&Acre::Lumberyard));
        assert_eq!(forest.get((9, 1)), Some(&Acre::Tree));
        assert_eq!(forest.get((9, 9)), Some(&Acre::Open));
        assert_eq!(forest.get((9, 10)), None);
        assert_eq!(forest.get((10, 9)), None);
    }

    #[test]
    fn test_iter() {
        let forest: Forest = ".#.#...|#.\n\
                              .....#|##|\n\
                              .|..|...#.\n\
                              ..|#.....#\n\
                              #.#|||#|#|\n\
                              ...#.||...\n\
                              .|....|...\n\
                              ||...#|.#|\n\
                              |.||||..|.\n\
                              ...#.|..|."
            .parse()
            .unwrap();
        let mut iter = forest.into_iter();
        // after 1
        assert_eq!(
            iter.next().unwrap().to_string(),
            ".......##.\n\
             ......|###\n\
             .|..|...#.\n\
             ..|#||...#\n\
             ..##||.|#|\n\
             ...#||||..\n\
             ||...|||..\n\
             |||||.||.|\n\
             ||||||||||\n\
             ....||..|.\n"
        );
        // after 2
        assert_eq!(
            iter.next().unwrap().to_string(),
            ".......#..\n\
             ......|#..\n\
             .|.|||....\n\
             ..##|||..#\n\
             ..###|||#|\n\
             ...#|||||.\n\
             |||||||||.\n\
             ||||||||||\n\
             ||||||||||\n\
             .|||||||||\n"
        );
        // after 3
        assert_eq!(
            iter.next().unwrap().to_string(),
            ".......#..\n\
             ....|||#..\n\
             .|.||||...\n\
             ..###|||.#\n\
             ...##|||#|\n\
             .||##|||||\n\
             ||||||||||\n\
             ||||||||||\n\
             ||||||||||\n\
             ||||||||||\n"
        );

        // after 10
        assert_eq!(
            iter.nth(6).unwrap().to_string(),
            ".||##.....\n\
             ||###.....\n\
             ||##......\n\
             |##.....##\n\
             |##.....##\n\
             |##....##|\n\
             ||##.####|\n\
             ||#####|||\n\
             ||||#|||||\n\
             ||||||||||\n"
        );
    }

    #[test]
    fn test_resource_value() {
        let forest: Forest = ".#.#...|#.\n\
                              .....#|##|\n\
                              .|..|...#.\n\
                              ..|#.....#\n\
                              #.#|||#|#|\n\
                              ...#.||...\n\
                              .|....|...\n\
                              ||...#|.#|\n\
                              |.||||..|.\n\
                              ...#.|..|."
            .parse()
            .unwrap();
        assert_eq!(forest.into_iter().nth(9).unwrap().resource_value(), 1147);
    }
}
