// --- Day 12: Subterranean Sustainability ---
//
// The year 518 is significantly more underground than your history books implied. Either that, or you've arrived in a vast cavern network under the North Pole.
//
// After exploring a little, you discover a long tunnel that contains a row of small pots as far as you can see to your left and right. A few of them contain plants - someone is trying to grow things in these geothermally-heated caves.
//
// The pots are numbered, with 0 in front of you. To the left, the pots are numbered -1, -2, -3, and so on; to the right, 1, 2, 3.... Your puzzle input contains a list of pots from 0 to the right and whether they do (#) or do not (.) currently contain a plant, the initial state. (No other pots currently contain plants.) For example, an initial state of #..##.... indicates that pots 0, 3, and 4 currently contain plants.
//
// Your puzzle input also contains some notes you find on a nearby table: someone has been trying to figure out how these plants spread to nearby pots. Based on the notes, for each generation of plants, a given pot has or does not have a plant based on whether that pot (and the two pots on either side of it) had a plant in the last generation. These are written as LLCRR => N, where L are pots to the left, C is the current pot being considered, R are the pots to the right, and N is whether the current pot will have a plant in the next generation. For example:
//
//     A note like ..#.. => . means that a pot that contains a plant but with no plants within two pots of it will not have a plant in it during the next generation.
//     A note like ##.## => . means that an empty pot with two plants on each side of it will remain empty in the next generation.
//     A note like .##.# => # means that a pot has a plant in a given generation if, in the previous generation, there were plants in that pot, the one immediately to the left, and the one two pots to the right, but not in the ones immediately to the right and two to the left.
//
// It's not clear what these plants are for, but you're sure it's important, so you'd like to make sure the current configuration of plants is sustainable by determining what will happen after 20 generations.
//
// For example, given the following input:
//
// initial state: #..#.#..##......###...###
//
// ...## => #
// ..#.. => #
// .#... => #
// .#.#. => #
// .#.## => #
// .##.. => #
// .#### => #
// #.#.# => #
// #.### => #
// ##.#. => #
// ##.## => #
// ###.. => #
// ###.# => #
// ####. => #
//
// For brevity, in this example, only the combinations which do produce a plant are listed. (Your input includes all possible combinations.) Then, the next 20 generations will look like this:
//
//                  1         2         3
//        0         0         0         0
//  0: ...#..#.#..##......###...###...........
//  1: ...#...#....#.....#..#..#..#...........
//  2: ...##..##...##....#..#..#..##..........
//  3: ..#.#...#..#.#....#..#..#...#..........
//  4: ...#.#..#...#.#...#..#..##..##.........
//  5: ....#...##...#.#..#..#...#...#.........
//  6: ....##.#.#....#...#..##..##..##........
//  7: ...#..###.#...##..#...#...#...#........
//  8: ...#....##.#.#.#..##..##..##..##.......
//  9: ...##..#..#####....#...#...#...#.......
// 10: ..#.#..#...#.##....##..##..##..##......
// 11: ...#...##...#.#...#.#...#...#...#......
// 12: ...##.#.#....#.#...#.#..##..##..##.....
// 13: ..#..###.#....#.#...#....#...#...#.....
// 14: ..#....##.#....#.#..##...##..##..##....
// 15: ..##..#..#.#....#....#..#.#...#...#....
// 16: .#.#..#...#.#...##...#...#.#..##..##...
// 17: ..#...##...#.#.#.#...##...#....#...#...
// 18: ..##.#.#....#####.#.#.#...##...##..##..
// 19: .#..###.#..#.#.#######.#.#.#..#.#...#..
// 20: .#....##....#####...#######....#.#..##.
//
// The generation is shown along the left, where 0 is the initial state. The pot numbers are shown along the top, where 0 labels the center pot, negative-numbered pots extend to the left, and positive pots extend toward the right. Remember, the initial state begins at pot 0, which is not the leftmost pot used in this example.
//
// After one generation, only seven plants remain. The one in pot 0 matched the rule looking for ..#.., the one in pot 4 matched the rule looking for .#.#., pot 9 matched .##.., and so on.
//
// In this example, after 20 generations, the pots shown as # contain plants, the furthest left of which is pot -2, and the furthest right of which is pot 34. Adding up all the numbers of plant-containing pots after the 20th generation produces 325.
//
// After 20 generations, what is the sum of the numbers of all pots which contain a plant?
// part1 == 4818
//
use std::collections::VecDeque;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
struct Rule {
    pattern: Vec<bool>,
    outcome: bool,
}

#[derive(Debug, PartialEq)]
struct PotDeque {
    pots: VecDeque<bool>,
    zero_offset: i32,
}

impl PotDeque {
    fn new(vd: &[bool]) -> Self {
        PotDeque {
            pots: Vec::from(vd).into(),
            zero_offset: 0,
        }
    }

    fn windows(
        &self,
        window_len: usize,
    ) -> impl Debug + Iterator<Item = impl Debug + Clone + Iterator<Item = &bool>> {
        let mut base_iter = self.pots.iter();
        std::iter::from_fn(move || {
            if base_iter.len() >= window_len {
                // Clone the base iterator at its current position to make a window.
                let window = base_iter.clone().take(window_len);
                // Move the base iterator forward.
                base_iter.next();
                Some(window)
            } else {
                None
            }
        })
    }
}

impl std::fmt::Display for PotDeque {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.pots
                .iter()
                .map(|&p| if p { '#' } else { '.' })
                .collect::<String>()
        )
    }
}

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

impl FromStr for Rule {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut chars = s.trim().chars();
        let pattern: Vec<_> = chars.by_ref().take(5).map(|c| c == '#').collect();
        let outcome = chars
            .nth(4)
            .ok_or_else(|| Self::Err::from("No 5th character"))?
            == '#';
        Ok(Rule { pattern, outcome })
    }
}

fn parse_input(input: &str) -> Result<(PotDeque, Vec<Rule>)> {
    let mut lines = input.lines();
    let initial_state: Vec<bool> = lines
        .next()
        .expect("No first line")
        .chars()
        .skip(15)
        .map(|c| c == '#')
        .collect();
    lines.next();
    let rules = lines.map(str::parse).collect::<Result<Vec<_>>>()?;
    Ok((PotDeque::new(&initial_state), rules))
}

fn evolve(pot_deque: &mut PotDeque, rules: &[Rule]) {
    let add_to_front = pot_deque.pots.iter().take(5).position(|&x| x);
    if let Some(num) = add_to_front {
        (0..(5 - num)).for_each(|_| {
            pot_deque.pots.push_front(false);
            pot_deque.zero_offset += 1;
        });
    }

    let add_to_back = pot_deque.pots.iter().rev().take(5).position(|&x| x);
    if let Some(num) = add_to_back {
        (0..(5 - num)).for_each(|_| pot_deque.pots.push_back(false));
    }

    // Maintain the zero_offset, since windows will start producing the new values at an index of 2
    let mut next_generation = vec![false, false];
    for window in pot_deque.windows(5) {
        if let Some(rule) = rules
            .iter()
            .find(|rule| rule.pattern.iter().eq(window.clone()))
        {
            next_generation.push(rule.outcome);
        } else {
            next_generation.push(false);
        }
    }
    pot_deque.pots = next_generation.into();
}

fn score(pot_deque: &PotDeque) -> i32 {
    let nums = -pot_deque.zero_offset..;
    pot_deque
        .pots
        .iter()
        .zip(nums)
        .map(|(p, num)| if *p { num } else { 0 })
        .sum()
}

fn part1(input: &str, generations: u64) -> Result<i32> {
    let (mut pot_deque, rules) = parse_input(input)?;
    for generation in 1..=generations {
        evolve(&mut pot_deque, &rules);
        if generation % 10_000 == 0 {
            println!(
                "{:.2}% complete",
                (generation as f64 / generations as f64) * 100.
            );
        }
    }
    Ok(score(&pot_deque))
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    println!("part 1: {}", part1(&input, 20)?);
    println!("part 2: {}", part1(&input, 50_000_000_000)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let input = "initial state: #..#.#..##......###...###

        ...## => #
        ...## => .
        ..#.. => #";
        assert_eq!(
            parse_input(input).unwrap(),
            (
                PotDeque::new(&[
                    true, false, false, true, false, true, false, false, true, true, false, false,
                    false, false, false, false, true, true, true, false, false, false, true, true,
                    true
                ]),
                vec![
                    Rule {
                        pattern: vec![false, false, false, true, true],
                        outcome: true
                    },
                    Rule {
                        pattern: vec![false, false, false, true, true],
                        outcome: false
                    },
                    Rule {
                        pattern: vec![false, false, true, false, false],
                        outcome: true
                    },
                ]
            )
        )
    }
    #[test]
    fn test_part1() {
        let input = "initial state: #..#.#..##......###...###

        ...## => #
        ..#.. => #
        .#... => #
        .#.#. => #
        .#.## => #
        .##.. => #
        .#### => #
        #.#.# => #
        #.### => #
        ##.#. => #
        ##.## => #
        ###.. => #
        ###.# => #
        ####. => #";

        //  0: ...#..#.#..##......###...###...........
        //  1: ...#...#....#.....#..#..#..#...........
        //  2: ...##..##...##....#..#..#..##..........
        //  3: ..#.#...#..#.#....#..#..#...#..........
        //  4: ...#.#..#...#.#...#..#..##..##.........
        //  5: ....#...##...#.#..#..#...#...#.........
        //  6: ....##.#.#....#...#..##..##..##........
        //  7: ...#..###.#...##..#...#...#...#........
        //  8: ...#....##.#.#.#..##..##..##..##.......
        //  9: ...##..#..#####....#...#...#...#.......
        // 10: ..#.#..#...#.##....##..##..##..##......
        // 11: ...#...##...#.#...#.#...#...#...#......
        // 12: ...##.#.#....#.#...#.#..##..##..##.....
        // 13: ..#..###.#....#.#...#....#...#...#.....
        // 14: ..#....##.#....#.#..##...##..##..##....
        // 15: ..##..#..#.#....#....#..#.#...#...#....
        // 16: .#.#..#...#.#...##...#...#.#..##..##...
        // 17: ..#...##...#.#.#.#...##...#....#...#...
        // 18: ..##.#.#....#####.#.#.#...##...##..##..
        // 19: .#..###.#..#.#.#######.#.#.#..#.#...#..
        // 20: .#....##....#####...#######....#.#..##.

        // In this example, after 20 generations, the pots shown as # contain plants, the furthest left of which is pot -2, and the furthest right of which is pot 34. Adding up all the numbers of plant-containing pots after the 20th generation produces 325.
        assert_eq!(part1(&input, 20).unwrap(), 325)
    }
}
