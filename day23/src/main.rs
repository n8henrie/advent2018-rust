// --- Day 23: Experimental Emergency Teleportation ---
//
// Using your torch to search the darkness of the rocky cavern, you finally locate the man's friend: a small reindeer.
//
// You're not sure how it got so far in this cave. It looks sick - too sick to walk - and too heavy for you to carry all the way back. Sleighs won't be invented for another 1500 years, of course.
//
// The only option is experimental emergency teleportation.
//
// You hit the "experimental emergency teleportation" button on the device and push I accept the risk on no fewer than 18 different warning messages. Immediately, the device deploys hundreds of tiny nanobots which fly around the cavern, apparently assembling themselves into a very specific formation. The device lists the X,Y,Z position (pos) for each nanobot as well as its signal radius (r) on its tiny screen (your puzzle input).
//
// Each nanobot can transmit signals to any integer coordinate which is a distance away from it less than or equal to its signal radius (as measured by Manhattan distance). Coordinates a distance away of less than or equal to a nanobot's signal radius are said to be in range of that nanobot.
//
// Before you start the teleportation process, you should determine which nanobot is the strongest (that is, which has the largest signal radius) and then, for that nanobot, the total number of nanobots that are in range of it, including itself.
//
// For example, given the following nanobots:
//
// pos=<0,0,0>, r=4
// pos=<1,0,0>, r=1
// pos=<4,0,0>, r=3
// pos=<0,2,0>, r=1
// pos=<0,5,0>, r=3
// pos=<0,0,3>, r=1
// pos=<1,1,1>, r=1
// pos=<1,1,2>, r=1
// pos=<1,3,1>, r=1
//
// The strongest nanobot is the first one (position 0,0,0) because its signal radius, 4 is the largest. Using that nanobot's location and signal radius, the following nanobots are in or out of range:
//
//     The nanobot at 0,0,0 is distance 0 away, and so it is in range.
//     The nanobot at 1,0,0 is distance 1 away, and so it is in range.
//     The nanobot at 4,0,0 is distance 4 away, and so it is in range.
//     The nanobot at 0,2,0 is distance 2 away, and so it is in range.
//     The nanobot at 0,5,0 is distance 5 away, and so it is not in range.
//     The nanobot at 0,0,3 is distance 3 away, and so it is in range.
//     The nanobot at 1,1,1 is distance 3 away, and so it is in range.
//     The nanobot at 1,1,2 is distance 4 away, and so it is in range.
//     The nanobot at 1,3,1 is distance 5 away, and so it is not in range.
//
// In this example, in total, 7 nanobots are in range of the nanobot with the largest signal radius.
//
// Find the nanobot with the largest signal radius. How many nanobots are in range of its signals?
// --- Part Two ---

// Now, you just need to figure out where to position yourself so that you're actually teleported when the nanobots activate.
//
// To increase the probability of success, you need to find the coordinate which puts you in range of the largest number of nanobots. If there are multiple, choose one closest to your position (0,0,0, measured by manhattan distance).
//
// For example, given the following nanobot formation:
//
// pos=<10,12,12>, r=2
// pos=<12,14,12>, r=2
// pos=<16,12,12>, r=4
// pos=<14,14,14>, r=6
// pos=<50,50,50>, r=200
// pos=<10,10,10>, r=5
//
// Many coordinates are in range of some of the nanobots in this formation. However, only the coordinate 12,12,12 is in range of the most nanobots: it is in range of the first five, but is not in range of the nanobot at 10,10,10. (All other coordinates are in range of fewer than five nanobots.) This coordinate's distance from 0,0,0 is 36.
//
// Find the coordinates that are in range of the largest number of nanobots. What is the shortest manhattan distance between any of those points and 0,0,0?
//
// Although it hasn't changed, you can still get your puzzle input.

// part1: 640
// part2:
//   - wrong: 114653985
use std::collections::BinaryHeap;
use std::convert::TryInto;
use std::error;
use std::io::{self, Write};
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

macro_rules! err {
    ($($tt:tt)*) => { Box::<dyn error::Error>::from(format!($($tt)*)) }
}

fn parse_bot(line: &str) -> Result<Bot> {
    let mut chunks = line.split('=');
    chunks.next();

    let pos_chunk = chunks.next().ok_or_else(|| err!("No second chunk"))?;
    let (start_pos, end_pos) = (
        pos_chunk
            .find('<')
            .ok_or_else(|| err!("Didn't find open bracket"))?,
        pos_chunk
            .find('>')
            .ok_or_else(|| err!("Didn't find close bracket"))?,
    );
    let pos = &pos_chunk[start_pos + 1..end_pos]
        .split(',')
        .map(|s| s.parse::<isize>())
        .collect::<std::result::Result<Vec<_>, _>>()?;

    let r_chunk = chunks.next().ok_or_else(|| err!("No third chunk"))?;
    let r = r_chunk.parse::<usize>()?;
    Ok(Bot {
        pos: (pos[0], pos[1], pos[2]),
        r,
    })
}

#[derive(Debug, Clone, PartialEq)]
struct Bot {
    pos: (isize, isize, isize),
    r: usize,
}

impl FromStr for Bot {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        parse_bot(s)
    }
}

fn part1(input: &str) -> Result<u32> {
    let bots = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Bot>>>()?;
    let max_r = bots
        .iter()
        .max_by_key(|bot| bot.r)
        .ok_or_else(|| err!("No max found"))?;

    Ok(bots
        .iter()
        .filter(|bot| {
            let (p, rp) = (bot.pos, max_r.pos);
            let (x, y, z) = (rp.0 - p.0, rp.1 - p.1, rp.2 - p.2);
            let diff = (x.abs() + y.abs() + z.abs()) as usize;
            diff < max_r.r
        })
        .count() as u32)
}

#[derive(Debug)]
struct Extrema<'a> {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
    zmin: isize,
    zmax: isize,
    volume: u128,
    max_bots: usize,
    bots: &'a [Bot],
}

impl<'a> PartialEq for Extrema<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.xmin == other.xmin
            && self.xmax == other.xmin
            && self.ymin == other.xmin
            && self.ymax == other.xmin
            && self.zmin == other.xmin
            && self.zmax == other.xmin
    }
}

impl<'a> Eq for Extrema<'a> {}

impl<'a> Extrema<'a> {
    fn new(
        xmin: isize,
        xmax: isize,
        ymin: isize,
        ymax: isize,
        zmin: isize,
        zmax: isize,
        bots: &'a [Bot],
    ) -> Result<Self> {
        let volume: u128 =
            (xmax - xmin + 1) as u128 * (ymax - ymin + 1) as u128 * (zmax - zmin + 1) as u128;
        let max_bots = Self::max_bots(xmin, xmax, ymin, ymax, zmin, zmax, bots);

        Ok(Self {
            xmin,
            xmax,
            ymin,
            ymax,
            zmin,
            zmax,
            bots,
            volume: volume.try_into()?,
            // .unwrap_or_else(|_| {
            //     panic!(
            //         "problem with: {} {} {} {} {} {}",
            //         xmin, xmax, ymin, ymax, zmin, zmax
            //     )
            // }),
            max_bots,
        })
    }

    fn max_bots(
        xmin: isize,
        xmax: isize,
        ymin: isize,
        ymax: isize,
        zmin: isize,
        zmax: isize,
        bots: &[Bot],
    ) -> usize {
        bots.iter()
            .filter(|bot| {
                !(bot.pos.0 + (bot.r as isize) < xmin
                    || bot.pos.0 - (bot.r as isize) > xmax
                    || bot.pos.1 + (bot.r as isize) < ymin
                    || bot.pos.1 - (bot.r as isize) > ymax
                    || bot.pos.2 + (bot.r as isize) < zmin
                    || bot.pos.2 - (bot.r as isize) > zmax)
            })
            .count()
    }

    fn from_bots(bots: &'a [Bot]) -> Result<Self> {
        let (mut xmin, mut ymin, mut xmax, mut ymax, mut zmin, mut zmax) = (0, 0, 0, 0, 0, 0);
        for bot in bots.iter() {
            xmin = bot.pos.0.min(xmin);
            ymin = bot.pos.1.min(ymin);
            zmin = bot.pos.2.min(zmin);
            xmax = bot.pos.0.max(xmax);
            ymax = bot.pos.1.max(ymax);
            zmax = bot.pos.2.max(zmax);
        }
        Extrema::new(xmin, xmax, ymin, ymax, zmin, zmax, bots)
    }

    fn halfway((min, max): (isize, isize)) -> Vec<(isize, isize)> {
        if min == max {
            return vec![(min, max)];
        }
        let halfway = ((max - min) / 2) + min;
        vec![(min, halfway), (halfway + 1, max)]
    }

    fn split(self) -> Result<Vec<Extrema<'a>>> {
        let (xs, ys, zs) = (
            Extrema::halfway((self.xmin, self.xmax)),
            Extrema::halfway((self.ymin, self.ymax)),
            Extrema::halfway((self.zmin, self.zmax)),
        );
        let zs = &zs;
        let bots = self.bots;
        xs.into_iter()
            .flat_map(|(xmin, xmax)| {
                ys.iter().flat_map(move |&(ymin, ymax)| {
                    zs.iter().map(move |&(zmin, zmax)| {
                        Extrema::new(xmin, xmax, ymin, ymax, zmin, zmax, bots)
                    })
                })
            })
            .collect::<Result<Vec<_>>>()
    }

    fn calculate_bots(&mut self) {
        self.max_bots = self.bots.iter().fold(0, |acc, bot| {
            let (p, r) = (bot.pos, bot.r);
            if r as isize
                >= ((p.0 - self.xmin).abs() + (p.1 - self.ymin).abs() + (p.2 - self.zmin).abs())
            {
                acc + 1
            } else {
                acc
            }
        });
    }
}

impl<'a> PartialOrd for Extrema<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Extrema<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.max_bots.cmp(&other.max_bots)
        // .then_with(|| other.volume.cmp(&self.volume))
    }
}

fn get_points(bots: &[Bot]) -> Result<Vec<Extrema>> {
    let mut heap = BinaryHeap::new();
    let mut points = <Vec<Extrema>>::new();
    heap.push(Extrema::from_bots(bots)?);

    while let Some(grid) = heap.pop() {
        if let Some(p) = points.first() {
            if grid.max_bots < p.max_bots {
                break;
            }
        }
        if grid.volume != 1 {
            let grids = grid.split()?;
            heap.append(&mut grids.into());
            continue;
        }

        // Calculate actual max_bots instead of estimated
        let mut grid = grid;
        let before = grid.max_bots;
        grid.calculate_bots();
        // dbg!(grid.xmin, grid.ymin, grid.zmin);
        // dbg!(before, grid.max_bots);
        // dbg!(points.len(), heap.len());

        match (grid, points.first()) {
            (g, None) => points.push(g),
            (g, Some(p)) if g.max_bots > p.max_bots => points = vec![g],
            (g, Some(p)) if g.max_bots == p.max_bots => points.push(g),
            _ => (),
        }
    }
    Ok(points)
}

fn part2(input: &str) -> Result<u32> {
    let bots = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Bot>>>()?;
    let points = get_points(&bots)?;
    distance_of_closest_point(&points)
}

fn distance_of_closest_point(points: &[Extrema]) -> Result<u32> {
    Ok(points
        .iter()
        .map(|e| (e.xmin.abs() + e.ymin.abs() + e.zmin.abs()))
        .min()
        .ok_or_else(|| err!("No minimum"))?
        .try_into()?)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    writeln!(io::stdout(), "part1: {}", part1(&input)?)?;
    writeln!(io::stdout(), "part2: {}", part2(&input)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bot() {
        let input = "pos=<60118729,58965711,8716524>, r=71245377";
        let parsed: Bot = input.parse().unwrap();
        let output: Bot = Bot {
            pos: (60_118_729, 58_965_711, 8_716_524),
            r: 71_245_377,
        };

        assert_eq!(parsed, output)
    }

    #[test]
    fn test_part2() {
        let input = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";
        let output = part2(&input);
        assert_eq!(output.unwrap(), 36);
    }

    #[test]
    fn test_volume() {
        let input = std::fs::read_to_string("input.txt").unwrap();
        let bots = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Bot>>>()
            .unwrap();
        let extrema = Extrema::new(-1, 1, -1, 1, -1, 1, &bots).unwrap();
        assert_eq!(extrema.volume, 27);
        let extrema = Extrema::new(-1, 1, -1, 1, 0, 0, &bots).unwrap();
        assert_eq!(extrema.volume, 9);
        let extrema = Extrema::new(0, 0, 0, 0, 0, 0, &bots).unwrap();
        assert_eq!(extrema.volume, 1);
    }

    #[test]
    fn test_halfway() {
        for (i, o) in &[
            ((4, 8), vec![(4, 6), (7, 8)]),
            ((-5, 10), vec![(-5, 2), (3, 10)]),
        ] {
            let output = Extrema::halfway(*i);
            assert_eq!(output, *o);
        }
    }

    #[test]
    fn test_calc_bots() {
        let input = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";

        let bots = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Bot>>>()
            .unwrap();

        let points = get_points(&bots).unwrap();
        assert_eq!(points.len(), 1);
    }
}
