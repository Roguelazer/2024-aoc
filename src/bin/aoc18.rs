use aoclib::{DenseGrid, Point};
use clap::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    dimension: usize,
    #[clap(short, long)]
    steps: usize,
}

#[derive(Debug, Clone)]
struct Problem {
    map: DenseGrid<bool>,
    points: Vec<Point>,
}

impl Problem {
    fn parse_points(s: &str) -> IResult<&str, Vec<Point>> {
        map(
            separated_list1(
                newline,
                separated_pair(
                    nom::character::complete::i64,
                    tag(","),
                    nom::character::complete::i64,
                ),
            ),
            |ps| ps.into_iter().map(|(l, r)| Point::new(l, r)).collect(),
        )(s)
    }

    fn read(dimension: usize) -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let md = dimension as i64;
        let map = DenseGrid::new(Point::new(0, 0), Point::new(md, md));
        let (remainder, points) = Self::parse_points(s.trim()).unwrap();
        if !remainder.is_empty() {
            anyhow::bail!("unparsed input: {:?}", remainder);
        }
        Ok(Problem { map, points })
    }

    fn build_graph(
        &self,
    ) -> (
        BTreeMap<Point, NodeIndex>,
        petgraph::graph::DiGraph<Point, ()>,
    ) {
        let mut graph = petgraph::graph::DiGraph::new();
        let mut nodes = BTreeMap::new();
        for (point, value) in self.map.iter() {
            if value {
                continue;
            }
            let node = *nodes.entry(point).or_insert_with(|| graph.add_node(point));
            for neighbor in point.ordinal_neighbors_array() {
                if self.map.get(neighbor) != Some(false) {
                    continue;
                }
                let nnode = *nodes
                    .entry(neighbor)
                    .or_insert_with(|| graph.add_node(neighbor));
                graph.add_edge(node, nnode, ());
            }
        }
        (nodes, graph)
    }

    fn part1(&mut self, steps: usize) {
        for point in self.points.iter().take(steps) {
            self.map.set(*point, true);
        }
        let (mut nodes, mut graph) = self.build_graph();
        let start = *nodes
            .entry(Point::new(0, 0))
            .or_insert_with(|| graph.add_node(Point::new(0, 0)));
        let exit_point = Point::new(self.map.max_x, self.map.max_y);
        let goal = *nodes
            .entry(exit_point)
            .or_insert_with(|| graph.add_node(exit_point));
        let d = petgraph::algo::dijkstra(&graph, start, Some(goal), |_| 1);
        println!("part 1: {}", d.get(&goal).unwrap());
    }

    fn part2(&mut self) {
        // this feels like it's probably Ford-Fulkerson, but whatever
        // just do it the brute-force way.
        let (nodes, graph) = self.build_graph();
        let start = *nodes.get(&Point::new(0, 0)).unwrap();
        let exit_point = Point::new(self.map.max_x, self.map.max_y);
        let goal = *nodes.get(&exit_point).unwrap();
        let mut removed = BTreeSet::new();
        for (i, point) in self.points.iter().enumerate() {
            let node = *nodes.get(point).unwrap();
            removed.insert(node);
            let d = petgraph::algo::dijkstra(&graph, start, Some(goal), |e| {
                if removed.contains(&e.source()) || removed.contains(&e.target()) {
                    f32::INFINITY
                } else {
                    0.0
                }
            });
            match d.get(&goal).cloned() {
                Some(f32::INFINITY) | None => {
                    println!("part 2: {} {}", i, point);
                    return;
                }
                _ => {}
            }
        }
        println!("part 2 failed!");
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let problem = Problem::read(args.dimension)?;
    problem.clone().part1(args.steps);
    problem.clone().part2();
    Ok(())
}
