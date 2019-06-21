// --- Day 14: Chocolate Charts ---
//
// You finally have a chance to look at all of the produce moving around. Chocolate, cinnamon, mint, chili peppers, nutmeg, vanilla... the Elves must be growing these plants to make hot chocolate! As you realize this, you hear a conversation in the distance. When you go to investigate, you discover two Elves in what appears to be a makeshift underground kitchen/laboratory.
//
// The Elves are trying to come up with the ultimate hot chocolate recipe; they're even maintaining a scoreboard which tracks the quality score (0-9) of each recipe.
//
// Only two recipes are on the board: the first recipe got a score of 3, the second, 7. Each of the two Elves has a current recipe: the first Elf starts with the first recipe, and the second Elf starts with the second recipe.
//
// To create new recipes, the two Elves combine their current recipes. This creates new recipes from the digits of the sum of the current recipes' scores. With the current recipes' scores of 3 and 7, their sum is 10, and so two new recipes would be created: the first with score 1 and the second with score 0. If the current recipes' scores were 2 and 3, the sum, 5, would only create one recipe (with a score of 5) with its single digit.
//
// The new recipes are added to the end of the scoreboard in the order they are created. So, after the first round, the scoreboard is 3, 7, 1, 0.
//
// After all new recipes are added to the scoreboard, each Elf picks a new current recipe. To do this, the Elf steps forward through the scoreboard a number of recipes equal to 1 plus the score of their current recipe. So, after the first round, the first Elf moves forward 1 + 3 = 4 times, while the second Elf moves forward 1 + 7 = 8 times. If they run out of recipes, they loop back around to the beginning. After the first round, both Elves happen to loop around until they land on the same recipe that they had in the beginning; in general, they will move to different recipes.
//
// Drawing the first Elf as parentheses and the second Elf as square brackets, they continue this process:
//
// (3)[7]
// (3)[7] 1  0
//  3  7  1 [0](1) 0
//  3  7  1  0 [1] 0 (1)
// (3) 7  1  0  1  0 [1] 2
//  3  7  1  0 (1) 0  1  2 [4]
//  3  7  1 [0] 1  0 (1) 2  4  5
//  3  7  1  0 [1] 0  1  2 (4) 5  1
//  3 (7) 1  0  1  0 [1] 2  4  5  1  5
//  3  7  1  0  1  0  1  2 [4](5) 1  5  8
//  3 (7) 1  0  1  0  1  2  4  5  1  5  8 [9]
//  3  7  1  0  1  0  1 [2] 4 (5) 1  5  8  9  1  6
//  3  7  1  0  1  0  1  2  4  5 [1] 5  8  9  1 (6) 7
//  3  7  1  0 (1) 0  1  2  4  5  1  5 [8] 9  1  6  7  7
//  3  7 [1] 0  1  0 (1) 2  4  5  1  5  8  9  1  6  7  7  9
//  3  7  1  0 [1] 0  1  2 (4) 5  1  5  8  9  1  6  7  7  9  2
//
// The Elves think their skill will improve after making a few recipes (your puzzle input). However, that could take ages; you can speed this up considerably by identifying the scores of the ten recipes after that. For example:
//
//     If the Elves think their skill will improve after making 9 recipes, the scores of the ten recipes after the first nine on the scoreboard would be 5158916779 (highlighted in the last line of the diagram).
//     After 5 recipes, the scores of the next ten would be 0124515891.
//     After 18 recipes, the scores of the next ten would be 9251071085.
//     After 2018 recipes, the scores of the next ten would be 5941429882.
//
// What are the scores of the ten recipes immediately after the number of recipes in your puzzle input?
//
// Your puzzle input is 846021.
// part 1: 5482326119
// part 2 is not 846021
use std::char;

fn part1(input: u32) -> String {
    let (mut first_elf, mut second_elf) = ((0_usize, 3_u8), (1_usize, 7_u8));
    let mut full_recipe = vec![
        char::from_digit(first_elf.1.into(), 10).unwrap(),
        char::from_digit(second_elf.1.into(), 10).unwrap(),
    ];

    loop {
        let new_recipe = first_elf.1 + second_elf.1;
        full_recipe.extend(new_recipe.to_string().chars());

        let len = full_recipe.len();
        if len as u32 >= input + 10 {
            return full_recipe
                .split_off(input as usize)
                .into_iter()
                .take(10)
                .collect();
            ;
        }

        let first_elf_idx = (first_elf.0 + first_elf.1 as usize + 1) % full_recipe.len();
        let second_elf_idx = (second_elf.0 + second_elf.1 as usize + 1) % full_recipe.len();
        first_elf = (
            first_elf_idx,
            char::to_digit(full_recipe[first_elf_idx], 10).unwrap() as u8,
        );
        second_elf = (
            second_elf_idx,
            char::to_digit(full_recipe[second_elf_idx], 10).unwrap() as u8,
        );
    }
}

fn part2(input: &str) -> usize {
    let (mut first_elf, mut second_elf) = ((0_usize, 3_u8), (1_usize, 7_u8));
    let mut full_recipe = vec![
        char::from_digit(first_elf.1.into(), 10).unwrap(),
        char::from_digit(second_elf.1.into(), 10).unwrap(),
    ];

    loop {
        let new_recipe = first_elf.1 + second_elf.1;
        full_recipe.extend(new_recipe.to_string().chars());

        if let Some(idx) = full_recipe.iter().collect::<String>().find(input) {
            return idx;
        }

        let first_elf_idx = (first_elf.0 + first_elf.1 as usize + 1) % full_recipe.len();
        let second_elf_idx = (second_elf.0 + second_elf.1 as usize + 1) % full_recipe.len();
        first_elf = (
            first_elf_idx,
            char::to_digit(full_recipe[first_elf_idx], 10).unwrap() as u8,
        );
        second_elf = (
            second_elf_idx,
            char::to_digit(full_recipe[second_elf_idx], 10).unwrap() as u8,
        );
    }
}

fn main() {
    println!("part 1: {}", part1(846_021));
    println!("part 2: {}", part2("846_021"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(5), "0124515891");
        assert_eq!(part1(18), "9251071085");
        assert_eq!(part1(2018), "5941429882");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("51589"), 9);
        assert_eq!(part2("01245"), 5);
        assert_eq!(part2("92510"), 18);
        assert_eq!(part2("59414"), 2018);
    }
}
