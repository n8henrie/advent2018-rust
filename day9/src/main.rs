// Then, each Elf takes a turn placing the lowest-numbered remaining marble into the circle between the marbles that are 1 and 2 marbles clockwise of the current marble. (When the circle is large enough, this means that there is one marble between the marble that was just placed and the current marble.) The marble that was just placed then becomes the current marble.
//
// However, if the marble that is about to be placed has a number which is a multiple of 23, something entirely different happens. First, the current player keeps the marble they would have placed, adding it to their score. In addition, the marble 7 marbles counter-clockwise from the current marble is removed from the circle and also added to the current player's score. The marble located immediately clockwise of the marble that was removed becomes the new current marble.
// What is the winning Elf's score?

// Amused by the speed of your answer, the Elves are curious:
//
// What would the new winning Elf's score be if the number of the last marble were 100 times larger?

#[derive(Clone, Debug, PartialEq)]
struct GameConfig {
    num_players: usize,
    last_marble_worth: u32,
}

fn part1(config: &GameConfig) -> u32 {
    let mut played_marbles = vec![0];
    let mut current_marble_index: usize = 0;
    let mut players = vec![0; config.num_players];
    for (marble, player_idx) in (1..).zip((0..config.num_players).cycle()) {
        if marble % 10000 == 0 {
            println!(
                "{:.2}%",
                (marble as f32 / config.last_marble_worth as f32) * 100.
            );
        }
        if marble % 23 == 0 {
            current_marble_index = if current_marble_index < 7 {
                played_marbles.len() - (7 - current_marble_index)
            } else {
                current_marble_index - 7
            };
            players[player_idx] += marble + played_marbles.remove(current_marble_index);
        } else {
            current_marble_index = (current_marble_index + 2) % played_marbles.len();
            played_marbles.insert(current_marble_index, marble);
        }
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
