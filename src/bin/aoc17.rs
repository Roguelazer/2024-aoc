use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Operation {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Operation {
    fn from_int(val: u8) -> Self {
        match val {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("unknown input {}", val),
        }
    }

    fn apply(&self, operand: i64, machine: &mut Machine) -> bool {
        match self {
            Self::Adv => {
                let numerator = machine.a;
                let divisor = 1 << machine.get_combo(operand);
                machine.a = numerator / divisor;
            }
            Self::Bxl => {
                machine.b = machine.b ^ operand;
            }
            Self::Bst => {
                machine.b = machine.get_combo(operand) & 0x07;
            }
            Self::Jnz => {
                if machine.a != 0 {
                    machine.ip = operand as usize;
                    return false;
                }
            }
            Self::Bxc => {
                machine.b = machine.b ^ machine.c;
            }
            Self::Out => machine.emit((machine.get_combo(operand) & 0x07) as u8),
            Self::Bdv => {
                let numerator = machine.a;
                let divisor = 1 << machine.get_combo(operand);
                machine.b = numerator / divisor;
            }
            Self::Cdv => {
                let numerator = machine.a;
                let divisor = 1 << machine.get_combo(operand);
                machine.c = numerator / divisor;
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
struct Machine {
    input: Vec<u8>,
    ip: usize,
    a: i64,
    b: i64,
    c: i64,
    initial_a: i64,
    initial_b: i64,
    initial_c: i64,
    output: Vec<u8>,
}

impl Machine {
    fn new(a: i64, b: i64, c: i64, input: Vec<u8>) -> Self {
        Machine {
            a,
            b,
            c,
            input,
            ip: 0,
            initial_a: a,
            initial_b: b,
            initial_c: c,
            output: vec![],
        }
    }
    fn get_combo(&self, operand: i64) -> i64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("got combo opeand 7"),
            _ => panic!("malformed input {}", operand),
        }
    }

    fn emit(&mut self, val: u8) {
        self.output.push(val)
    }

    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            tuple((
                terminated(
                    preceded(tag("Register A: "), nom::character::complete::i64),
                    newline,
                ),
                terminated(
                    preceded(tag("Register B: "), nom::character::complete::i64),
                    newline,
                ),
                terminated(
                    preceded(tag("Register C: "), nom::character::complete::i64),
                    tuple((newline, newline)),
                ),
                preceded(
                    tag("Program: "),
                    separated_list1(tag(","), nom::character::complete::u8),
                ),
            )),
            |(a, b, c, input)| Machine::new(a, b, c, input),
        )(s)
    }

    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let (remainder, obj) = Self::parse(s.trim()).unwrap();
        if !remainder.is_empty() {
            anyhow::bail!("unparsed input: {:?}", remainder);
        }
        Ok(obj)
    }

    fn step(&mut self) {
        let p1 = self.input[self.ip];
        let p2 = self.input[self.ip + 1];
        let operation = Operation::from_int(p1);
        if operation.apply(p2 as i64, self) {
            self.ip += 2;
        }
    }

    fn run(&mut self) {
        while self.ip < self.input.len() {
            self.step();
        }
    }

    fn reset(&mut self) {
        self.output.clear();
        self.ip = 0;
        self.a = self.initial_a;
        self.b = self.initial_b;
        self.c = self.initial_c;
    }
}

fn part1(mut machine: Machine) {
    machine.run();
    println!(
        "part 1: {}",
        machine
            .output
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
}

fn part2_rec(machine: &mut Machine, target: &[u8]) -> i64 {
    let mut a = if target.len() == 1 {
        0
    } else {
        8 * part2_rec(machine, &target[1..])
    };
    loop {
        machine.a = a;
        machine.run();
        if machine.output == target {
            println!("for target {:?}, found {}", target, a);
            machine.reset();
            return a;
        }
        machine.reset();
        a += 1;
    }
}

fn part2(mut machine: Machine) {
    let input = machine.input.clone();
    let res = part2_rec(&mut machine, &input);
    machine.a = res;
    machine.run();
    if machine.output != input {
        panic!("uh oh");
    }
    println!("part 2: {}", res);
}

fn main() -> anyhow::Result<()> {
    let problem = Machine::read()?;
    part1(problem.clone());
    part2(problem.clone());
    Ok(())
}
