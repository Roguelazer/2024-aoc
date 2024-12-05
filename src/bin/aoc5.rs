use anyhow::Context;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{pair, separated_pair};
use nom::IResult;
use petgraph::graph::DiGraph;

use std::collections::BTreeMap;
use std::io::Read;

#[derive(Debug, Clone, Copy)]
struct Constraint {
    lhs: u32,
    rhs: u32,
}

impl Constraint {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                nom::character::complete::u32,
                tag("|"),
                nom::character::complete::u32,
            ),
            |(lhs, rhs)| Self { lhs, rhs },
        )(s)
    }
}

#[derive(Debug)]
struct Update {
    pages: Vec<u32>,
    page_indexes: BTreeMap<u32, usize>,
}

impl Update {
    fn new(pages: Vec<u32>) -> Self {
        let page_indexes = pages
            .iter()
            .enumerate()
            .map(|(i, val)| (*val, i))
            .collect::<BTreeMap<_, _>>();
        Self {
            pages,
            page_indexes,
        }
    }

    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_list1(tag(","), nom::character::complete::u32),
            |cs| Self::new(cs),
        )(s)
    }

    fn middle_page(&self) -> u32 {
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

    fn fix(&self, update: &Update) -> anyhow::Result<Update> {
        let mut graph: DiGraph<u32, ()> = DiGraph::new();
        let page_to_index: BTreeMap<u32, _> = update
            .pages
            .iter()
            .map(|page| (*page, graph.add_node(*page)))
            .collect();
        let index_to_page: BTreeMap<_, u32> = page_to_index.iter().map(|(k, v)| (*v, *k)).collect();
        self.constraints
            .iter()
            .filter(|constraint| {
                update.page_indexes.contains_key(&constraint.lhs)
                    && update.page_indexes.contains_key(&constraint.rhs)
            })
            .for_each(|constraint| {
                let left_node = page_to_index.get(&constraint.lhs).unwrap();
                let right_node = page_to_index.get(&constraint.rhs).unwrap();
                graph.add_edge(*left_node, *right_node, ());
            });
        let order = petgraph::algo::toposort(&graph, None)
            .map_err(|e| anyhow::anyhow!("graph had a cycle: {:?}", e))
            .with_context(|| format!("failed to topologically sort graph"))?;
        let pages = order
            .into_iter()
            .map(|idx| *(index_to_page.get(&idx).unwrap()))
            .collect::<Vec<u32>>();
        Ok(Update::new(pages))
    }

    fn part1(&self) -> u32 {
        self.updates
            .iter()
            .filter(|update| self.is_valid(update))
            .map(|update| update.middle_page())
            .sum()
    }

    fn part2(&mut self) -> u32 {
        self.updates
            .iter()
            .filter(|update| !self.is_valid(update))
            .map(|update| self.fix(update).unwrap().middle_page())
            .sum()
    }
}

fn main() -> anyhow::Result<()> {
    let mut project = Project::read()?;
    println!("Part 1: {}", project.part1());
    println!("Part 2: {}", project.part2());
    Ok(())
}
