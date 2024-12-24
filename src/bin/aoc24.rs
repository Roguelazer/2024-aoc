use std::collections::BTreeMap;
use std::io::Read;

use itertools::Itertools;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use smol_str::SmolStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, multispace1, newline};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(alt((tag("AND"), tag("OR"), tag("XOR"))), |s| match s {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => unreachable!(),
        })(s)
    }

    fn apply(&self, inputs: &[bool]) -> bool {
        match self {
            Op::And => inputs
                .into_iter()
                .cloned()
                .reduce(|acc, v| acc & v)
                .unwrap_or(false),
            Op::Or => inputs
                .into_iter()
                .cloned()
                .reduce(|acc, v| acc | v)
                .unwrap_or(false),
            Op::Xor => inputs
                .into_iter()
                .cloned()
                .reduce(|acc, v| acc ^ v)
                .unwrap_or(false),
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    graph: DiGraph<SmolStr, ()>,
    initial_value: BTreeMap<SmolStr, bool>,
    ops: BTreeMap<SmolStr, Op>,
}

impl Problem {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                separated_list1(
                    newline,
                    separated_pair(alphanumeric1, tag(": "), nom::character::complete::u8),
                ),
                multispace1,
                separated_list1(
                    newline,
                    tuple((
                        alphanumeric1,
                        multispace1,
                        Op::parse,
                        multispace1,
                        alphanumeric1,
                        tag(" -> "),
                        alphanumeric1,
                    )),
                ),
            ),
            |(initials, definitions)| {
                let mut initial_value = BTreeMap::new();
                for (name, initial_u8) in initials {
                    let s = SmolStr::new(name);
                    initial_value.insert(s, initial_u8 == 1);
                }
                let mut graph = DiGraph::new();
                let mut nodes = BTreeMap::new();
                let mut ops = BTreeMap::new();
                for (lhs, _, op, _, rhs, _, name) in definitions {
                    let name = SmolStr::new(name);
                    let lhs = SmolStr::new(lhs);
                    let rhs = SmolStr::new(rhs);
                    ops.insert(name.clone(), op);
                    let name_node = *nodes
                        .entry(name.clone())
                        .or_insert_with(|| graph.add_node(name.clone()));
                    let lhs_node = *nodes
                        .entry(lhs.clone())
                        .or_insert_with(|| graph.add_node(lhs.clone()));
                    let rhs_node = *nodes
                        .entry(rhs.clone())
                        .or_insert_with(|| graph.add_node(rhs.clone()));
                    graph.add_edge(lhs_node, name_node, ());
                    graph.add_edge(rhs_node, name_node, ());
                }
                Problem {
                    graph,
                    initial_value,
                    ops,
                }
            },
        )(s)
    }

    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let (remainder, p) = Self::parse(s.trim()).unwrap();
        if !remainder.is_empty() {
            anyhow::bail!("unparsed input: {:?}", remainder);
        }
        Ok(p)
    }

    fn simulate(
        &self,
        value: &mut BTreeMap<SmolStr, bool>,
        idx_to_node: &BTreeMap<NodeIndex, SmolStr>,
    ) {
        for ni in petgraph::algo::toposort(&self.graph, None).unwrap() {
            let node = idx_to_node[&ni].clone();
            if !value.contains_key(&node) {
                let inputs = self
                    .graph
                    .edges_directed(ni, petgraph::Direction::Incoming)
                    .map(|e| *value.get(&idx_to_node[&e.source()]).unwrap())
                    .collect::<Vec<_>>();
                let op = self.ops[&node];
                let computed = op.apply(inputs.as_slice());
                value.insert(node.clone(), computed);
            }
        }
    }

    fn part1(&self) -> usize {
        let mut value = self.initial_value.clone();
        let idx_to_node = self
            .graph
            .node_indices()
            .map(|n| (n, self.graph[n].clone()))
            .collect::<BTreeMap<_, _>>();
        self.simulate(&mut value, &idx_to_node);
        values_to_usize(&value)
    }

    fn part2(&self) -> String {
        /*
        let noop_fn = Box::new(|_, _| "".to_string());
        let node_label_fn = Box::new(|_, (n, s): (NodeIndex, &SmolStr)| {
            if let Some(op) = self.ops.get(s) {
                format!("label=\"{}\\n{:?}\"", s, op)
            } else {
                format!("label=\"{}\"", s)
            }
        });
        format!(
            "{:?}",
            petgraph::dot::Dot::with_attr_getters(
                &self.graph,
                &[petgraph::dot::Config::EdgeNoLabel],
                &noop_fn,
                &node_label_fn,
            )
        )
        */
        let idx_to_node = self
            .graph
            .node_indices()
            .map(|n| (n, self.graph[n].clone()))
            .collect::<BTreeMap<_, _>>();
        for exp in 0..=43 {
            for addend in [0, 1] {
                let x = 1 << exp + addend;
                let y = 1 << exp + addend;
                let expected = x + y;
                let mut values = BTreeMap::new();
                for (i, bit) in usize_to_bits(x).into_iter().enumerate() {
                    values.insert(format!("x{:02}", i).into(), bit);
                }
                for (i, bit) in usize_to_bits(y).into_iter().enumerate() {
                    values.insert(format!("y{:02}", i).into(), bit);
                }
                self.simulate(&mut values, &idx_to_node);
                let got = values_to_usize(&values);
                if got != expected {
                    println!(
                        "error at {} : {} + {} = {} (expected {})",
                        exp, x, y, got, expected
                    );
                    println!("{:?}", values);
                }
            }
        }
        "(solved via inspection)".to_string()
    }
}

fn usize_to_bits(mut val: usize) -> Vec<bool> {
    let mut v = vec![];
    for _ in 0..64 {
        if val & 0x01 == 1 {
            v.push(true)
        } else {
            v.push(false)
        }
        val = val >> 1;
    }
    v
}

fn values_to_usize(values: &BTreeMap<SmolStr, bool>) -> usize {
    values
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted()
        .rev()
        .fold(0, |acc, (_, v)| if *v { (acc << 1) + 1 } else { acc << 1 })
}

fn main() -> anyhow::Result<()> {
    let p = Problem::read()?;
    println!("part 1: {}", p.part1());
    println!("part 2: {}", p.part2());
    Ok(())
}
