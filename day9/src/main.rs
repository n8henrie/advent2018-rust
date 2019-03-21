// Part 1: 429943
// Part 2: 3615691746

use std::collections::LinkedList;

#[derive(Clone, Debug, PartialEq)]
struct GameConfig {
    num_players: usize,
    last_marble_worth: u32,
}

struct Game {
    idx: usize,
    marbles: LinkedList<u32>,
}

impl Game {
    fn new() -> Self {
        let mut marble = LinkedList::new();
        marble.push_back(0);
        Game {
            idx: 0,
            marbles: marble,
        }
    }
    fn add_marble(&mut self, marble: u32) -> u32 {
        if marble % 23 == 0 {
            for _ in 0..7 {
                self.next_back();
            }
            let mut tail = self.marbles.split_off(self.idx);
            let val = tail.pop_front().unwrap();
            self.marbles.append(&mut tail);
            // dbg!((val, marble));
            // dbg!(&self.marbles);
            val + marble
        } else {
            for _ in 0..2 {
                self.next();
            }
            let mut tail = self.marbles.split_off(self.idx);
            tail.push_front(marble);
            self.marbles.append(&mut tail);
            0
        }
    }
}

impl Iterator for Game {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.idx = (self.idx + 1) % self.marbles.len();
        Some(*self.marbles.iter().nth(self.idx).unwrap())
    }
}
impl DoubleEndedIterator for Game {
    fn next_back(&mut self) -> Option<Self::Item> {
        let prev_idx = self.idx.checked_sub(1).unwrap_or(self.marbles.len() - 1);
        self.idx = prev_idx;
        Some(*self.marbles.iter().nth(prev_idx).unwrap())
    }
}

fn part1(config: &GameConfig) -> u32 {
    let mut game = Game::new();
    let mut players = vec![0; config.num_players];
    for (marble, player_idx) in (1..).zip((0..config.num_players).cycle()) {
        // dbg!(format!("adding marble {:#?}", &marble));
        // dbg!(&players);
        // dbg!(&game.idx);
        // dbg!(&game.marbles);
        players[player_idx] += game.add_marble(marble);
        if marble == config.last_marble_worth {
            break;
        }
    }
    *players.iter().max().expect("no max value")
}

fn parse_input(s: &str) -> GameConfig {
    let mut s = s.split_whitespace().filter_map(|word| word.parse().ok());
    let (num_players, last_marble_worth) = (s.next().unwrap(), s.next().unwrap() as u32);
    GameConfig {
        num_players,
        last_marble_worth,
    }
}

fn part2(config: &GameConfig) -> u32 {
    let mut config = (*config).clone();
    config.last_marble_worth *= 100;
    part1(&config)
}

fn main() -> std::io::Result<()> {
    let input = parse_input(std::fs::read_to_string("input.txt")?.as_str());
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "10 players; last marble is worth 1618 points: high score is 8317";
        assert_eq!(
            parse_input(input),
            GameConfig {
                num_players: 10,
                last_marble_worth: 1618
            }
        )
    }

    #[test]
    fn test_part1() {
        let inputs = "
        9 25 32
        10 players; last marble is worth 1618 points: high score is 8317
        13 players; last marble is worth 7999 points: high score is 146373
        17 players; last marble is worth 1104 points: high score is 2764
        21 players; last marble is worth 6111 points: high score is 54718
        30 players; last marble is worth 5807 points: high score is 37305
        ";

        for input in inputs.lines().filter(|&line| !line.trim().is_empty()) {
            let score: u32 = input
                .split_whitespace()
                .last()
                .and_then(|s| s.parse().ok())
                .expect("bad test");
            let config = parse_input(input);
            assert_eq!(part1(&config), score)
        }
    }
}
