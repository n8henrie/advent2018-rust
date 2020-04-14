// For example:
//
//     In aA, a and A react, leaving nothing behind.
//     In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.
//     In abAB, no two adjacent units are of the same type, and so nothing happens.
//     In aabAAB, even though aa and AA are of the same type, their polarities match, and so nothing happens.
//
// Now, consider a larger example, dabAcCaCBAcCcaDA:
//
// dabAcCaCBAcCcaDA  The first 'cC' is removed.
// dabAaCBAcCcaDA    This creates 'Aa', which is removed.
// dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
// dabCBAcaDA        No further actions can be taken.
//
// After all possible reactions, the resulting polymer contains 10 units.
//
// How many units remain after fully reacting the polymer you scanned? (Note: in this puzzle and others, the input is large; if you copy/paste your input, make sure you get the whole thing.)

fn part1(input: &str) -> usize {
    let mut index = 0;
    let mut chars = input.chars().collect::<Vec<_>>();
    loop {
        match (chars.get(index), chars.get(index + 1)) {
            (Some(ch0), Some(ch1))
                if (ch0 != ch1) && (ch0.to_lowercase().eq(ch1.to_lowercase())) =>
            {
                chars.remove(index + 1);
                chars.remove(index);
                if index > 0 {
                    index -= 1;
                }
            }
            (_, Some(_)) => index += 1,
            (_, None) => break,
        }
    }
    chars.iter().collect::<String>().len()
}

fn part2(input: &str) -> usize {
    (b'a'..b'z' + 1)
        .map(|c| {
            part1(
                input
                    .replace(c as char, "")
                    .replace((c as char).to_ascii_uppercase(), "")
                    .as_str(),
            )
        })
        .min()
        .unwrap()
}

fn main() {

    let input = std::fs::read_to_string("day5/input.txt").expect("unable to read input file");
    let input = input.trim();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("aBcCdBA"), 5);
        assert_eq!(part1("abcCBA"), 0);
        assert_eq!(part1("aaBB"), 4);
        assert_eq!(part1("aaBBCc"), 4);
        assert_eq!(part1("aABBCC"), 4);
        assert_eq!(part1("dabAcCaCBAcCcaDA"), 10);
    }
}
