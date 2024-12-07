use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

use itertools::Itertools;
use std::io::Read;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Multiply => lhs * rhs,
            Self::Concatenate => {
                let shift: u32 = (rhs as f64).log10().floor() as u32;
                let multiplier = 10i64.pow(shift + 1);
                lhs * multiplier + rhs
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Equation {
    value: i64,
    inputs: Vec<i64>,
}

impl Equation {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                nom::character::complete::i64,
                tag(": "),
                separated_list1(tag(" "), nom::character::complete::i64),
            ),
            |(value, inputs)| Equation { value, inputs },
        )(s)
    }

    fn evaluate(&self, operators: &[&Operator]) -> i64 {
        let mut iiter = self.inputs.iter();
        let mut value = iiter.next().map(|i| *i).unwrap_or(0);
        for operator in operators {
            let rhs = *(iiter.next().unwrap());
            value = operator.apply(value, rhs)
        }
        value
    }

    fn is_satisfiable(&self, part2: bool) -> bool {
        let iter = if part2 {
            [Operator::Add, Operator::Multiply, Operator::Concatenate].as_slice()
        } else {
            [Operator::Add, Operator::Multiply].as_slice()
        };
        itertools::repeat_n(iter, self.inputs.len() - 1)
            .multi_cartesian_product()
            .any(|permutation| {
                let found = self.evaluate(permutation.as_slice());
                found == self.value
            })
    }
}

#[derive(Debug)]
struct Problem {
    equations: Vec<Equation>,
}

impl Problem {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(separated_list1(newline, Equation::parse), |equations| {
            Problem { equations }
        })(s)
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
}

fn calibration(problem: &Problem, part2: bool) -> i64 {
    problem
        .equations
        .iter()
        .filter(|e| e.is_satisfiable(part2))
        .map(|e| e.value)
        .sum()
}

fn main() -> anyhow::Result<()> {
    let problem = Problem::read()?;
    println!("part 1: {}", calibration(&problem, false));
    println!("part 2: {}", calibration(&problem, true));
    Ok(())
}
