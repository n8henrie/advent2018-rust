use std::collections::HashSet;
use std::error;
use std::io::{Error, ErrorKind};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

type Arguments = [usize; 3];

/// addi (add immediate) stores into register C the result of adding register A and value B.
fn addi(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = input[command[0]] + command[1];
}

/// addr (add register) stores into register C the result of adding register A and register B.
fn addr(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = input[command[0]] + input[command[1]];
}

/// (multiply register) stores into register C the result of multiplying register A and register B.
fn mulr(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = input[command[0]] * input[command[1]];
}

/// muli (multiply immediate) stores into register C the result of multiplying register A and value B.
fn muli(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = input[command[0]] * command[1];
}

/// banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
fn banr(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = input[command[0]] & input[command[1]];
}

/// bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
fn bani(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = input[command[0]] & command[1];
}

/// borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
fn borr(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = input[command[0]] | input[command[1]];
}

/// bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
fn bori(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = input[command[0]] | command[1];
}

/// setr (set register) copies the contents of register A into register C. (Input B is ignored.)
fn setr(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = input[command[0]];
}

/// seti (set immediate) stores value A into register C. (Input B is ignored.)
fn seti(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = command[0];
}

/// gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
fn gtir(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = if command[0] > input[command[1]] { 1 } else { 0 };
}

/// gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
fn gtri(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = if input[command[0]] > command[1] { 1 } else { 0 };
}

/// gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
fn gtrr(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = if input[command[0]] > input[command[1]] {
        1
    } else {
        0
    };
}

/// eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
fn eqir(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = if command[0] == input[command[1]] {
        1
    } else {
        0
    };
}

/// eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
fn eqri(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = if input[command[0]] == command[1] {
        1
    } else {
        0
    };
}

/// eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
fn eqrr(command: &Arguments, input: &mut [usize; 6]) {
    input[command[2]] = if input[command[0]] == input[command[1]] {
        1
    } else {
        0
    };
}

struct Program<'a> {
    ip: usize,
    cmds: Vec<(&'a str, [usize; 3])>,
}

impl<'a> Program<'a> {
    // Looking at the input, step 28 (line 30) is the only place that register[0] is even looked
    // at, and it isn't set anywhere. If register[4] == register[0] at step 28, the program will
    // end due to step 29, which will skip step 30, so we will just return the value of register[4]
    // the first time step 28 is reached.
    fn run(&self, regzero: usize) -> Result<(Option<usize>, Option<usize>)> {
        let ip = self.ip;
        let mut register: [usize; 6] = [0; 6];
        register[0] = regzero;

        let mut seen = HashSet::new();
        let mut fourset = HashSet::new();
        let mut fours = (None, None);

        while let Some((progname, args)) = self.cmds.get(register[ip]) {
            if register[ip] == 18 && ((register[2] + 1) * 256) <= register[3] {
                register[1] = 0;
                register[2] = register[3] / 256;
                register[5] = 18;
                continue;
            }
            let prog = match *progname {
                "addi" => addi,
                "addr" => addr,
                "bani" => bani,
                "bori" => bori,
                "eqri" => eqri,
                "eqrr" => eqrr,
                "gtir" => gtir,
                "gtrr" => gtrr,
                "muli" => muli,
                "seti" => seti,
                "setr" => setr,
                _ => unreachable!(),
            };
            prog(args, &mut register);
            register[ip] += 1;
            if register[ip] == 28 && fourset.insert(register[4]) {
                fours = match fours {
                    (None, _) => (Some(register[4]), None),
                    (Some(num), _) => (Some(num), Some(register[4])),
                }
            }
            if !seen.insert(register) {
                return Ok(fours);
            }
        }
        Err(Box::new(Error::new(ErrorKind::Other, "Out of bounds")))
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut lines = input.lines();
    let ip = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let program = Program {
        ip,
        cmds: lines
            .map(|line| {
                let mut parts = line.split_whitespace();
                if let (Some(cmd), Some(a), Some(b), Some(c)) =
                    (parts.next(), parts.next(), parts.next(), parts.next())
                {
                    (
                        cmd,
                        [a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap()],
                    )
                } else {
                    panic!("Parse error!")
                }
            })
            .collect(),
    };
    let solution = program.run(0)?;
    println!("part1: {}", solution.0.unwrap());
    println!("part2: {}", solution.1.unwrap());
    Ok(())
}
