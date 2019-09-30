use std::convert::{TryFrom, TryInto};
use std::io::{Error, ErrorKind::InvalidInput};
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

trait Usizeable:
    TryInto<usize, Error = <Self as Usizeable>::Error>
    + TryFrom<usize, Error = <Self as Usizeable>::Error>
    + std::ops::Add<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::BitAnd<Output = Self>
    + std::ops::BitOr<Output = Self>
    + std::ops::AddAssign
    + PartialOrd
    + PartialEq
    + Copy
    + std::fmt::Debug
where
    <Self as Usizeable>::Error: std::fmt::Debug,
{
    type Error;
}

impl Usizeable for u128 {
    type Error = std::num::TryFromIntError;
}

impl Usizeable for usize {
    type Error = std::convert::Infallible;
}

// where <T as std::convert::TryFrom<usize>>::Error: std::fmt::Debug,
/// addi (add immediate) stores into register C the result of adding register A and value B.
fn addi<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] = register[arguments[0]] + arguments[1].try_into().unwrap();
}

/// addr (add register) stores into register C the result of adding register A and register B.
fn addr<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] = register[arguments[0]] + register[arguments[1]];
}

/// (multiply register) stores into register C the result of multiplying register A and register B.
fn mulr<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] = register[arguments[0]] * register[arguments[1]];
}

/// muli (multiply immediate) stores into register C the result of multiplying register A and value B.
fn muli<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] = register[arguments[0]] * arguments[1].try_into().unwrap();
}

/// banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
fn banr<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] = register[arguments[0]] & register[arguments[1]];
}

/// bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
fn bani<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] = register[arguments[0]] & arguments[1].try_into().unwrap();
}

/// borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
fn borr<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] = register[arguments[0]] | register[arguments[1]];
}

/// bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
fn bori<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] = register[arguments[0]] | arguments[1].try_into().unwrap();
}

/// setr (set register) copies the contents of register A into register C. (Input B is ignored.)
fn setr<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] = register[arguments[0]];
}

/// seti (set immediate) stores value A into register C. (Input B is ignored.)
fn seti<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] = arguments[0].try_into().unwrap();
}

/// gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
fn gtir<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] =
        if TryInto::<T>::try_into(arguments[0]).unwrap() > register[arguments[1]] {
            1.try_into().unwrap()
        } else {
            0.try_into().unwrap()
        };
}

/// gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
fn gtri<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] =
        if register[arguments[0]] > TryInto::<T>::try_into(arguments[1]).unwrap() {
            1.try_into().unwrap()
        } else {
            0.try_into().unwrap()
        };
}

/// gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
fn gtrr<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] = if register[arguments[0]] > register[arguments[1]] {
        1.try_into().unwrap()
    } else {
        0.try_into().unwrap()
    };
}

/// eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
fn eqir<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] = if arguments[0] == register[arguments[1]].try_into().unwrap() {
        1.try_into().unwrap()
    } else {
        0.try_into().unwrap()
    };
}

/// eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
fn eqri<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] = if register[arguments[0]].try_into().unwrap() == arguments[1] {
        1.try_into().unwrap()
    } else {
        0.try_into().unwrap()
    };
}

/// eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
fn eqrr<T: Usizeable>(arguments: Arguments, register: &mut Register<T>) {
    register[arguments[2]] = if register[arguments[0]] == register[arguments[1]] {
        1.try_into().unwrap()
    } else {
        0.try_into().unwrap()
    };
}

struct Program<T> {
    ip: usize,
    commands: Vec<Command<T>>,
}

type Register<T> = [T; 6];
type Arguments = [usize; 3];
type Command<T> = Box<dyn Fn(&mut Register<T>)>;

impl<T: Usizeable> FromStr for Program<T> {
    type Err = Box<dyn std::error::Error>;
    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines = input.lines();
        let header = lines
            .next()
            .ok_or_else(|| Error::new(InvalidInput, "No header present"))?;
        let ip: usize = header
            .split_whitespace()
            .last()
            .ok_or_else(|| Error::new(InvalidInput, "No ip found in header"))?
            .parse()?;
        let commands: Vec<Command<T>> = lines
            .map(|line| {
                let mut words = line.split_whitespace();
                let cmd = words.next().expect("No first word");
                let mut args = words.map(|word| word.parse::<usize>().unwrap());
                let args: [usize; 3] = [
                    args.next().unwrap(),
                    args.next().unwrap(),
                    args.next().unwrap(),
                ];
                match cmd {
                    "addi" => Box::new(move |reg: &mut Register<T>| addi(args, reg)) as Command<T>,
                    "addr" => Box::new(move |reg: &mut Register<T>| addr(args, reg)) as Command<T>,
                    "mulr" => Box::new(move |reg: &mut Register<T>| mulr(args, reg)) as Command<T>,
                    "muli" => Box::new(move |reg: &mut Register<T>| muli(args, reg)) as Command<T>,
                    "banr" => Box::new(move |reg: &mut Register<T>| banr(args, reg)) as Command<T>,
                    "bani" => Box::new(move |reg: &mut Register<T>| bani(args, reg)) as Command<T>,
                    "borr" => Box::new(move |reg: &mut Register<T>| borr(args, reg)) as Command<T>,
                    "bori" => Box::new(move |reg: &mut Register<T>| bori(args, reg)) as Command<T>,
                    "setr" => Box::new(move |reg: &mut Register<T>| setr(args, reg)) as Command<T>,
                    "seti" => Box::new(move |reg: &mut Register<T>| seti(args, reg)) as Command<T>,
                    "gtir" => Box::new(move |reg: &mut Register<T>| gtir(args, reg)) as Command<T>,
                    "gtri" => Box::new(move |reg: &mut Register<T>| gtri(args, reg)) as Command<T>,
                    "gtrr" => Box::new(move |reg: &mut Register<T>| gtrr(args, reg)) as Command<T>,
                    "eqir" => Box::new(move |reg: &mut Register<T>| eqir(args, reg)) as Command<T>,
                    "eqri" => Box::new(move |reg: &mut Register<T>| eqri(args, reg)) as Command<T>,
                    "eqrr" => Box::new(move |reg: &mut Register<T>| eqrr(args, reg)) as Command<T>,
                    _ => panic!("Unrecognized function name!"),
                }
            })
            .collect();
        Ok(Self { ip, commands })
    }
}

fn part1<T>(input: &str, register: &mut [T; 6]) -> Result<usize>
where
    T: Usizeable,
{
    let mut program: Program<T> = input.parse()?;
    while let Some(func) = program
        .commands
        .get_mut(TryInto::<usize>::try_into(register[program.ip]).unwrap())
    {
        func(register);
        register[program.ip] += 1.try_into().unwrap();
    }
    Ok(register[0].try_into().unwrap())
}

fn part2(input: &str, register: &mut [usize; 6]) -> Result<usize> {
    let mut program: Program<usize> = input.parse()?;

    while let Some(func) = program
        .commands
        .get_mut(TryInto::<usize>::try_into(register[program.ip]).unwrap())
    {
        if register[program.ip] == 3
            && register[5] * register[2] != register[4]
            && (register[2] + 1) <= register[4]
        {
            // 3 overridden by 4
            // 4 due to conditions
            register[1] = 0;
            // 5 noop since reg[1] is 0
            // 6 makes it skip the next one
            // 7 skipped by 6
            // 8
            // register[2] += 1;
            // Since 4 and 5 are not changed in this loop and 2 just incrememnts until one of the
            // conditions is false, just jump 2 until it would invalidate one of the conditions
            register[2] = match (
                (
                    register[4].checked_div(register[5]),
                    register[4].checked_rem(register[5]),
                ),
                register[4],
            ) {
                ((Some(div), Some(0)), sub) if div <= sub && div > register[2] => div,
                ((_, _), sub) if sub > register[2] => sub,
                _ => register[2] + 1,
            };
            // 9 reg 1 will remain 0 due to conditions
            // 10 noop since reg 1 is 0
            // 11 Taking into account IP increment
            register[3] = 2 + 1;
            continue;
        }

        func(register);
        register[program.ip] += 1;
    }
    Ok(register[0].try_into().unwrap())
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut reg: [usize; 6] = [0; 6];
    println!("part1: {}", part1(&input, &mut reg)?);
    reg = [0; 6];
    reg[0] = 1;
    println!("part2: {}", part2(&input, &mut reg)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input = "#ip 3
seti 5 0 1
seti 6 0 2
addi 0 1 0";

        let mut program: Program<usize> = input.parse().unwrap();
        assert_eq!(program.ip, 3);
        let mut reg: Register<usize> = [0; 6];
        program.commands.get_mut(0).unwrap()(&mut reg);
        assert_eq!(reg, [0, 5, 0, 0, 0, 0]);
        program.commands.get_mut(0).unwrap()(&mut reg);
    }

    #[test]
    fn test_part1() {
        let input = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

        let mut reg: Register<usize> = [0; 6];
        let result = part1(&input, &mut reg).unwrap();
        assert_eq!(result, 7);
    }
}
