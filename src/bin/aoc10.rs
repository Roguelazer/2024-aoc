use std::collections::BTreeMap;
use std::io::Read;

use aoclib::{DenseGrid, Point};
use petgraph::algo::floyd_warshall;
use petgraph::graph::{DiGraph, NodeIndex};

#[derive(Debug)]
struct Map {
    grid: DenseGrid<u32>,
    graph: DiGraph<Point, ()>,
    node_indexes: BTreeMap<Point, NodeIndex>,
    starts: Vec<Point>,
    ends: Vec<Point>,
}

impl Map {
    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let grid = DenseGrid::try_from_input(&s, |f| {
            if f == '.' {
                Ok(10000)
            } else {
                f.to_digit(10)
                    .ok_or(anyhow::anyhow!("failed to parse {} as a digit", f))
            }
        })?;
        let mut graph = DiGraph::new();
        let mut node_indexes = BTreeMap::new();
        let mut starts = vec![];
        let mut ends = vec![];
        for (point, value) in grid.iter() {
            let (north, east, south, west) = point.ordinal_neighbors();
            let index = node_indexes
                .entry(point)
                .or_insert_with(|| graph.add_node(point))
                .to_owned();
            if value == 0 {
                starts.push(point);
            }
            if value == 9 {
                ends.push(point);
            }
            for neighbor in &[north, east, south, west] {
                if let Some(neighbor_val) = grid.get(*neighbor) {
                    if neighbor_val == value + 1 {
                        let neighbor_index = node_indexes
                            .entry(*neighbor)
                            .or_insert_with(|| graph.add_node(*neighbor))
                            .to_owned();
                        graph.add_edge(index, neighbor_index, ());
                    }
                }
            }
        }
        Ok(Map {
            grid,
            graph,
            node_indexes,
            starts,
            ends,
        })
    }

    fn part1(&self) -> anyhow::Result<usize> {
        let all_pairs = floyd_warshall(&self.graph, |_| 1)
            .map_err(|_| anyhow::anyhow!("cannot compute F-W"))?;
        let destinations = self
            .ends
            .iter()
            .map(|p| self.node_indexes.get(p).unwrap().to_owned())
            .collect::<Vec<_>>();
        let score = self
            .starts
            .iter()
            .map(|start_point| {
                let index = self.node_indexes.get(&start_point).unwrap();
                let score = destinations
                    .iter()
                    .filter(|d| {
                        if let Some(distance) = all_pairs.get(&(*index, **d)) {
                            *distance < i32::MAX
                        } else {
                            false
                        }
                    })
                    .count();
                score
            })
            .sum();
        Ok(score)
    }

    fn part2(&self) -> anyhow::Result<usize> {
        let reverse_node_indexes: BTreeMap<NodeIndex, Point> =
            self.node_indexes.iter().map(|(p, i)| (*i, *p)).collect();
        let mut paths = DenseGrid::new_with_dimensions_from(&self.grid, None);
        let mut roots: Vec<Point> = self
            .grid
            .iter()
            .filter(|(p, v)| {
                if *v == 9 {
                    true
                } else {
                    let index = self.node_indexes.get(p).unwrap();
                    self.graph.neighbors(*index).count() == 0
                }
            })
            .map(|(p, _)| p)
            .collect();
        for point in &roots {
            if self.grid.get(*point) == Some(9) {
                paths.set(*point, Some(1));
            } else {
                paths.set(*point, Some(0));
            }
        }
        while let Some(item) = roots.pop() {
            let index = self.node_indexes.get(&item).unwrap().to_owned();
            if paths.get(item).unwrap().is_none() {
                // look at all my inbound neighbors
                let mut any_unknowns = false;
                let mut sums = 0;
                for neighbor in self.graph.neighbors(index) {
                    let neighbor_point = reverse_node_indexes.get(&neighbor).unwrap().to_owned();
                    if let Some(value) = paths.get(neighbor_point).unwrap().to_owned() {
                        sums += value;
                    } else {
                        any_unknowns = true;
                    }
                }
                if !any_unknowns {
                    paths.set(item, Some(sums));
                }
            }
            for neighbor in self
                .graph
                .neighbors_directed(index, petgraph::Direction::Incoming)
            {
                let point = reverse_node_indexes.get(&neighbor).unwrap().to_owned();
                roots.push(point);
            }
        }
        let val = self
            .starts
            .iter()
            .map(|start_point| {
                if let Some(val) = paths.get(*start_point).unwrap() {
                    val
                } else {
                    0
                }
            })
            .sum();
        Ok(val)
    }
}

fn main() -> anyhow::Result<()> {
    let map = Map::read()?;
    println!("part 1: {}", map.part1()?);
    println!("part 2: {}", map.part2()?);
    Ok(())
}
