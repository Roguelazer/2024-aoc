use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace1, newline};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::BTreeMap;
use std::io::Read;

#[derive(Debug)]
struct Design {
    stripes: String,
}

impl Design {
    fn ways_satisfiable(&self, patterns: &[String]) -> usize {
        let mut cache = BTreeMap::new();
        let mut work: Vec<(&str, Vec<String>)> =
            vec![(self.stripes.as_str(), vec![self.stripes.to_owned()])];
        while let Some((task, tails)) = work.pop() {
            if task.len() == 0 {
                for item in &tails {
                    *cache.entry(item.to_owned()).or_insert(0) += 1;
                }
                continue;
            }
            match cache.get(task) {
                Some(0) => continue,
                Some(v) => {
                    let v = *v;
                    for item in &tails {
                        if item != task {
                            *cache.entry(item.to_owned()).or_insert(0) += v;
                        }
                    }
                    continue;
                }
                None => {}
            }
            let mut possible = false;
            for pattern in patterns {
                if let Some(rest) = task.strip_prefix(pattern.as_str()) {
                    if cache.get(rest) == Some(&0) {
                        continue;
                    }
                    assert!(rest.len() < task.len());
                    let mut next_tails = tails.clone();
                    next_tails.push(rest.to_owned());
                    work.push((rest, next_tails));
                    possible = true;
                }
            }
            if !possible {
                cache.insert(task.to_owned(), 0);
            }
        }
        cache.get(&self.stripes).map(|c| *c).unwrap_or(0)
    }
}

#[derive(Debug)]
struct Problem {
    patterns: Vec<String>,
    designs: Vec<Design>,
}

impl Problem {
    fn parse_problem(s: &str) -> IResult<&str, Self> {
        nom::combinator::map(
            separated_pair(
                separated_list1(tag(", "), alpha1),
                multispace1,
                separated_list1(newline, alpha1),
            ),
            |(patterns, designs)| {
                let mut patterns: Vec<String> =
                    patterns.into_iter().map(|s: &str| s.to_owned()).collect();
                patterns.sort();
                Problem {
                    patterns,
                    designs: designs
                        .into_iter()
                        .map(|s: &str| Design {
                            stripes: s.to_owned(),
                        })
                        .collect(),
                }
            },
        )(s)
    }

    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let (remainder, p) = Self::parse_problem(s.trim()).unwrap();
        if !remainder.is_empty() {
            anyhow::bail!("unparsed input: {:?}", remainder);
        }
        Ok(p)
    }
}

fn part1(problem: &Problem) {
    println!(
        "part 1: {}",
        problem
            .designs
            .iter()
            .filter(|s| s.ways_satisfiable(&problem.patterns) > 0)
            .count()
    );
}

fn part2(problem: &Problem) {
    println!(
        "part 2: {}",
        problem
            .designs
            .iter()
            .map(|s| s.ways_satisfiable(&problem.patterns))
            .sum::<usize>()
    );
}

fn main() -> anyhow::Result<()> {
    let problem = Problem::read()?;
    part1(&problem);
    part2(&problem);
    Ok(())
}
