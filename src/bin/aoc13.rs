use std::io::Read;

use good_lp::{constraint, default_solver, variables, Solution, SolverModel};
use ndarray::prelude::*;
use ndarray_linalg::Solve;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

#[derive(Debug)]
struct Machine {
    ax: i32,
    ay: i32,
    bx: i32,
    by: i32,
    px: i32,
    py: i32,
}

impl Machine {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            tuple((
                terminated(
                    tuple((
                        preceded(tag("Button A: X+"), nom::character::complete::i32),
                        preceded(tag(", Y+"), nom::character::complete::i32),
                    )),
                    tag("\n"),
                ),
                terminated(
                    tuple((
                        preceded(tag("Button B: X+"), nom::character::complete::i32),
                        preceded(tag(", Y+"), nom::character::complete::i32),
                    )),
                    tag("\n"),
                ),
                terminated(
                    tuple((
                        preceded(tag("Prize: X="), nom::character::complete::i32),
                        preceded(tag(", Y="), nom::character::complete::i32),
                    )),
                    multispace1,
                ),
            )),
            |((ax, ay), (bx, by), (px, py))| Self {
                ax,
                ay,
                bx,
                by,
                px,
                py,
            },
        )(s)
    }

    fn part1_cheapest(&self) -> Option<i32> {
        variables! {
            vars:
                0 <= a (integer);
                0 <= b (integer);
        }

        let solution = vars
            .minimise(3 * a + b)
            .using(default_solver)
            .with(constraint!(a * self.ax + b * self.bx == self.px))
            .with(constraint!(a * self.ay + b * self.by == self.py))
            .with(constraint!(a + b <= 200))
            .solve();
        if let Ok(solution) = solution {
            let a = solution.value(a) as i32;
            let b = solution.value(b) as i32;
            Some(3 * a + b)
        } else {
            None
        }
    }

    fn part2_cheapest(&self) -> Option<u64> {
        let ax = self.ax as f64;
        let ay = self.ay as f64;
        let bx = self.bx as f64;
        let by = self.by as f64;
        let px = self.px as f64 + 10_000_000_000_000.0;
        let py = self.py as f64 + 10_000_000_000_000.0;

        let a: Array2<f64> = array![[ax, bx], [ay, by]];
        let b: Array1<f64> = array![px, py];
        let c: Array1<f64> = array![3.0, 1.0];

        let z = a.solve_into(b).unwrap();
        if z.iter().any(|v| (v - v.round()).abs() > 0.001) {
            return None;
        }
        Some(z.dot(&c).round() as u64)
    }
}

#[derive(Debug)]
struct Problem {
    machines: Vec<Machine>,
}

impl Problem {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(many1(Machine::parse), |machines| Problem { machines })(s)
    }

    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let (remainder, obj) = Self::parse(&s).unwrap();
        if !remainder.is_empty() {
            anyhow::bail!("unparsed input: {:?}", remainder);
        }
        Ok(obj)
    }
}

fn main() -> anyhow::Result<()> {
    let problem = Problem::read()?;
    let part1: i32 = problem
        .machines
        .iter()
        .filter_map(|m| m.part1_cheapest())
        .sum();
    println!("part 1: {}", part1);
    let part2: u64 = problem
        .machines
        .iter()
        .filter_map(|m| m.part2_cheapest())
        .sum();
    println!("part 2: {}", part2);
    Ok(())
}
