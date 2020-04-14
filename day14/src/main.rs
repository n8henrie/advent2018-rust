use std::char;
use std::collections::VecDeque;

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
    let mut check_queue = VecDeque::new();
    let start_char = input.chars().next().unwrap();

    loop {
        let new_recipe: Vec<_> = (first_elf.1 + second_elf.1).to_string().chars().collect();
        if new_recipe.contains(&start_char) {
            check_queue.push_back(full_recipe.len() - 1);
        }

        full_recipe.extend(new_recipe);

        if let Some(idx) = check_queue.get(0) {
            if full_recipe.len().saturating_sub(input.len()) > *idx {
                {
                    check_queue.pop_front();

                    if full_recipe
                        .get(full_recipe.len() - input.len() - 2..)
                        .unwrap()
                        .iter()
                        .collect::<String>()
                        .contains(input)
                    {
                        return full_recipe.iter().collect::<String>().find(input).unwrap();
                    }
                }
            }
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
    println!("part 2: {}", part2("846021"));
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
