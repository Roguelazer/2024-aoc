use std::io::Read;

use memoize::memoize;

#[derive(Debug)]
struct Problem {
    stones: Vec<u64>,
}

#[memoize(Capacity: 10000000)]
fn sim_rec(stone_value: u64, remaining: usize) -> usize {
    if remaining == 0 {
        1
    } else if stone_value == 0 {
        sim_rec(1, remaining - 1)
    } else {
        let digits = stone_value.ilog(10) + 1;
        if digits % 2 == 0 {
            let multiplier = 10u64.pow(digits / 2);
            let top = stone_value / 10u64.pow(digits / 2);
            let bottom = stone_value - top * multiplier;
            sim_rec(top, remaining - 1) + sim_rec(bottom, remaining - 1)
        } else {
            sim_rec(stone_value * 2024, remaining - 1)
        }
    }
}

impl Problem {
    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let stones = s
            .split_whitespace()
            .map(|w| w.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()?;
        Ok(Problem { stones })
    }

    fn simulate(&self, ticks: usize) -> usize {
        let mut count = 0;
        for stone in &self.stones {
            count += sim_rec(*stone, ticks);
        }
        count
    }
}

fn main() -> anyhow::Result<()> {
    let problem = Problem::read()?;
    println!("part 1: {}", problem.simulate(25));
    println!("part 2: {}", problem.simulate(75));
    Ok(())
}
