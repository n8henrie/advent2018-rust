use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct RegisterCommand {
    before: [usize; 4],
    after: [usize; 4],
    command: [usize; 4],
}

/// addi (add immediate) stores into register C the result of adding register A and value B.
fn addi(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = input[command[1]] + command[2];
    output
}

/// addr (add register) stores into register C the result of adding register A and register B.
fn addr(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = input[command[1]] + input[command[2]];
    output
}

/// (multiply register) stores into register C the result of multiplying register A and register B.
fn mulr(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = input[command[1]] * input[command[2]];
    output
}

/// muli (multiply immediate) stores into register C the result of multiplying register A and value B.
fn muli(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = input[command[1]] * command[2];
    output
}

/// banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
fn banr(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = input[command[1]] & input[command[2]];
    output
}

/// bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
fn bani(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = input[command[1]] & command[2];
    output
}

/// borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
fn borr(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = input[command[1]] | input[command[2]];
    output
}

/// bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
fn bori(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = input[command[1]] | command[2];
    output
}

/// setr (set register) copies the contents of register A into register C. (Input B is ignored.)
fn setr(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = input[command[1]];
    output
}

/// seti (set immediate) stores value A into register C. (Input B is ignored.)
fn seti(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = command[1];
    output
}

/// gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
fn gtir(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = if command[1] > input[command[2]] { 1 } else { 0 };
    output
}

/// gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
fn gtri(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = if input[command[1]] > command[2] { 1 } else { 0 };
    output
}

/// gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
fn gtrr(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = if input[command[1]] > input[command[2]] {
        1
    } else {
        0
    };
    output
}

/// eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
fn eqir(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = if command[1] == input[command[2]] {
        1
    } else {
        0
    };
    output
}

/// eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
fn eqri(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = if input[command[1]] == command[2] {
        1
    } else {
        0
    };
    output
}

/// eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
fn eqrr(command: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut output = *input;
    output[command[3]] = if input[command[1]] == input[command[2]] {
        1
    } else {
        0
    };
    output
}

type Operation = fn(&[usize; 4], &[usize; 4]) -> [usize; 4];

fn part1(reg_commands: &[RegisterCommand], ops: &[Operation]) -> usize {
    reg_commands
        .iter()
        .filter(|rc| {
            ops.iter()
                .filter(|&op| op(&rc.command, &rc.before) == rc.after)
                .count()
                >= 3
        })
        .count()
}

fn part2(reg_commands: &[RegisterCommand], ops: &[Operation], program: &[[usize; 4]]) -> usize {
    let mut candidates: HashMap<usize, Vec<Operation>> =
        (0..ops.len()).map(|idx| (idx, ops.to_vec())).collect();
    let mut command_map: HashMap<usize, Operation> = HashMap::new();

    while command_map.len() != ops.len() {
        for rc in reg_commands {
            let idx = rc.command[0];
            if let Some(v) = candidates.get_mut(&idx) {
                if v.len() == 1 {
                    command_map.insert(idx, v[0]);
                    continue;
                }
                v.retain(|op| op(&rc.command, &rc.before) == rc.after);
            }
            candidates.retain(|k, _| !command_map.contains_key(k));
            candidates.iter_mut().for_each(|(_, cand_vec)| {
                cand_vec.retain(|cand_op| {
                    !command_map
                        .values()
                        .any(|&op| op as usize == *cand_op as usize)
                });
            });
        }
    }

    let mut output = [0; 4];
    for p in program {
        let op = command_map[&p[0]];
        output = op(&p, &output);
    }
    output[0]
}

fn parse_program(input: &[&str]) -> Vec<[usize; 4]> {
    input
        .iter()
        .filter_map(|line| {
            let mut nums = line.split_whitespace();
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                nums.next().and_then(|n| n.parse::<usize>().ok()),
                nums.next().and_then(|n| n.parse::<usize>().ok()),
                nums.next().and_then(|n| n.parse::<usize>().ok()),
                nums.next().and_then(|n| n.parse::<usize>().ok()),
            ) {
                Some([a, b, c, d])
            } else {
                None
            }
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("day16/input.txt")?;
    let commands: Vec<RegisterCommand> = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(4)
        .filter_map(|chunk| chunk.join("\n").parse().ok())
        .collect();
    let ops: Vec<Operation> = vec![
        addi, addr, bani, banr, bori, borr, eqir, eqri, eqrr, gtir, gtri, gtrr, muli, mulr, seti,
        setr,
    ];
    let program = parse_program(&input.lines().skip(commands.len() * 4).collect::<Vec<_>>());

    println!("part1: {}", part1(&commands, &ops));
    println!("part2: {}", part2(&commands, &ops, &program));
    Ok(())
}

impl FromStr for RegisterCommand {
    type Err = Box<dyn std::error::Error>;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parse_line = |line: &str, prefix| {
            if !line.starts_with(prefix) {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Unexpected start of line",
                )));
            };
            let (start, end, sep) = match prefix {
                "Before: " | "After: " => {
                    (line.find('[').unwrap() + 1, line.find(']').unwrap(), ", ")
                }
                "" => (0, line.len(), " "),
                _ => panic!("Unreachable code"),
            };
            if let Some(s) = line.get(start..end) {
                let mut nums = s.split(sep);
                if let (Some(a), Some(b), Some(c), Some(d)) = (
                    nums.next().and_then(|n| n.parse::<usize>().ok()),
                    nums.next().and_then(|n| n.parse::<usize>().ok()),
                    nums.next().and_then(|n| n.parse::<usize>().ok()),
                    nums.next().and_then(|n| n.parse::<usize>().ok()),
                ) {
                    return Ok([a, b, c, d]);
                }
            }
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Did not find numbers in brackets",
            )))
        };

        let mut lines = input.lines();
        if let (Some(before), Some(command), Some(after)) =
            (lines.next(), lines.next(), lines.next())
        {
            let before = parse_line(before, "Before: ")?;
            let command = parse_line(command, "")?;
            let after = parse_line(after, "After: ")?;
            return Ok(RegisterCommand {
                before,
                command,
                after,
            });
        }
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Unable to parse chunk",
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input = "Before: [0, 0, 2, 1]
11 0 1 1
After:  [0, 1, 2, 1]
";
        assert_eq!(
            input.parse::<RegisterCommand>().unwrap(),
            RegisterCommand {
                before: [0, 0, 2, 1],
                command: [11, 0, 1, 1],
                after: [0, 1, 2, 1],
            }
        );
    }

    #[test]
    fn test_addi() {
        let input = "Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
";
        let cmd = input.parse::<RegisterCommand>().unwrap();
        assert_eq!(addi(&cmd.command, &cmd.before), cmd.after);
    }

    #[test]
    fn test_part1() {
        let input = "Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
";
        let ops: Vec<Operation> = vec![
            addi, addr, bani, banr, bori, borr, eqir, eqri, eqrr, gtir, gtri, gtrr, muli, mulr,
            seti, setr,
        ];
        let cmds = vec![input.parse::<RegisterCommand>().unwrap()];
        assert_eq!(part1(&cmds, &ops), 1);
    }
}
