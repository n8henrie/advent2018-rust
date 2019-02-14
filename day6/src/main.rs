use nom::{complete, do_parse, is_digit, many1, map_res, named, newline, opt, tag, take_while};

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
    nearest: Option<usize>,
}

struct Grid {
    min_x: u16,
    max_x: u16,
    min_y: u16,
    max_y: u16,
    points: Vec<Point>,
}

impl Grid {
    fn new(points: &Vec<Point>) -> Self {
        let min_x = points.iter().min_by_key(|p| p.x).unwrap().x;
        let max_x = points.iter().max_by_key(|p| p.x).unwrap().x;
        let min_y = points.iter().min_by_key(|p| p.y).unwrap().y;
        let max_y = points.iter().max_by_key(|p| p.y).unwrap().y;

        Grid {
            min_x,
            max_x,
            min_y,
            max_y,
            points: (min_x..=max_x)
                .flat_map(|x| {
                    (min_y..=max_y).map(move |y| Point {
                        x,
                        y,
                        nearest: None,
                    })
                })
                .collect::<Vec<_>>(),
        }
    }
}

named!(parse_num<&[u8], u16>,
       map_res!(
           map_res!(take_while!(is_digit), std::str::from_utf8), {
           |s: &str| s.parse::<u16>()
               })
       );

named!(parse_line<&[u8], Point>,
       do_parse!(
           x: parse_num >>
           tag!(", ") >>
           y: parse_num >>
           opt!(newline) >>
           ( Point { x, y, nearest: None } )
           )
       );

named!(parse_input<&[u8], Vec<Point>>,
    many1!(complete!(parse_line))
);

fn part1(points: &Vec<Point>) -> u32 {
    let mut grid = Grid::new(points);

    // For each point in the grid, find the nearest point
    for point in grid.points.iter_mut() {
        point.nearest = {
            // Vec<(id, distance>
            let distances = points
                .iter()
                .enumerate()
                .map(|(idx, p)| {
                    (
                        idx,
                        (p.x as i32 - point.x as i32).abs() + (p.y as i32 - point.y as i32).abs(),
                    )
                })
                .collect::<Vec<_>>();
            let min_distance = distances.iter().min_by_key(|(_i, d)| d).unwrap();
            if distances
                .iter()
                .filter(|(_i, d)| *d == min_distance.1)
                .collect::<Vec<_>>()
                .len()
                > 1
            {
                // grid with multiple nearest points
                None
            } else {
                // index of single nearest point
                Some(min_distance.0)
            }
        }
    }

    // Collect points into tuples of (index, count of grid with this point as nearest)
    let touches_edges = grid
        .points
        .iter()
        .filter_map(|p| {
            if [grid.min_x, grid.max_x].contains(&p.x) || [grid.min_y, grid.max_y].contains(&p.y) {
                Some(p.nearest)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let counts = points
        .iter()
        .enumerate()
        .map(|(i, _p)| {
            (
                i,
                grid.points
                    .iter()
                    .filter(|&p| {
                        if let Some(nearest) = p.nearest {
                            nearest == i
                        } else {
                            false
                        }
                    })
                    .collect::<Vec<_>>()
                    .len(),
            )
        })
        .collect::<Vec<_>>();
    counts
        .iter()
        .filter(|(i, _c)| !touches_edges.contains(&Some(*i)))
        .max_by_key(|(_i, c)| c)
        .unwrap()
        .1 as u32
}

fn part2(points: &Vec<Point>) -> u32 {
    Grid::new(points)
        .points
        .iter()
        .filter(|&gp| {
            points
                .iter()
                .map(|p| (p.x as i32 - gp.x as i32).abs() + (p.y as i32 - gp.y as i32).abs())
                .sum::<i32>()
                < 10000
        })
        .collect::<Vec<_>>()
        .len() as u32
}

fn main() {
    let input = include_bytes!("../input.txt");
    let (_incomplete, points) = parse_input(input).unwrap();
    println!("part 1 solution: {}", part1(&points));
    println!("part 2 solution: {}", part2(&points));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_grid() {
        let points = vec![
            Point {
                x: 1,
                y: 1,
                nearest: None,
            },
            Point {
                x: 1,
                y: 6,
                nearest: None,
            },
            Point {
                x: 8,
                y: 3,
                nearest: None,
            },
            Point {
                x: 3,
                y: 4,
                nearest: None,
            },
            Point {
                x: 5,
                y: 5,
                nearest: None,
            },
            Point {
                x: 8,
                y: 9,
                nearest: None,
            },
        ];
        assert_eq!(part1(&points), 17)
    }
}
