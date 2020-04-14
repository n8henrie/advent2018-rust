use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
struct GameConfig {
    num_players: usize,
    last_marble_worth: u32,
}

#[derive(Default)]
struct Marble {
    value: u32,
    next: Option<Rc<RefCell<Marble>>>,
    prev: Option<Rc<RefCell<Marble>>>,
}

impl std::fmt::Debug for Marble {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}, next: {:?}, prev: {:?}",
            &self.value,
            self.next.as_ref().unwrap().borrow().value,
            self.prev.as_ref().unwrap().borrow().value
        )
    }
}

#[derive(Debug)]
struct Game {
    current: Rc<RefCell<Marble>>,
}

impl Game {
    fn new() -> Self {
        let marble = Rc::new(RefCell::new(Marble {
            ..Default::default()
        }));
        marble.borrow_mut().prev = Some(Rc::clone(&marble));
        marble.borrow_mut().next = Some(Rc::clone(&marble));
        Game {
            current: Rc::clone(&marble),
        }
    }
    fn add_marble(&mut self, value: u32) -> u32 {
        if value % 23 == 0 {
            for _ in 0..7 {
                let prev = Rc::clone(self.current.borrow().prev.as_ref().unwrap());
                self.current = prev;
            }
            let score = value + self.current.borrow().value;

            let prev = Rc::clone(self.current.borrow().prev.as_ref().unwrap());
            prev.borrow_mut().next = Some(Rc::clone(self.current.borrow().next.as_ref().unwrap()));
            let next = Rc::clone(self.current.borrow().next.as_ref().unwrap());
            next.borrow_mut().prev = Some(Rc::clone(self.current.borrow().prev.as_ref().unwrap()));
            self.current = next;

            score
        } else {
            for _ in 0..2 {
                let next = Rc::clone(self.current.borrow().next.as_ref().unwrap());
                self.current = next;
            }
            let new = Rc::new(RefCell::new(Marble {
                value,
                ..Default::default()
            }));
            let prev = Rc::clone(self.current.borrow().prev.as_ref().unwrap());
            prev.borrow_mut().next = Some(Rc::clone(&new));
            new.borrow_mut().prev = Some(Rc::clone(&prev));
            let next = Rc::clone(&self.current);
            next.borrow_mut().prev = Some(Rc::clone(&new));
            new.borrow_mut().next = Some(Rc::clone(&next));
            self.current = new;
            0
        }
    }
}

fn part1(config: &GameConfig) -> u32 {
    let mut game = Game::new();
    let mut players = vec![0; config.num_players];
    for (marble, player_idx) in (1..).zip((0..config.num_players).cycle()) {
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

    let input = parse_input(std::fs::read_to_string("day9/input.txt")?.as_str());
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
