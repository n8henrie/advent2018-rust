use std::collections::BinaryHeap;
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
    max_bots: Option<usize>,
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
        let volume: u128 = (xmax - xmin + 1) as u128
            * (ymax - ymin + 1) as u128
            * (zmax - zmin + 1) as u128;

        let mut pre_calc = Self {
            xmin,
            xmax,
            ymin,
            ymax,
            zmin,
            zmax,
            bots,
            volume,
            max_bots: None,
        };
        pre_calc.calculate_bots();
        Ok(pre_calc)
    }

    fn from_bots(bots: &'a [Bot]) -> Result<Self> {
        let (mut xmin, mut ymin, mut xmax, mut ymax, mut zmin, mut zmax) =
            (0, 0, 0, 0, 0, 0);
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
        self.max_bots = Some(self.bots.iter().fold(0, |acc, bot| {
            let (p, r) = (bot.pos, bot.r);
            if r as u32 >= self.abs_distance(&p) {
                acc + 1
            } else {
                acc
            }
        }));
    }

    fn abs_distance(&self, other: &(isize, isize, isize)) -> u32 {
        let x_dis = {
            if self.xmin <= other.0 && self.xmax >= other.0 {
                0
            } else {
                (self.xmin - other.0).abs().min((self.xmax - other.0).abs())
            }
        };

        let y_dis = {
            if self.ymin <= other.1 && self.ymax >= other.1 {
                0
            } else {
                (self.ymin - other.1).abs().min((self.ymax - other.1).abs())
            }
        };

        let z_dis = {
            if self.zmin <= other.2 && self.zmax >= other.2 {
                0
            } else {
                (self.zmin - other.2).abs().min((self.zmax - other.2).abs())
            }
        };
        (x_dis + y_dis + z_dis) as u32
    }
}

impl<'a> PartialOrd for Extrema<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Extrema<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.max_bots
            .cmp(&other.max_bots)
            .then_with(|| {
                other
                    .abs_distance(&(0, 0, 0))
                    .cmp(&self.abs_distance(&(0, 0, 0)))
            })
            .then_with(|| other.volume.cmp(&self.volume))
    }
}

fn get_point(bots: &[Bot]) -> Result<Extrema> {
    let mut heap = BinaryHeap::new();
    let grid = Extrema::from_bots(bots)?;
    heap.push(grid);

    while let Some(grid) = heap.pop() {
        if grid.volume == 1 {
            return Ok(grid);
        } else {
            heap.append(&mut grid.split()?.into());
        }
    }
    Err(err!("No closest point found"))
}

fn part2(input: &str) -> Result<u32> {
    let bots = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Bot>>>()?;
    let point = get_point(&bots)?;
    distance_of_closest_point(&[point])
}

fn distance_of_closest_point(points: &[Extrema]) -> Result<u32> {
    points
        .iter()
        .map(|e| e.abs_distance(&(0, 0, 0)))
        .min()
        .ok_or_else(|| err!("No minimum"))
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day23/input.txt")?;
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
        let output = part2(input);
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

        let point = get_point(&bots).unwrap();
        for (min, max) in &[
            (point.xmin, point.xmax),
            (point.ymin, point.ymax),
            (point.zmin, point.zmax),
        ] {
            assert_eq!(min, max)
        }
        let pos = (point.xmin, point.ymin, point.zmin);
        assert_eq!(pos, (12, 12, 12))
    }

    #[test]
    fn test_abs_distance() {
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
        let point = Extrema::new(-1, 1, -1, 1, -1, 1, &bots).unwrap();
        assert_eq!(point.abs_distance(&(0, 0, 0)), 0);
        let point = Extrema::new(-4, -2, -5, -3, -12, -10, &bots).unwrap();
        assert_eq!(point.abs_distance(&(0, 0, 0)), 15);
        let point = Extrema::new(2, 4, 3, 5, 10, 12, &bots).unwrap();
        assert_eq!(point.abs_distance(&(0, 0, 0)), 15);
        let point = Extrema::new(0, 0, 0, 0, 0, 0, &bots).unwrap();
        assert_eq!(point.abs_distance(&(1, 1, 1)), 3);
        let point = Extrema::new(10, 20, 10, 20, 10, 20, &bots).unwrap();
        assert_eq!(point.abs_distance(&(15, 15, 5)), 5);
    }

    #[test]
    fn test_heap_sort() {
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
        let grid = Extrema::from_bots(&bots).unwrap();
        // grid.calculate_bots();
        assert_eq!(
            (
                grid.xmin,
                grid.xmax,
                grid.ymin,
                grid.ymax,
                grid.zmin,
                grid.zmax,
                grid.max_bots
            ),
            (0, 50, 0, 50, 0, 50, Some(6))
        );

        let grids = grid.split().unwrap();
        let mut heap = <BinaryHeap<_>>::from(grids);
        let top = heap.pop().unwrap();
        assert_eq!(
            (top.xmin, top.xmax, top.ymin, top.ymax, top.zmin, top.zmax),
            (0, 25, 0, 25, 0, 25)
        );
        heap.append(&mut top.split().unwrap().into());
        let top = heap.pop().unwrap();
        assert_eq!(
            (top.xmin, top.xmax, top.ymin, top.ymax, top.zmin, top.zmax),
            (0, 12, 0, 12, 0, 12)
        );
        heap.append(&mut top.split().unwrap().into());
        let top = heap.pop().unwrap();
        assert_eq!(
            (top.xmin, top.xmax, top.ymin, top.ymax, top.zmin, top.zmax),
            (7, 12, 7, 12, 7, 12)
        );
        heap.append(&mut top.split().unwrap().into());
        let top = heap.pop().unwrap();
        assert_eq!(
            (top.xmin, top.xmax, top.ymin, top.ymax, top.zmin, top.zmax),
            (10, 12, 10, 12, 10, 12)
        );
    }
}
