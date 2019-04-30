// Solution:
// part1: EKALLKLB
// part2: 10227

use std::collections::HashSet;

#[derive(Default)]
struct Grid {
    xmin: Option<i32>,
    xmax: Option<i32>,
    ymin: Option<i32>,
    ymax: Option<i32>,
    pointset: HashSet<(i32, i32)>,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut v = String::with_capacity(
            (self.ymax.unwrap() - self.ymin.unwrap()) as usize
                * (self.xmax.unwrap() - self.xmin.unwrap()) as usize,
        );
        for y in self.ymin.unwrap()..=self.ymax.unwrap() {
            for x in self.xmin.unwrap()..=self.xmax.unwrap() {
                v.push(if self.pointset.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                })
            }
            v.push('\n');
        }
        write!(f, "{}", v.trim())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    pos: (i32, i32),
    vel: (i32, i32),
}

fn parse_line(line: &str) -> Point {
    let mut nums = line
        .split(&['<', '>', ','][..])
        .filter_map(|s| s.trim().parse::<i32>().ok());
    let x = nums.next().expect("no x");
    let y = nums.next().expect("no y");
    let xv = nums.next().expect("no xv");
    let yv = nums.next().expect("no yv");
    Point {
        pos: (x, y),
        vel: (xv, yv),
    }
}

fn part1(points: &mut Vec<Point>) -> Option<(String, usize)> {
    let mut most_neighbors: Option<(usize, usize)> = None;
    for step in 0.. {
        let mut grid = Grid {
            ..Default::default()
        };

        for point in points.iter_mut() {
            // Set grid with points *prior* to update so that if the count of neighbors is
            // decreasing we can refer back to the grid prior to the change
            grid.pointset.insert((point.pos.0, point.pos.1));

            point.pos.0 += point.vel.0;
            grid.xmin = grid
                .xmin
                .min(Some(point.pos.0))
                .or_else(|| Some(point.pos.0));
            grid.xmax = grid.xmax.max(Some(point.pos.0));

            point.pos.1 += point.vel.1;
            grid.ymin = grid
                .ymin
                .min(Some(point.pos.1))
                .or_else(|| Some(point.pos.1));
            grid.ymax = grid.ymax.max(Some(point.pos.1));
        }

        let neighbor_count = points.iter().fold(0, |acc, point| {
            acc + points
                .iter()
                .filter(|p| {
                    (p.pos.0 - point.pos.0).abs() <= 1 && (p.pos.1 - point.pos.1).abs() <= 1
                })
                .count()
        });
        if let Some((_step, old_count)) = most_neighbors {
            // If neighbor_count is increasing, keep going
            if neighbor_count >= old_count {
                most_neighbors = Some((step, neighbor_count));

            // Each star is double counted, and will on average have 2 neighbor stars, so only
            // print if the neighbor_count is somewhat close to 4 times the number of stars
            } else if neighbor_count > (points.len() * 2) {
                return Some((grid.to_string(), step));
            }
        } else {
            most_neighbors = Some((step, neighbor_count));
        }
    }
    None
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut points: Vec<_> = input.lines().map(|line| parse_line(line)).collect();

    if let Some((stars, steps)) = part1(&mut points) {
        println!("part 1:\n{}", stars);
        println!("part 2: {}", steps);
    } else {
        println!("No solution found");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_line() {
        let line = "position=< 9,  -1> velocity=< 0,  2>";
        let parsed = parse_line(line);
        assert_eq!(
            parsed,
            Point {
                pos: (9, -1),
                vel: (0, 2)
            }
        )
    }

    #[test]
    #[test]
    fn test_part1() {
        let mut points = "
            position=< 9,  1> velocity=< 0,  2>
            position=< 7,  0> velocity=<-1,  0>
            position=< 3, -2> velocity=<-1,  1>
            position=< 6, 10> velocity=<-2, -1>
            position=< 2, -4> velocity=< 2,  2>
            position=<-6, 10> velocity=< 2, -2>
            position=< 1,  8> velocity=< 1, -1>
            position=< 1,  7> velocity=< 1,  0>
            position=<-3, 11> velocity=< 1, -2>
            position=< 7,  6> velocity=<-1, -1>
            position=<-2,  3> velocity=< 1,  0>
            position=<-4,  3> velocity=< 2,  0>
            position=<10, -3> velocity=<-1,  1>
            position=< 5, 11> velocity=< 1, -2>
            position=< 4,  7> velocity=< 0, -1>
            position=< 8, -2> velocity=< 0,  1>
            position=<15,  0> velocity=<-2,  0>
            position=< 1,  6> velocity=< 1,  0>
            position=< 8,  9> velocity=< 0, -1>
            position=< 3,  3> velocity=<-1,  1>
            position=< 0,  5> velocity=< 0, -1>
            position=<-2,  2> velocity=< 2,  0>
            position=< 5, -2> velocity=< 1,  2>
            position=< 1,  4> velocity=< 2,  1>
            position=<-2,  7> velocity=< 2, -2>
            position=< 3,  6> velocity=<-1, -1>
            position=< 5,  0> velocity=< 1,  0>
            position=<-6,  0> velocity=< 2,  0>
            position=< 5,  9> velocity=< 1, -2>
            position=<14,  7> velocity=<-2,  0>
            position=<-3,  6> velocity=< 2, -1>
            "
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();
        println!("{}", part1(&mut points).expect("No solution found").0);
    }
}
