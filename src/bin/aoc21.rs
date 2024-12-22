use std::collections::BTreeMap;
use std::io::Read;
use std::sync::OnceLock;

use aoclib::petgraph_bellman_ford_multi::bellman_ford_multi_predecessors;
use itertools::Itertools;
use memoize::memoize;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::NodeIndexable;

fn find_all_shortest_paths<N, E>(graph: &DiGraph<N, E>) -> BTreeMap<(N, N), Vec<Vec<E>>>
where
    N: Ord + PartialEq + Eq + Clone + Copy + std::fmt::Debug,
    E: Ord + PartialEq + Eq + Clone + Copy + std::fmt::Debug,
{
    let node_to_index: BTreeMap<N, NodeIndex> =
        graph.node_indices().map(|n| (graph[n], n)).collect();
    let nodes: Vec<N> = node_to_index.keys().cloned().collect();
    let simplified_graph = graph.map(|_, n| *n, |_, _| 1.0);
    let snode_to_index: BTreeMap<N, NodeIndex> = simplified_graph
        .node_indices()
        .map(|n| (simplified_graph[n], n))
        .collect();

    let mut paths = BTreeMap::new();
    for current in nodes.iter() {
        let bpf =
            bellman_ford_multi_predecessors(&simplified_graph, node_to_index[current]).unwrap();
        for dest in nodes.iter() {
            let mut subpaths = vec![];
            if current == dest {
                paths.insert((*current, *dest), vec![vec![]]);
                continue;
            }
            let mut work = vec![(*dest, vec![*dest])];
            while let Some((next_node, mut path)) = work.pop() {
                if next_node == *current {
                    path.reverse();
                    subpaths.push(path);
                } else {
                    let index = simplified_graph.to_index(snode_to_index[&next_node]);
                    let Some(dests) = &bpf.predecessors[index] else {
                        continue;
                    };
                    for dest in dests.iter() {
                        let dn = simplified_graph[*dest];
                        let mut pc = path.clone();
                        pc.push(dn);
                        work.push((dn, pc));
                    }
                }
            }
            let epaths = subpaths
                .into_iter()
                .map(|path| {
                    path.into_iter()
                        .tuple_windows()
                        .map(|(n1, n2)| {
                            let n1i = node_to_index[&n1];
                            let n2i = node_to_index[&n2];
                            let edge = graph.find_edge(n1i, n2i).unwrap();
                            let weight = graph[edge];
                            weight
                        })
                        .collect()
                })
                .collect();
            paths.insert((*current, *dest), epaths);
        }
    }
    paths
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CodeButton {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

impl CodeButton {
    fn from_char(c: char) -> Self {
        use CodeButton::*;
        match c {
            '0' => Zero,
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'A' => A,
            _ => panic!("unhandled char {}", c),
        }
    }

    fn digit_value(&self) -> Option<usize> {
        use CodeButton::*;

        let v = match self {
            Zero => 0,
            One => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            _ => return None,
        };
        Some(v)
    }
}

#[derive(Debug, Clone)]
struct Code {
    buttons: Vec<CodeButton>,
}

impl Code {
    fn from_line(s: &str) -> Self {
        Self {
            buttons: s.chars().map(|c| CodeButton::from_char(c)).collect(),
        }
    }

    fn numeric_prefix(&self) -> usize {
        self.buttons
            .iter()
            .map(|s| s.digit_value())
            .take_while(Option::is_some)
            .filter_map(|s| s)
            .collect::<Vec<_>>()
            .into_iter()
            .rfold((1, 0), |(multiplier, sum), value| {
                let new_sum = (multiplier * value) + sum;
                let new_multiplier = 10 * multiplier;
                (new_multiplier, new_sum)
            })
            .1
    }

    /*
     * The shortest sequence is a path where each cost is the
     * cost of pressing each button on the highest tier of dpad
     */
    fn shortest_sequence(&self, numpad: &mut NumPad, tier: usize) -> usize {
        self.buttons
            .iter()
            .fold((CodeButton::A, 0usize), |(current, cost), next| {
                let shortest_paths = &numpad.paths[&(current, *next)];
                let this_cost = shortest_paths
                    .iter()
                    .map(|p| {
                        let mut p = p.clone();
                        p.push(DPadButton::A);
                        shortest_sequence(p, tier)
                    })
                    .min()
                    .unwrap();
                (*next, cost + this_cost)
            })
            .1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum DPadButton {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl DPadButton {
    fn mirror(&self) -> Self {
        use DPadButton::*;

        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
            A => panic!("nope"),
        }
    }
}

#[derive(Debug, Clone)]
struct NumPad {
    paths: BTreeMap<(CodeButton, CodeButton), Vec<Vec<DPadButton>>>,
}

fn add_edges(
    graph: &mut DiGraph<CodeButton, DPadButton>,
    nodes: &BTreeMap<CodeButton, NodeIndex>,
    from_c: CodeButton,
    to_c: CodeButton,
    dir: DPadButton,
) {
    let source_node = nodes[&from_c];
    let dest_node = nodes[&to_c];
    graph.add_edge(source_node, dest_node, dir);
    graph.add_edge(dest_node, source_node, dir.mirror());
}

impl NumPad {
    fn new() -> Self {
        use CodeButton::*;
        use DPadButton::{Down, Right};
        let mut graph = DiGraph::new();
        let mut nodes = BTreeMap::<CodeButton, NodeIndex>::new();
        for cb in [
            Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine, A,
        ] {
            nodes.entry(cb).or_insert_with(|| graph.add_node(cb));
        }
        add_edges(&mut graph, &nodes, Seven, Eight, Right);
        add_edges(&mut graph, &nodes, Eight, Nine, Right);
        add_edges(&mut graph, &nodes, Four, Five, Right);
        add_edges(&mut graph, &nodes, Five, Six, Right);
        add_edges(&mut graph, &nodes, One, Two, Right);
        add_edges(&mut graph, &nodes, Two, Three, Right);
        add_edges(&mut graph, &nodes, Zero, A, Right);
        add_edges(&mut graph, &nodes, Seven, Four, Down);
        add_edges(&mut graph, &nodes, Four, One, Down);
        add_edges(&mut graph, &nodes, Eight, Five, Down);
        add_edges(&mut graph, &nodes, Five, Two, Down);
        add_edges(&mut graph, &nodes, Two, Zero, Down);
        add_edges(&mut graph, &nodes, Nine, Six, Down);
        add_edges(&mut graph, &nodes, Six, Three, Down);
        add_edges(&mut graph, &nodes, Three, A, Down);
        let paths = find_all_shortest_paths(&graph);
        Self { paths }
    }
}

#[derive(Debug, Clone)]
struct DPads {
    paths: BTreeMap<(DPadButton, DPadButton), Vec<Vec<DPadButton>>>,
}

impl DPads {
    fn new() -> Self {
        use DPadButton::*;
        let mut graph = DiGraph::new();

        let mut nodes: BTreeMap<DPadButton, NodeIndex> = BTreeMap::new();

        for key in [Left, Down, Up, Right, A] {
            nodes.insert(key, graph.add_node(key));
        }

        Self::add_dpad_edge(&mut graph, &nodes, Left, Down, Right);
        Self::add_dpad_edge(&mut graph, &nodes, Down, Right, Right);
        Self::add_dpad_edge(&mut graph, &nodes, Up, A, Right);
        Self::add_dpad_edge(&mut graph, &nodes, Up, Down, Down);
        Self::add_dpad_edge(&mut graph, &nodes, A, Right, Down);

        let paths = find_all_shortest_paths(&graph);

        Self { paths }
    }

    fn add_dpad_edge(
        graph: &mut DiGraph<DPadButton, DPadButton>,
        nodes: &BTreeMap<DPadButton, NodeIndex>,
        source: DPadButton,
        dest: DPadButton,
        direction: DPadButton,
    ) {
        let snode = nodes[&source];
        let dnode = nodes[&dest];
        graph.add_edge(snode, dnode, direction);
        graph.add_edge(dnode, snode, direction.mirror());
    }
}

static DPADS: OnceLock<DPads> = OnceLock::new();

fn get_dpads() -> &'static DPads {
    DPADS.get().unwrap()
}

#[memoize(Capacity: 10000000)]
fn shortest_sequence(path: Vec<DPadButton>, tier: usize) -> usize {
    let pads = get_dpads();

    path.iter()
        .fold((DPadButton::A, 0usize), |(current, cost), next| {
            let shortest_paths = &pads.paths[&(current, *next)];
            let this_cost = if tier == 0 {
                shortest_paths.first().map(|s| s.len()).unwrap_or(0) + 1
            } else {
                shortest_paths
                    .iter()
                    .map(|p| {
                        let mut p = p.clone();
                        p.push(DPadButton::A);
                        shortest_sequence(p, tier - 1)
                    })
                    .min()
                    .unwrap()
            };
            (*next, cost + this_cost)
        })
        .1
}

#[derive(Debug, Clone)]
struct Problem {
    codes: Vec<Code>,
    numpad: NumPad,
}

impl Problem {
    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let codes = s
            .lines()
            .map(|s| Code::from_line(s.trim()))
            .collect::<Vec<_>>();
        DPADS.set(DPads::new()).unwrap();
        let numpad = NumPad::new();
        Ok(Problem { codes, numpad })
    }

    fn simulate(&mut self, tier: usize) -> usize {
        self.codes
            .iter()
            .map(|code| {
                let np = code.numeric_prefix();
                let ss = code.shortest_sequence(&mut self.numpad, tier);
                let complexity = ss * np;
                complexity
            })
            .sum()
    }
}

fn main() -> anyhow::Result<()> {
    let p = Problem::read()?;
    println!("part 1: {}", p.clone().simulate(1));
    println!("part 2: {}", p.clone().simulate(24));
    Ok(())
}
