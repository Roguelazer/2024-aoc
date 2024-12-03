use regex::Regex;
use std::io::Read;

#[derive(Debug, Copy, Clone)]
enum Action {
    Do,
    Dont,
    Mul(i64, i64),
}

fn main() -> anyhow::Result<()> {
    let mut stdin = std::io::stdin().lock();
    let mut s = String::new();
    stdin.read_to_string(&mut s)?;
    let part1_re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;
    let part1: i64 = part1_re
        .captures_iter(&s)
        .map(|s| s.extract())
        .map(|(_, [lhs_s, rhs_s])| {
            let lhs: i64 = lhs_s.parse::<i64>().unwrap();
            let rhs: i64 = rhs_s.parse::<i64>().unwrap();
            lhs * rhs
        })
        .sum();
    println!("part 1: {}", part1);
    let part2_re = Regex::new(
        r"(?x:
        (mul\(([0-9]{1,3}),([0-9]{1,3})\)) |
        (do\(\)) |
        (don't\(\))
    )",
    )?;
    let part2: i64 = part2_re
        .captures_iter(&s)
        .map(|s| {
            let entire = s.get(0).unwrap().as_str();
            if entire.starts_with("don't") {
                Action::Dont
            } else if entire.starts_with("do") {
                Action::Do
            } else {
                let lhs = s.get(2).unwrap().as_str().parse().unwrap();
                let rhs = s.get(3).unwrap().as_str().parse().unwrap();
                Action::Mul(lhs, rhs)
            }
        })
        .fold((true, 0), |(action, sum), elem| match elem {
            Action::Do => (true, sum),
            Action::Dont => (false, sum),
            Action::Mul(x, y) => match action {
                true => (true, sum + x * y),
                false => (false, sum),
            },
        })
        .1;
    println!("part 2: {}", part2);
    Ok(())
}
