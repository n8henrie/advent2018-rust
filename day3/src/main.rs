use nom::{
    char, digit, do_parse, map_res, named, separated_list_complete, space,
    types::CompleteStr,
};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rect {
    pub id: u32,
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    fn get_boundaries(&self) -> (u16, u16, u16, u16) {
        (self.x, self.x + self.w - 1, self.y, self.y + self.h - 1)
    }
    fn overlaps(&self, rect: &Self) -> bool {
        let (xmin1, xmax1, ymin1, ymax1) = self.get_boundaries();
        let (xmin2, xmax2, ymin2, ymax2) = rect.get_boundaries();
        !(xmax1 < xmin2 || xmax2 < xmin1 || ymax1 < ymin2 || ymax2 < ymin1)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug, PartialEq)]
struct Grid {
    points: HashMap<Point, u32>,
}

impl Grid {
    fn new(h: u16, w: u16) -> Self {
        Self {
            points: (0..h)
                .flat_map(|y| (0..w).map(move |x| (Point { x, y }, 0)))
                .collect(),
        }
    }

    fn add_rect(&mut self, rect: &Rect) {
        for x in rect.x..(rect.x + rect.w) {
            for y in rect.y..(rect.y + rect.h) {
                let val = self.points.get_mut(&Point { x, y }).unwrap_or_else(
                    || {
                        panic!(
                            "Grid does not contain point {}",
                            format!("{},{}", x, y)
                        )
                    },
                );
                *val += 1;
            }
        }
    }
}

named!(make_rect<CompleteStr, Rect>,
do_parse!(
    char!('#') >>
    id: map_res!(digit, |CompleteStr(s)| FromStr::from_str(s)) >>
    space >>
    char!('@') >>
    space >>
    x: map_res!(digit, |CompleteStr(s)| FromStr::from_str(s)) >>
    char!(',') >>
    y: map_res!(digit, |CompleteStr(s)| FromStr::from_str(s)) >>
    char!(':') >>
    space >>
    w: map_res!(digit, |CompleteStr(s)| FromStr::from_str(s)) >>
    char!('x') >>
    h: map_res!(digit, |CompleteStr(s)| FromStr::from_str(s)) >>
    ( Rect { id, x, y, w, h } )
    )

);

named!(parse_to_rects<CompleteStr, Vec<Rect>>,
separated_list_complete!(
    char!('\n'),
    make_rect)
);

fn parse(input: &str) -> Vec<Rect> {
    // Could just unwrap but probaby good to get used to handling specific error cases
    match parse_to_rects(CompleteStr(input)) {
        Ok((_remaining, value)) => value,
        Err(nom::Err::Incomplete(needed)) => panic!("needed: {:?}", needed),
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
            panic!("error: {:?}", e)
        }
    }
}

fn part1(input: &str) -> u32 {
    let rects = parse(input);
    let mut grid = Grid::new(1000, 1000);
    for rect in &rects {
        grid.add_rect(rect);
    }

    grid.points.values().filter(|&count| *count > 1).count() as u32
}

fn part2(input: &str) -> u32 {
    let rects = parse(input);

    rects
        .iter()
        .find(|r1| rects.iter().all(|r2| !r1.overlaps(r2) || r1.id == r2.id))
        .unwrap()
        .id
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("day3/input.txt")?;
    println!("Number of overlapping points: {}", part1(&input));
    println!("ID of claim with all unique points: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parser() {
        assert_eq!(
            parse("#10 @ 82,901: 26x12\n#2 @ 8,540: 18x12"),
            vec![
                Rect {
                    id: 10,
                    x: 82,
                    y: 901,
                    w: 26,
                    h: 12
                },
                Rect {
                    id: 2,
                    x: 8,
                    y: 540,
                    w: 18,
                    h: 12
                },
            ]
        )
    }
    #[test]
    fn test_new_grid() {
        let g1 = Grid::new(2, 3);
        let g2 = Grid::new(2, 3);

        let mut points = HashMap::new();
        for y in 0..2 {
            for x in 0..3 {
                points.insert(Point { x, y }, 0);
            }
        }
        let g3 = Grid { points };
        assert_eq!(g1, g2);
        assert_eq!(g2, g3);
    }

    #[test]
    fn test_add_rect() {
        let mut g1 = Grid::new(3, 4);
        let rect = Rect {
            id: 0,
            x: 1,
            y: 2,
            w: 3,
            h: 1,
        };
        g1.add_rect(&rect);

        let mut points: HashMap<Point, u32> = (0..3)
            .flat_map(|y| (0..4).map(move |x| (Point { x, y }, 0)))
            .collect();
        for (x, y) in &[(1, 2), (2, 2), (3, 2)] {
            points.insert(Point { x: *x, y: *y }, 1);
        }
        let g2 = Grid { points };
        assert_eq!(g1, g2);
    }

    #[test]
    #[should_panic]
    fn test_rect_too_big() {
        let mut g1 = Grid::new(4, 3);
        g1.add_rect(&Rect {
            id: 0,
            x: 1,
            y: 2,
            w: 3,
            h: 1,
        });
    }

    #[test]
    fn test_rect_overlaps() {
        let r1 = Rect {
            id: 0,
            x: 1,
            y: 2,
            w: 2,
            h: 2,
        };
        let r2 = Rect {
            id: 2,
            x: 2,
            y: 3,
            w: 2,
            h: 2,
        };
        let r3 = Rect {
            id: 3,
            x: 3,
            y: 4,
            w: 2,
            h: 3,
        };
        assert!(r1.overlaps(&r2));
        assert!(r2.overlaps(&r3));
        assert!(r2.overlaps(&r1));
        assert!(r3.overlaps(&r2));
        assert!(!r1.overlaps(&r3));
        assert!(!r3.overlaps(&r1));
    }
}
