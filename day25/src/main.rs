use std::collections::HashSet;
use std::convert::TryInto;
use std::error;
use std::io::{self, Write};
use std::result;
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Box::<dyn error::Error>::from(format!($($tt)*)) }
}
type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(PartialEq, Debug)]
struct Point {
    coords: [isize; 4],
    neighbors: HashSet<usize>,
}

fn parse_points(s: &str) -> Result<Vec<Point>> {
    s.lines().map(str::parse).collect::<Result<Vec<_>>>()
}

impl Point {
    fn distance(&self, other: &Self) -> u32 {
        self.coords
            .iter()
            .zip(&other.coords)
            .map(|(a, b)| (a - b).abs() as u32)
            .sum()
    }
}

impl FromStr for Point {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        let mut nums = s.trim().split(',');
        Ok(Point {
            neighbors: HashSet::new(),
            coords: [nums.next(), nums.next(), nums.next(), nums.next()]
                .iter()
                .map(|s| s.map(str::parse))
                .collect::<Option<std::result::Result<Vec<_>, std::num::ParseIntError>>>()
                .transpose()?
                .ok_or_else(|| err!("Missing coordinate"))?
                .as_slice()
                .try_into()?,
        })
    }
}

fn set_neighbors(points: &mut Vec<Point>) {
    for idx in 0..points.len() {
        for other_idx in (idx + 1)..points.len() {
            let distance = points[idx].distance(&points[other_idx]);
            if distance < 4 {
                for &(a, b) in &[(idx, other_idx), (other_idx, idx)] {
                    if let Some(p) = points.get_mut(a) {
                        p.neighbors.insert(b);
                    }
                }
            }
        }
    }
}

type Constellation = HashSet<usize>;

fn find_constellations(points: &[Point]) -> Vec<Constellation> {
    fn traverse(start: usize, points: &[Point]) -> Constellation {
        let mut constellation = HashSet::new();
        let mut queue = vec![start];
        while let Some(point) = queue.pop() {
            for &neighbor in points[point].neighbors.iter() {
                if constellation.insert(neighbor) {
                    queue.push(neighbor);
                }
            }
        }
        constellation
    }
    let mut visited = <Constellation>::new();
    let mut constellations = Vec::new();
    for idx in 0..points.len() {
        if visited.contains(&idx) {
            continue;
        }
        let constellation = traverse(idx, points);
        visited.extend(&constellation);
        constellations.push(constellation);
    }
    constellations
}

fn part1(input: &str) -> Result<usize> {
    let mut points = parse_points(input)?;
    set_neighbors(&mut points);
    let constellations = find_constellations(&points);
    Ok(constellations.len())
}

fn main() -> Result<()> {

    let input = std::fs::read_to_string("day25/input.txt")?;
    writeln!(io::stdout(), "part1: {:?}", part1(&input)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_points() -> Result<()> {
        let input = "2,5,-4,-7
8,-3,5,7";
        let output = vec![
            Point {
                neighbors: HashSet::new(),
                coords: [2, 5, -4, -7],
            },
            Point {
                neighbors: HashSet::new(),
                coords: [8, -3, 5, 7],
            },
        ];
        assert_eq!(output, parse_points(input)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let pairs: &[(&str, usize)] = &[
            (
                "0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0",
                2,
            ),
            (
                "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0",
                4,
            ),
            (
                "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2",
                3,
            ),
            (
                "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2",
                8,
            ),
        ];

        for (input, answer) in pairs {
            assert_eq!(part1(input)?, *answer);
        }
        Ok(())
    }
}
