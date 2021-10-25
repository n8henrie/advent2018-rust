use std::collections::{BinaryHeap, HashMap};
use std::convert::TryInto;
use std::fmt;
use std::io::{self, Error, ErrorKind, Write};
use std::u32;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse_input(input: &str) -> Result<(usize, (usize, usize))> {
    let mut lines = input.lines();
    let depth = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()?;
    let mut target = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .split(',');
    let target = (
        target.next().unwrap().parse()?,
        target.next().unwrap().parse()?,
    );
    Ok((depth, target))
}

#[derive(Debug, PartialEq)]
enum Region {
    Mouth,
    Target,
    Rocky,
    Narrow,
    Wet,
}

struct Cave {
    grid: Vec<Vec<Region>>,
    depth: usize,
    target: (usize, usize),
    cache: HashMap<(usize, usize), u32>,
}

impl Cave {
    fn new(
        depth: usize,
        target: (usize, usize),
        expand: Option<(usize, usize)>,
    ) -> Self {
        let (width, height) = if let Some(e) = expand {
            (e.0 + target.0, e.1 + target.1)
        } else {
            target
        };
        let mut cave = Cave {
            grid: Vec::with_capacity(height + 1),
            depth,
            target,
            cache: HashMap::new(),
        };
        for y in 0..=height {
            let mut row = Vec::with_capacity(width + 1);
            for x in 0..=width {
                let geologic_index = cave.geologic_index((x, y));
                cave.cache.insert((x, y), geologic_index);
                let erosion_level = cave.erosion_level(geologic_index);
                let region_type = Cave::region_type(erosion_level);
                row.push(region_type);
            }
            cave.grid.push(row);
        }
        cave
    }

    fn risk_level(&self) -> u32 {
        self.grid
            .iter()
            .flat_map(|row| {
                row.iter().map(|c| match c {
                    Region::Rocky => 0,
                    Region::Wet => 1,
                    Region::Narrow => 2,
                    _ => unreachable!(),
                })
            })
            .sum()
    }

    // (a * b) % c == ((a % c) * (b % c)) % c
    fn geologic_index(&self, coords: (usize, usize)) -> u32 {
        if let Some(val) = self.cache.get(&coords) {
            *val
        } else {
            match coords {
                _ if coords == self.target => 0,
                (0, 0) => 0,
                (x, 0) => x as u32 * 16807,
                (0, y) => y as u32 * 48271,
                (x, y) => {
                    self.erosion_level(self.geologic_index((x - 1, y)))
                        * self.erosion_level(self.geologic_index((x, y - 1)))
                }
            }
        }
    }

    // (a + b) % c == (a % c + b) % c
    fn erosion_level(&self, geologic_index: u32) -> u32 {
        (geologic_index + self.depth as u32) % 20183
    }

    fn region_type(erosion_level: u32) -> Region {
        match erosion_level % 3 {
            0 => Region::Rocky,
            1 => Region::Wet,
            2 => Region::Narrow,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Region::*;
        write!(
            f,
            "{}",
            match self {
                Mouth => 'M',
                Target => 'T',
                Rocky => '.',
                Wet => '=',
                Narrow => '|',
            }
        )
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let numrows = self.grid.len();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, region) in row.iter().enumerate() {
                let output = match (x, y) {
                    (0, 0) => &Region::Mouth,
                    t if t == self.target => &Region::Target,
                    _ => region,
                };
                write!(f, "{}", output)?;
            }
            if (y + 1) < numrows {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn part1(input: &str) -> Result<u32> {
    let (depth, target) = parse_input(input)?;
    let cave = Cave::new(depth, target, None);
    Ok(cave.risk_level())
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

#[derive(Clone, Eq, PartialEq)]
struct Climber {
    pos: (usize, usize),
    tool: Tool,
    time: u32,
}

impl Ord for Climber {
    fn cmp(&self, other: &Climber) -> std::cmp::Ordering {
        other
            .time
            .cmp(&self.time)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Climber {
    fn partial_cmp(&self, other: &Climber) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Climber {
    fn new() -> Self {
        Climber {
            pos: (0, 0),
            tool: Tool::Torch,
            time: 0,
        }
    }

    fn climb(&mut self, cave: &Cave) -> Result<u32> {
        type Gridmap =
            HashMap<(usize, usize, Tool), ((usize, usize, Tool), u32)>;

        #[allow(dead_code)]
        fn get_parents(
            pos: (usize, usize, Tool),
            parents: Gridmap,
        ) -> Result<Vec<(usize, usize, Tool, u32)>> {
            let mut pos = pos;
            let mut path = <Vec<(usize, usize, Tool, u32)>>::new();
            while let Some(&((x, y, tool), score)) = parents.get(&(pos)) {
                path.push((x, y, tool, score));
                if (x, y) == (0, 0) {
                    return Ok(path.into_iter().rev().collect::<Vec<_>>());
                }
                pos = (x, y, tool);
            }
            Err(Box::from(Error::new(
                ErrorKind::Other,
                "No path from origin to goal",
            )))
        }
        use Tool::*;

        let mut dist: HashMap<((usize, usize), Tool), u32> = HashMap::new();
        let mut heap = BinaryHeap::new();
        dist.insert(((0, 0), Torch), 0);
        heap.push(self.clone());

        let mut parents: Gridmap = HashMap::new();
        let edges = [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .iter()
            .flat_map(|&pos| {
                [Torch, ClimbingGear, Neither]
                    .iter()
                    .map(move |&gear| (pos, gear))
            })
            .collect::<Vec<_>>();
        while let Some(Climber { time, pos, tool }) = heap.pop() {
            if pos == cave.target && tool == Torch {
                return Ok(time);
            }
            if time > *dist.entry((pos, tool)).or_insert(u32::MAX) {
                continue;
            }
            'outer: for (delta, newtool) in edges.iter() {
                let ipos = (
                    TryInto::<isize>::try_into(pos.0).unwrap() + delta.0,
                    TryInto::<isize>::try_into(pos.1).unwrap() + delta.1,
                );
                let newpos = if let (Ok(x), Ok(y)) = (
                    TryInto::<usize>::try_into(ipos.0),
                    TryInto::<usize>::try_into(ipos.1),
                ) {
                    (x, y)
                } else {
                    continue;
                };
                let current_region = &cave.grid[pos.1][pos.0];
                let next_region = if let Some(r) =
                    cave.grid.get(newpos.1).and_then(|p| p.get(newpos.0))
                {
                    r
                } else {
                    continue;
                };

                // Continue to next edge if the next tool combination would be invalid with either
                // the next step or with the current one
                for r in &[current_region, next_region] {
                    match (newtool, r) {
                        (Neither, Region::Rocky)
                        | (Torch, Region::Wet)
                        | (ClimbingGear, Region::Narrow) => continue 'outer,
                        _ => (),
                    }
                }

                let cost = if &tool == newtool { 1 } else { 8 };

                let next = Climber {
                    time: time + cost,
                    pos: newpos,
                    tool: *newtool,
                };

                let best_time =
                    dist.entry((next.pos, next.tool)).or_insert(u32::MAX);
                if &next.time < best_time {
                    *best_time = next.time;
                    heap.push(next);
                    parents.insert(
                        (newpos.0, newpos.1, *newtool),
                        ((pos.0, pos.1, tool), time),
                    );
                }
            }
        }
        Err(Box::from(Error::new(ErrorKind::Other, "No route found")))
    }
}

fn part2(input: &str) -> Result<u32> {
    let (depth, target) = parse_input(input)?;
    let cave = Cave::new(depth, target, Some((50, 50)));
    let mut climber = Climber::new();
    climber.climb(&cave)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day22/input.txt")?;
    writeln!(io::stdout(), "part1: {}", part1(&input)?)?;
    writeln!(io::stdout(), "part2: {}", part2(&input)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("input.txt").unwrap();
        assert_eq!(parse_input(&input).unwrap(), (11394, (7, 701)))
    }

    #[test]
    fn test_build_cave() {}

    #[test]
    fn test_geologic_index() {
        let (depth, target) = (510, (10, 10));
        let cave = Cave::new(depth, target, None);
        assert_eq!(cave.geologic_index((0, 0)), 0);
        assert_eq!(cave.geologic_index((1, 0)), 16807);
        assert_eq!(cave.geologic_index((0, 1)), 48271);
        assert_eq!(cave.geologic_index((1, 1)), 145_722_555);
        assert_eq!(cave.geologic_index((10, 10)), 0);
    }

    #[test]
    fn test_erosion_leve() {
        let (depth, target) = (510, (10, 10));
        let cave = Cave::new(depth, target, None);
        let test = |idx| cave.erosion_level(cave.geologic_index(idx));
        assert_eq!(test((1, 0)), 17317);
        assert_eq!(test((0, 1)), 8415);
        assert_eq!(test((1, 1)), 1805);
        assert_eq!(test((10, 10)), 510);
    }

    #[test]
    fn test_region_type() {
        let (depth, target) = (510, (10, 10));
        let cave = Cave::new(depth, target, None);
        let test = |idx| {
            Cave::region_type(cave.erosion_level(cave.geologic_index(idx)))
        };
        assert_eq!(test((1, 0)), Region::Wet);
        assert_eq!(test((0, 1)), Region::Rocky);
        assert_eq!(test((1, 1)), Region::Narrow);
        assert_eq!(test((10, 10)), Region::Rocky);
    }

    #[test]
    fn test_display() {
        let (depth, target) = (510, (10, 10));
        let cave = Cave::new(depth, target, Some((5, 5)));
        let solution = "M=.|=.|.|=.|=|=.
.|=|=|||..|.=...
.==|....||=..|==
=.|....|.==.|==.
=|..==...=.|==..
=||.=.=||=|=..|=
|.=.===|||..=..|
|..==||=.|==|===
.=..===..=|.|||.
.======|||=|=.|=
.===|=|===T===||
=|||...|==..|=.|
=.=|=.=..=.||==|
||=|=...|==.=|==
|=.=||===.|||===
||.|==.|.|.||=||";
        let proposed = cave.to_string();
        assert_eq!(proposed, solution);
    }

    #[test]
    fn test_risk_level() {
        let (depth, target) = (510, (10, 10));
        let cave = Cave::new(depth, target, None);
        assert_eq!(cave.risk_level(), 114)
    }

    #[test]
    fn test_part2() {
        let (depth, target) = (510, (10, 10));
        let cave = Cave::new(depth, target, Some((10, 10)));
        let mut climber = Climber::new();
        assert_eq!(climber.climb(&cave).unwrap(), 45);
    }
}
