use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

/// Find the largest 14-digit number with no zero digits accepted by
/// the program.
pub fn part1(lines: &[String]) -> i64 {
    let input = (1..=9).rev().collect_vec();
    solve(&parse(lines), &input).unwrap()
}

/// Find the smallest 14-digit number with no zero digits accepted by
/// the program.
pub fn part2(lines: &[String]) -> i64 {
    let input = (1..=9).collect_vec();
    solve(&parse(lines), &input).unwrap()
}

fn parse(lines: &[String]) -> Vec<Op> {
    lines.iter().map(|line| line.parse().unwrap()).collect_vec()
}

#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
/// Registers in the ALU.
pub enum Reg {
    /// W register
    W = 0,
    /// X register
    X = 1,
    /// Y register
    Y = 2,
    /// Z register, used to store the result.
    Z = 3,
}

impl Reg {
    #[inline(always)]
    /// Cast register to its index.
    pub fn idx(&self) -> usize {
        *self as usize
    }
}

impl FromStr for Reg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().next().ok_or("missing register operand")?;
        match c {
            'w' => Ok(Reg::W),
            'x' => Ok(Reg::X),
            'y' => Ok(Reg::Y),
            'z' => Ok(Reg::Z),
            _ => Err(format!("unrecognized register {}", c)),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
/// Right-hand-operand to an instruction.
pub enum Val {
    /// Read a value from the register.
    Reg(Reg),
    /// Use the immediate value.
    Imm(i64),
}

impl Val {
    /// Read the value, potentially from a register.
    #[inline]
    pub fn fetch(&self, registers: &[i64; 4]) -> i64 {
        match self {
            Val::Reg(reg) => registers[reg.idx()],
            Val::Imm(imm) => *imm,
        }
    }
}

impl FromStr for Val {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(imm) => Ok(Val::Imm(imm)),
            Err(_) => {
                let reg = s.parse()?;
                Ok(Val::Reg(reg))
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
/// ALU instructions.
pub enum Op {
    /// Read an integer into the register.
    Inp(Reg),
    /// `reg = reg + val`
    Add(Reg, Val),
    /// `reg = reg * val`
    Mul(Reg, Val),
    /// `reg = reg / val`
    Div(Reg, Val),
    /// `reg = reg % val`
    Mod(Reg, Val),
    /// `reg = reg == val`
    Eql(Reg, Val),
}

impl FromStr for Op {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts = line.split_whitespace().collect_vec();
        match &parts[..] {
            ["inp", acc] => {
                let acc: Reg = acc.parse()?;
                Ok(Op::Inp(acc))
            }
            [op, acc, val] => {
                let acc: Reg = acc.parse()?;
                let val: Val = val.parse()?;
                match *op {
                    "add" => Ok(Op::Add(acc, val)),
                    "mul" => Ok(Op::Mul(acc, val)),
                    "div" => Ok(Op::Div(acc, val)),
                    "mod" => Ok(Op::Mod(acc, val)),
                    "eql" => Ok(Op::Eql(acc, val)),
                    _ => Err(format!("unrecognized binary operation {}", op)),
                }
            }
            _ => Err(format!("unrecognized instruction {}", line)),
        }
    }
}

#[derive(Debug)]
/// Does the ALU need input or has it halted?
pub enum Progress {
    /// The ALU needs more input.
    More,
    /// The ALU has halted.
    Done,
}

/// Run the ALU starting until it halts or requests more input. Mutates
/// the argument program counter and registers.
fn step(program: &[Op], pc: &mut usize, registers: &mut [i64; 4], input: i64) -> Progress {
    let mut input = Some(input);
    while let Some(op) = program.get(*pc) {
        match op {
            Op::Inp(acc) => {
                if let Some(input) = input.take() {
                    registers[acc.idx()] = input;
                } else {
                    return Progress::More;
                }
            }
            Op::Add(acc, val) => registers[acc.idx()] += val.fetch(registers),
            Op::Mul(acc, val) => registers[acc.idx()] *= val.fetch(registers),
            Op::Div(acc, val) => registers[acc.idx()] /= val.fetch(registers),
            Op::Mod(acc, val) => registers[acc.idx()] %= val.fetch(registers),
            Op::Eql(acc, val) => {
                registers[acc.idx()] = (registers[acc.idx()] == val.fetch(registers)) as i64
            }
        }
        *pc += 1;
    }
    Progress::Done
}

/// Find the first input (as a decimal number) that causes the program
/// to store zero in the Z register.
fn solve(program: &[Op], input: &[i64]) -> Option<i64> {
    let mut seen: HashMap<(usize, i64), Option<i64>> = HashMap::new();
    search(0, program, 0, [0; 4], input, &mut seen)
}

/// Recursively generate possible inputs until one causes the program
/// to store zero in the Z register.
fn search(
    acc: i64,
    program: &[Op],
    pc: usize,
    registers: [i64; 4],
    input: &[i64],
    seen: &mut HashMap<(usize, i64), Option<i64>>,
) -> Option<i64> {
    let z = Reg::Z.idx();
    if let Some(&result) = seen.get(&(pc, registers[z])) {
        return result;
    }

    for &i in input.iter() {
        let mut pc = pc;
        let mut registers = registers;

        match step(program, &mut pc, &mut registers, i) {
            Progress::Done => {
                if registers[z] == 0 {
                    let result = Some(acc * 10 + i);
                    seen.insert((pc, registers[z]), result);
                    return result;
                }
            }
            Progress::More => {
                if let result @ Some(_) = search(acc * 10 + i, program, pc, registers, input, seen)
                {
                    seen.insert((pc, registers[z]), result);
                    return result;
                }
            }
        }
    }

    seen.insert((pc, registers[z]), None);
    None
}

// Warning, even in release mode, it takes about 10 seconds to run
// both tests. In debug mode, part 2 takes 3 seconds, but part 1
// takes 2 minutes :(
check!(part 1 = 53999995829399, part 2 = 11721151118175);
