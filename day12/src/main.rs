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

    // https://www.reddit.com/r/rust/comments/bo053e/hey_rustaceans_got_an_easy_question_ask_here/enn75mx/
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

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl FromStr for Rule {
    type Err = Box<dyn std::error::Error>;

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

    // Every iteration will lose 2 from the front due to the windows
    pot_deque.zero_offset -= 2;

    pot_deque.pots = pot_deque
        .windows(5)
        .map(|window| {
            if let Some(rule) = rules
                .iter()
                .find(|rule| rule.pattern.iter().eq(window.clone()))
            {
                rule.outcome
            } else {
                false
            }
        })
        .collect::<VecDeque<_>>();
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

fn part1(input: &str, generations: u64) -> Result<i64> {
    let (mut pot_deque, rules) = parse_input(input)?;
    let mut last_score = 0;
    let mut score_: i64;
    let mut diff = 0_i64;
    for generation in 1..=generations {
        evolve(&mut pot_deque, &rules);

        score_ = score(&pot_deque).into();
        if diff - (score_ - last_score) == 0 {
            return Ok((generations - generation) as i64 * diff + score_);
        }
        diff = score_ - last_score;
        last_score = score_;
    }
    Ok(i64::from(score(&pot_deque)))
}

fn main() -> Result<()> {

    let input = std::fs::read_to_string("day12/input.txt")?;
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

        // In this example, after 20 generations, the pots shown as # contain plants, the furthest
        // left of which is pot -2, and the furthest right of which is pot 34. Adding up all the
        // numbers of plant-containing pots after the 20th generation produces 325.
        assert_eq!(part1(input, 20).unwrap(), 325)
    }
}
