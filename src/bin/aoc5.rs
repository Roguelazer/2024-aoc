use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{pair, separated_pair};
use nom::IResult;

use std::collections::BTreeMap;
use std::io::Read;

#[derive(Debug, Clone, Copy)]
struct Constraint {
    lhs: i32,
    rhs: i32,
}

impl Constraint {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                nom::character::complete::i32,
                tag("|"),
                nom::character::complete::i32,
            ),
            |(lhs, rhs)| Self { lhs, rhs },
        )(s)
    }
}

#[derive(Debug)]
struct Update {
    pages: Vec<i32>,
    page_indexes: BTreeMap<i32, usize>,
}

impl Update {
    fn new(pages: Vec<i32>) -> Self {
        let page_indexes = pages.iter().enumerate().map(|(i, val)| (*val, i)).collect();
        Self {
            pages,
            page_indexes,
        }
    }

    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_list1(tag(","), nom::character::complete::i32),
            |cs| Self::new(cs),
        )(s)
    }

    fn middle_page(&self) -> i32 {
        self.pages[self.pages.len() / 2]
    }
}

#[derive(Debug)]
struct Project {
    constraints: Vec<Constraint>,
    updates: Vec<Update>,
}

impl Project {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                separated_list1(nom::character::complete::newline, Constraint::parse),
                pair(newline, newline),
                separated_list1(nom::character::complete::newline, Update::parse),
            ),
            |(constraints, updates)| Self {
                constraints,
                updates,
            },
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

    fn is_valid(&self, update: &Update) -> bool {
        self.constraints.iter().all(|constraint| {
            let before = constraint.lhs;
            let after = constraint.rhs;
            let before_index = update.page_indexes.get(&before);
            let after_index = update.page_indexes.get(&after);
            match (before_index, after_index) {
                (Some(a), Some(b)) => a < b,
                _ => true,
            }
        })
    }

    fn part1(&self) -> i32 {
        self.updates
            .iter()
            .filter(|update| self.is_valid(update))
            .map(|update| update.middle_page())
            .sum()
    }
}

fn main() -> anyhow::Result<()> {
    let project = Project::read()?;
    println!("Part 1: {}", project.part1());
    Ok(())
}
