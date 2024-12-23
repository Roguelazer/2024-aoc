use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;

use itertools::Itertools;
use petgraph::graph::{NodeIndex, UnGraph};
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Computer(SmolStr);

impl Computer {
    fn starts_with_t(&self) -> bool {
        self.0.starts_with('t')
    }
}

#[derive(Debug, Clone)]
struct Problem {
    map: UnGraph<Computer, ()>,
    i_to_n: BTreeMap<NodeIndex, Computer>,
}

impl Problem {
    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let mut nodes = BTreeMap::new();
        let mut computers = BTreeSet::new();
        let mut map = UnGraph::new_undirected();

        for line in s.lines() {
            let Some((a, b)) = line.split_once('-') else {
                continue;
            };
            let a = Computer(SmolStr::new(a));
            let b = Computer(SmolStr::new(b));
            computers.insert(a.clone());
            computers.insert(b.clone());
            let an = *nodes.entry(a.clone()).or_insert_with(|| map.add_node(a));
            let bn = *nodes.entry(b.clone()).or_insert_with(|| map.add_node(b));
            map.add_edge(an, bn, ());
        }

        let i_to_n = map
            .node_indices()
            .map(|n| (n, map[n].clone()))
            .collect::<BTreeMap<_, _>>();
        Ok(Problem { map, i_to_n })
    }

    fn find_k_cliques<const K: usize>(&self) -> BTreeSet<[Computer; K]> {
        let mut cliques = BTreeSet::new();
        let mut work: Vec<(_, _)> = self.map.node_indices().map(|n| (n, vec![n])).collect();
        while let Some((vertex, clique_candidate)) = work.pop() {
            if clique_candidate.len() == K {
                let cc = clique_candidate
                    .into_iter()
                    .map(|n| self.i_to_n[&n].clone())
                    .sorted()
                    .collect::<Vec<Computer>>()
                    .try_into()
                    .unwrap();
                cliques.insert(cc);
                continue;
            }
            for neighbor in self.map.neighbors_undirected(vertex) {
                if clique_candidate.contains(&neighbor) {
                    continue;
                }
                if !clique_candidate
                    .iter()
                    .all(|n| self.map.contains_edge(*n, neighbor))
                {
                    continue;
                }
                let mut nc = clique_candidate.clone();
                nc.push(neighbor);
                work.push((neighbor, nc));
            }
        }
        cliques
    }

    fn part1(&self) -> usize {
        let cliques = self.find_k_cliques::<3>();
        cliques
            .into_iter()
            .filter(|c| c.iter().any(|c| c.starts_with_t()))
            .count()
    }

    fn part2(&self) -> String {
        let k = aoclib::petgraph_bron_kerbosch::maximal_cliques(&self.map);
        let v = k
            .iter()
            .max_by_key(|l| l.len())
            .unwrap()
            .into_iter()
            .map(|n| self.i_to_n[&n].clone())
            .map(|c| c.0)
            .sorted()
            .map(|s| s.to_string())
            .join(",");
        v
    }
}

fn main() -> anyhow::Result<()> {
    let p = Problem::read()?;
    println!("part 1: {}", p.part1());
    println!("part 2: {}", p.part2());
    Ok(())
}
