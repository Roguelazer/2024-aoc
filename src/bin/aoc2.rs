#[derive(Debug, Clone, Copy)]
enum Dir {
    None,
    Increasing,
    Decreasing,
}

fn safe_transition(current: i32, last: i32, direction: Dir) -> (bool, Dir) {
    let mut new_direction = direction;
    if current == last {
        return (false, direction);
    }
    if current.abs_diff(last) > 3 {
        return (false, direction);
    }
    match (direction, current >= last, current <= last) {
        (Dir::Increasing, true, _) => {}
        (Dir::Decreasing, _, true) => {}
        (Dir::None, inc, dec) => {
            new_direction = if inc {
                Dir::Increasing
            } else if dec {
                Dir::Decreasing
            } else {
                Dir::None
            }
        }
        _ => return (false, direction),
    }
    (true, new_direction)
}

fn is_safe<I: Iterator<Item = i32>>(report: I) -> bool {
    let mut dir = Dir::None;
    let mut last: Option<i32> = None;

    for i in report {
        if let Some(last) = last {
            let (is_safe, new_dir) = safe_transition(i, last, dir);
            if !is_safe {
                return false;
            }
            dir = new_dir;
        }
        last = Some(i);
    }
    true
}

fn is_safe_part1(report: &[i32]) -> bool {
    is_safe(report.iter().map(|i| *i))
}

fn is_safe_part2(report: &[i32]) -> bool {
    if is_safe_part1(report) {
        return true;
    }
    for omitted_index in 0..report.len() {
        let before = &report[0..omitted_index];
        let after = &report[omitted_index + 1..];
        if is_safe(before.iter().chain(after.iter()).map(|i| *i)) {
            return true;
        }
    }
    false
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let reports: Vec<Vec<i32>> = stdin
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let part1 = reports.iter().filter(|f| is_safe_part1(f)).count();
    println!("Part 1: {}", part1);
    let part2 = reports.iter().filter(|f| is_safe_part2(f)).count();
    println!("Part 2: {}", part2);
    Ok(())
}
