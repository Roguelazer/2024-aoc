use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let mut lhs: Vec<i32> = vec![];
    let mut rhs: Vec<i32> = vec![];
    for line in stdin.lines() {
        let line = line?;
        if line.trim().is_empty() {
            break;
        }
        let mut parts = line.split_whitespace();
        lhs.push(parts.next().unwrap().parse().unwrap());
        rhs.push(parts.next().unwrap().parse().unwrap());
    }
    lhs.sort();
    rhs.sort();
    let part1: u32 = lhs
        .iter()
        .zip(rhs.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();
    println!("Part 1: {}", part1);

    let frequencies = rhs.into_iter().counts();

    let part2: i32 = lhs
        .iter()
        .map(|v| frequencies.get(v).map_or(0, |q| *q as i32) * v)
        .sum();
    println!("part 2: {}", part2);
    Ok(())
}
