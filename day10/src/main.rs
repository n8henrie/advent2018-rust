// --- Day 10: The Stars Align ---
//
// It's no use; your navigation system simply isn't capable of providing walking directions in the arctic circle, and certainly not in 1018.
//
// The Elves suggest an alternative. In times like these, North Pole rescue operations will arrange points of light in the sky to guide missing Elves back to base. Unfortunately, the message is easy to miss: the points move slowly enough that it takes hours to align them, but have so much momentum that they only stay aligned for a second. If you blink at the wrong time, it might be hours before another message appears.
//
// You can see these points of light floating in the distance, and record their position in the sky and their velocity, the relative change in position per second (your puzzle input). The coordinates are all given from your perspective; given enough time, those positions and velocities will move the points into a cohesive message!
//
// Rather than wait, you decide to fast-forward the process and calculate what the points will eventually spell.
//
// For example, suppose you note the following points:
//
// position=< 9,  1> velocity=< 0,  2>
// position=< 7,  0> velocity=<-1,  0>
// position=< 3, -2> velocity=<-1,  1>
// position=< 6, 10> velocity=<-2, -1>
// position=< 2, -4> velocity=< 2,  2>
// position=<-6, 10> velocity=< 2, -2>
// position=< 1,  8> velocity=< 1, -1>
// position=< 1,  7> velocity=< 1,  0>
// position=<-3, 11> velocity=< 1, -2>
// position=< 7,  6> velocity=<-1, -1>
// position=<-2,  3> velocity=< 1,  0>
// position=<-4,  3> velocity=< 2,  0>
// position=<10, -3> velocity=<-1,  1>
// position=< 5, 11> velocity=< 1, -2>
// position=< 4,  7> velocity=< 0, -1>
// position=< 8, -2> velocity=< 0,  1>
// position=<15,  0> velocity=<-2,  0>
// position=< 1,  6> velocity=< 1,  0>
// position=< 8,  9> velocity=< 0, -1>
// position=< 3,  3> velocity=<-1,  1>
// position=< 0,  5> velocity=< 0, -1>
// position=<-2,  2> velocity=< 2,  0>
// position=< 5, -2> velocity=< 1,  2>
// position=< 1,  4> velocity=< 2,  1>
// position=<-2,  7> velocity=< 2, -2>
// position=< 3,  6> velocity=<-1, -1>
// position=< 5,  0> velocity=< 1,  0>
// position=<-6,  0> velocity=< 2,  0>
// position=< 5,  9> velocity=< 1, -2>
// position=<14,  7> velocity=<-2,  0>
// position=<-3,  6> velocity=< 2, -1>
//
// Each line represents one point. Positions are given as <X, Y> pairs: X represents how far left (negative) or right (positive) the point appears, while Y represents how far up (negative) or down (positive) the point appears.
//
// At 0 seconds, each point has the position given. Each second, each point's velocity is added to its position. So, a point with velocity <1, -2> is moving to the right, but is moving upward twice as quickly. If this point's initial position were <3, 9>, after 3 seconds, its position would become <6, 3>.
//
// Over time, the points listed above would move like this:
//
// Initially:
// ........#.............
// ................#.....
// .........#.#..#.......
// ......................
// #..........#.#.......#
// ...............#......
// ....#.................
// ..#.#....#............
// .......#..............
// ......#...............
// ...#...#.#...#........
// ....#..#..#.........#.
// .......#..............
// ...........#..#.......
// #...........#.........
// ...#.......#..........
//
// After 1 second:
// ......................
// ......................
// ..........#....#......
// ........#.....#.......
// ..#.........#......#..
// ......................
// ......#...............
// ....##.........#......
// ......#.#.............
// .....##.##..#.........
// ........#.#...........
// ........#...#.....#...
// ..#...........#.......
// ....#.....#.#.........
// ......................
// ......................
//
// After 2 seconds:
// ......................
// ......................
// ......................
// ..............#.......
// ....#..#...####..#....
// ......................
// ........#....#........
// ......#.#.............
// .......#...#..........
// .......#..#..#.#......
// ....#....#.#..........
// .....#...#...##.#.....
// ........#.............
// ......................
// ......................
// ......................
//
// After 3 seconds:
// ......................
// ......................
// ......................
// ......................
// ......#...#..###......
// ......#...#...#.......
// ......#...#...#.......
// ......#####...#.......
// ......#...#...#.......
// ......#...#...#.......
// ......#...#...#.......
// ......#...#..###......
// ......................
// ......................
// ......................
// ......................
//
// After 4 seconds:
// ......................
// ......................
// ......................
// ............#.........
// ........##...#.#......
// ......#.....#..#......
// .....#..##.##.#.......
// .......##.#....#......
// ...........#....#.....
// ..............#.......
// ....#......#...#......
// .....#.....##.........
// ...............#......
// ...............#......
// ......................
// ......................
//
// After 3 seconds, the message appeared briefly: HI. Of course, your message will be much longer and will take many more seconds to appear.
//
// What message will eventually appear in the sky?

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
        let mut output = String::with_capacity(
            (self.xmax.unwrap() - self.xmin.unwrap()) as usize
                * (self.ymax.unwrap() - self.ymin.unwrap()) as usize,
        );
        for y in self.ymin.unwrap()..=self.ymax.unwrap() {
            dbg!(format!("writing y {}", y));
            for x in self.xmin.unwrap()..=self.xmax.unwrap() {
                output.push(if self.pointset.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                })
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    pos: (i32, i32),
    vel: (i32, i32),
}

// position=< 3, -2> velocity=<-1,  1>
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

fn make_grid(points: &[Point]) -> String {
    let xmin = points.iter().map(|p| p.pos.0).min().unwrap();
    let ymin = points.iter().map(|p| p.pos.1).min().unwrap();
    let xmax = points.iter().map(|p| p.pos.0).max().unwrap();
    let ymax = points.iter().map(|p| p.pos.1).max().unwrap();
    let adjusted_xmax = xmax - xmin;
    let adjusted_ymax = ymax - ymin;

    dbg!((adjusted_ymax, adjusted_xmax));
    let mut grid = vec![vec!['.'; adjusted_xmax as usize + 1]; adjusted_ymax as usize + 1];
    dbg!("finished initializing grid");
    for (idx, point) in points.iter().enumerate() {
        dbg!(format!("working on point {}", idx));
        let (x, y): (usize, usize) = ((point.pos.1 - ymin) as usize, (point.pos.0 - xmin) as usize);
        grid[x][y] = '#';
    }
    grid.iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn part1(points: &mut Vec<Point>) -> String {
    let mut most_neighbors: Option<(usize, usize)> = None;
    let mut stars: Vec<String> = Vec::new();
    for step in 0..100 {
        let mut grid = Grid {
            ..Default::default()
        };

        dbg!(format!("Step {}", step));
        for point in points.iter_mut() {
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

            grid.pointset.insert((point.pos.0, point.pos.1));
        }

        dbg!("counting neighbors");
        let neighbor_count = points.iter().fold(0, |acc, point| {
            acc + points
                .iter()
                .filter(|p| {
                    (p.pos.0 - point.pos.0).abs() <= 1 && (p.pos.1 - point.pos.1).abs() <= 1
                })
                .count()
        });
        if let Some((_step, count)) = most_neighbors {
            if neighbor_count > count {
                most_neighbors = Some((step, neighbor_count));
            }
        } else {
            most_neighbors = Some((step, neighbor_count));
        }
        dbg!("making grid");
        stars.push(grid.to_string());
        // stars.push(make_grid(&points));
    }

    stars
        .get(most_neighbors.expect("Nobody had any neighbors!").0)
        .expect("Couldn't find any stars!")
        .to_string()
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let points: Vec<_> = input.lines().map(|line| parse_line(line)).collect();
    println!("part 1: {}", part1(&mut points.clone()));
    // println!("part 2: {}, part2(&input));
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
        println!("{}", part1(&mut points));
    }
}
