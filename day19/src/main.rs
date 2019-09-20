// --- Day 19: Go With The Flow ---
// With the Elves well on their way constructing the North Pole base, you turn your attention back to understanding the inner workings of programming the device.
//
// You can't help but notice that the device's opcodes don't contain any flow control like jump instructions. The device's manual goes on to explain:
//
// "In programs where flow control is required, the instruction pointer can be bound to a register so that it can be manipulated directly. This way, setr/seti can function as absolute jumps, addr/addi can function as relative jumps, and other opcodes can cause truly fascinating effects."
//
// This mechanism is achieved through a declaration like #ip 1, which would modify register 1 so that accesses to it let the program indirectly access the instruction pointer itself. To compensate for this kind of binding, there are now six registers (numbered 0 through 5); the five not bound to the instruction pointer behave as normal. Otherwise, the same rules apply as the last time you worked with this device.
//
// When the instruction pointer is bound to a register, its value is written to that register just before each instruction is executed, and the value of that register is written back to the instruction pointer immediately after each instruction finishes execution. Afterward, move to the next instruction by adding one to the instruction pointer, even if the value in the instruction pointer was just updated by an instruction. (Because of this, instructions must effectively set the instruction pointer to the instruction before the one they want executed next.)
//
// The instruction pointer is 0 during the first instruction, 1 during the second, and so on. If the instruction pointer ever causes the device to attempt to load an instruction outside the instructions defined in the program, the program instead immediately halts. The instruction pointer starts at 0.
//
// It turns out that this new information is already proving useful: the CPU in the device is not very powerful, and a background process is occupying most of its time. You dump the background process' declarations and instructions to a file (your puzzle input), making sure to use the names of the opcodes rather than the numbers.
//
// For example, suppose you have the following program:
//
// #ip 0
// seti 5 0 1
// seti 6 0 2
// addi 0 1 0
// addr 1 2 3
// setr 1 0 0
// seti 8 0 4
// seti 9 0 5
//
// When executed, the following instructions are executed. Each line contains the value of the instruction pointer at the time the instruction started, the values of the six registers before executing the instructions (in square brackets), the instruction itself, and the values of the six registers after executing the instruction (also in square brackets).
//
// ip=0 [0, 0, 0, 0, 0, 0] seti 5 0 1 [0, 5, 0, 0, 0, 0]
// ip=1 [1, 5, 0, 0, 0, 0] seti 6 0 2 [1, 5, 6, 0, 0, 0]
// ip=2 [2, 5, 6, 0, 0, 0] addi 0 1 0 [3, 5, 6, 0, 0, 0]
// ip=4 [4, 5, 6, 0, 0, 0] setr 1 0 0 [5, 5, 6, 0, 0, 0]
// ip=6 [6, 5, 6, 0, 0, 0] seti 9 0 5 [6, 5, 6, 0, 0, 9]
//
// In detail, when running this program, the following events occur:
//
//     The first line (#ip 0) indicates that the instruction pointer should be bound to register 0 in this program. This is not an instruction, and so the value of the instruction pointer does not change during the processing of this line.
//     The instruction pointer contains 0, and so the first instruction is executed (seti 5 0 1). It updates register 0 to the current instruction pointer value (0), sets register 1 to 5, sets the instruction pointer to the value of register 0 (which has no effect, as the instruction did not modify register 0), and then adds one to the instruction pointer.
//     The instruction pointer contains 1, and so the second instruction, seti 6 0 2, is executed. This is very similar to the instruction before it: 6 is stored in register 2, and the instruction pointer is left with the value 2.
//     The instruction pointer is 2, which points at the instruction addi 0 1 0. This is like a relative jump: the value of the instruction pointer, 2, is loaded into register 0. Then, addi finds the result of adding the value in register 0 and the value 1, storing the result, 3, back in register 0. Register 0 is then copied back to the instruction pointer, which will cause it to end up 1 larger than it would have otherwise and skip the next instruction (addr 1 2 3) entirely. Finally, 1 is added to the instruction pointer.
//     The instruction pointer is 4, so the instruction setr 1 0 0 is run. This is like an absolute jump: it copies the value contained in register 1, 5, into register 0, which causes it to end up in the instruction pointer. The instruction pointer is then incremented, leaving it at 6.
//     The instruction pointer is 6, so the instruction seti 9 0 5 stores 9 into register 5. The instruction pointer is incremented, causing it to point outside the program, and so the program ends.
//
// What value is left in register 0 when the background process halts?
use std::convert::{TryFrom, TryInto};
use std::io::{Error, ErrorKind::InvalidInput};
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

trait Usizeable:
    TryInto<usize, Error = <Self as Usizeable>::Error>
    + TryFrom<usize, Error = <Self as Usizeable>::Error>
    + std::ops::Add<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::BitAnd<Output = Self>
    + std::ops::BitOr<Output = Self>
    + PartialOrd
    + PartialEq
    + Copy
    + std::fmt::Debug
where
    <Self as Usizeable>::Error: std::fmt::Debug,
{
    type Error;
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

fn part1(input: &str, register: &mut [usize; 6]) -> Result<usize> {
    let mut program: Program<usize> = input.parse()?;
    while let Some(func) = program
        .commands
        .get_mut(TryInto::<usize>::try_into(register[program.ip]).unwrap())
    {
        func(register);
        register[program.ip] += 1;
    }
    Ok(register[0].try_into().unwrap())
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    println!("part1: {}", part1(&input, &mut [0; 6])?);
    println!("part2: {}", part1(&input, &mut [1, 0, 0, 0, 0, 0])?);
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
