use std::collections::HashMap;

fn power_level(coords: (u32, u32), grid_serial_number: u32) -> i32 {
    let rack_id = coords.0 + 10;
    let mut power_level = coords.1 * rack_id + grid_serial_number;
    power_level *= rack_id;
    power_level = power_level
        .to_string()
        .chars()
        .rev()
        .nth(2)
        .expect("No hundreds digit")
        .to_digit(10)
        .expect("Unable to parse as digit");
    power_level as i32 - 5
}

fn sized_power_level(
    start_coords: (u32, u32),
    size: u32,
    lookup_table: &HashMap<(u32, u32), i32>,
) -> i32 {
    (start_coords.0..=300.min(start_coords.0 + size))
        .flat_map(|x| {
            (start_coords.1..=300.min(start_coords.1 + size))
                .map(move |y| lookup_table.get(&(x, y)).unwrap())
        })
        .sum()
}

fn part1(lookup_table: &HashMap<(u32, u32), i32>) -> (u32, u32) {
    (1..=300u32)
        .flat_map(|x| (1..=300u32).map(move |y| (x, y)))
        .max_by_key(|(x, y)| sized_power_level((*x, *y), 3, lookup_table))
        .expect("No max found")
}

fn part2(lookup_table: &HashMap<(u32, u32), i32>) -> (u32, u32, u32, i32) {
    let mut best: Option<(u32, u32, u32, i32)> = None;
    for x in 1..=300u32 {
        for y in 1..=300u32 {
            let mut prev = 0;
            for s in 1..=(300 - x).min(300 - y) {
                prev += (0..s)
                    .map(|dy| lookup_table.get(&(x + s - 1, y + dy)).unwrap())
                    .sum::<i32>();
                prev += (0..(s - 1))
                    .map(|dx| lookup_table.get(&(x + dx, y + s - 1)).unwrap())
                    .sum::<i32>();
                if let Some(val) = best {
                    if val.3 <= prev {
                        best = Some((x, y, s, prev));
                    }
                } else {
                    best = Some((x, y, s, prev));
                }
            }
        }
    }
    best.unwrap()
}

fn main() {
    let input = 7400;
    let lookup_table: HashMap<(u32, u32), i32> = (1..=300)
        .flat_map(|x| (1..=300).map(move |y| ((x, y), power_level((x, y), input))))
        .collect();
    let p1 = part1(&lookup_table);
    println!("part1: {},{}", p1.0, p1.1);
    let p2 = part2(&lookup_table);
    println!("part2: {},{},{}", p2.0, p2.1, p2.2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level((122, 79), 57), -5);
        assert_eq!(power_level((217, 196), 39), 0);
        assert_eq!(power_level((101, 153), 71), 4);
    }

    #[test]
    fn test_part2() {
        let input = 42;
        let lookup_table: HashMap<(u32, u32), i32> = (1..=300)
            .flat_map(|x| (1..=300).map(move |y| ((x, y), power_level((x, y), input))))
            .collect();
        assert_eq!(part2(&lookup_table), (232, 251, 12, 119));
    }
}
