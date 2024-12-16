use anyhow::Context;
use aoclib::{DenseGrid, Point};
use itertools::Itertools;
use petgraph::visit::NodeIndexable;
use smallvec::SmallVec;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cell {
    Empty,
    Wall,
    Start,
    End,
}

impl Cell {
    fn is_passable(&self) -> bool {
        match self {
            Cell::Wall => false,
            _ => true,
        }
    }
}

impl aoclib::HasEmpty for Cell {
    fn empty_value() -> Self {
        Cell::Empty
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turns_to(&self, other: Direction) -> usize {
        use Direction::{East, North, South, West};

        match (*self, other) {
            (North, North) => 0,
            (East, East) => 0,
            (West, West) => 0,
            (South, South) => 0,
            (North, East) => 1,
            (East, North) => 1,
            (North, South) => 2,
            (South, North) => 2,
            (North, West) => 1,
            (West, North) => 1,
            (East, South) => 1,
            (South, East) => 1,
            (East, West) => 2,
            (West, East) => 2,
            (South, West) => 1,
            (West, South) => 1,
        }
    }

    fn as_vector(&self) -> Point {
        match *self {
            Direction::North => Point::new(0, -1),
            Direction::East => Point::new(1, 0),
            Direction::West => Point::new(-1, 0),
            Direction::South => Point::new(0, 1),
        }
    }

    fn between(lhs: Point, rhs: Point) -> Self {
        let delta = rhs - lhs;
        if delta == Point::new(1, 0) {
            Direction::East
        } else if delta == Point::new(-1, 0) {
            Direction::West
        } else if delta == Point::new(0, 1) {
            Direction::South
        } else if delta == Point::new(0, -1) {
            Direction::North
        } else {
            panic!("bwah {} {} {}", lhs, rhs, delta);
        }
    }
}

#[derive(Debug)]
struct Problem {
    map: DenseGrid<Cell>,
    intersections: BTreeSet<Point>,
}

fn trim_dead_ends(map: &mut DenseGrid<Cell>, intersections: &mut BTreeSet<Point>) -> bool {
    let mut removed = false;
    let dead_ends = map
        .iter()
        .filter(|(p, v)| {
            if *v != Cell::Empty {
                return false;
            }
            let neighbors = p
                .ordinal_neighbors_array()
                .iter()
                .filter(|np| map.get(**np).map(|c| c.is_passable()).unwrap_or(false))
                .count();
            neighbors == 1
        })
        .map(|(p, _)| p)
        .collect::<Vec<_>>();
    for mut point in dead_ends {
        if intersections.contains(&point) {
            intersections.remove(&point);
        }
        while !intersections.contains(&point) {
            removed = true;
            map.set(point, Cell::Wall);
            if let Some(np) = point
                .ordinal_neighbors_array()
                .iter()
                .find(|np| map.get(**np).map(|c| c.is_passable()).unwrap_or(false))
            {
                point = *np
            } else {
                break;
            }
        }
    }
    removed
}

impl Problem {
    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let mut map = DenseGrid::try_from_input(s.trim(), |c| match c {
            '#' => Ok(Cell::Wall),
            '.' => Ok(Cell::Empty),
            'S' => Ok(Cell::Start),
            'E' => Ok(Cell::End),
            _ => Err(anyhow::anyhow!("unhandled input {}", c)),
        })?;
        let mut intersections = BTreeSet::new();
        for (p, v) in map.iter() {
            if v == Cell::Wall {
                continue;
            }
            let neighs = p
                .ordinal_neighbors_array()
                .iter()
                .filter(|np| match map.get(**np) {
                    Some(Cell::Wall) => false,
                    None => false,
                    _ => true,
                })
                .map(|np| Direction::between(p, *np))
                .collect::<SmallVec<[Direction; 4]>>();
            let any_angled = neighs
                .iter()
                .tuple_combinations()
                .any(|(l, r)| l.turns_to(*r) != 2);
            if neighs.len() > 2
                || any_angled
                || map.get(p) == Some(Cell::End)
                || map.get(p) == Some(Cell::Start)
            {
                intersections.insert(p);
            }
        }
        while trim_dead_ends(&mut map, &mut intersections) {}
        Ok(Problem { map, intersections })
    }

    fn cells_between(&self, mut start: Point, direction: Direction, end: Point) -> Vec<Point> {
        let mut res = vec![];
        while start != end {
            res.push(start);
            start = start + direction.as_vector();
        }
        res.push(end);
        res
    }

    fn walk_to_intersection(
        &self,
        mut point: Point,
        direction: Direction,
    ) -> Option<(Point, usize)> {
        let mut steps = 1;
        let v = direction.as_vector();
        while !self.intersections.contains(&point) {
            point = point + v;
            match self.map.get(point) {
                Some(Cell::Wall) => {
                    return None;
                }
                None => {
                    panic!("ran off the map at {}", point);
                }
                _ => {}
            }
            steps += 1;
        }
        return Some((point, steps));
    }

    fn build_graph(
        &self,
    ) -> (
        petgraph::graph::DiGraph<(Point, Direction), f32>,
        BTreeMap<(Point, Direction), petgraph::graph::NodeIndex>,
    ) {
        let mut nodes: BTreeMap<(Point, Direction), petgraph::graph::NodeIndex> = BTreeMap::new();

        let mut graph = petgraph::graph::DiGraph::<(Point, Direction), f32>::new();

        for point in &self.intersections {
            let point = *point;
            for direction in [
                Direction::North,
                Direction::East,
                Direction::West,
                Direction::South,
            ] {
                let next = point + direction.as_vector();
                if !self.map.get(next).map(|p| p.is_passable()).unwrap_or(false) {
                    continue;
                }
                // ray-cast in that direction
                if let Some((next_intersection, distance)) =
                    self.walk_to_intersection(next, direction)
                {
                    let node = *nodes
                        .entry((point, direction))
                        .or_insert_with(|| graph.add_node((point, direction)));
                    let next_node = *nodes
                        .entry((next_intersection, direction))
                        .or_insert_with(|| graph.add_node((next_intersection, direction)));
                    graph.add_edge(node, next_node, distance as f32);
                }
            }
            for (da, db) in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .iter()
            .circular_tuple_windows()
            {
                let node = *nodes
                    .entry((point, *da))
                    .or_insert_with(|| graph.add_node((point, *da)));
                let next_node = *nodes
                    .entry((point, *db))
                    .or_insert_with(|| graph.add_node((point, *db)));
                graph.add_edge(node, next_node, 1000.0);
            }
            for (da, db) in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ]
            .iter()
            .circular_tuple_windows()
            {
                let node = *nodes
                    .entry((point, *da))
                    .or_insert_with(|| graph.add_node((point, *da)));
                let next_node = *nodes
                    .entry((point, *db))
                    .or_insert_with(|| graph.add_node((point, *db)));
                graph.add_edge(node, next_node, 1000.0);
            }
        }
        (graph, nodes)
    }

    fn part1(&self) -> Option<usize> {
        let start = self.map.find(&Cell::Start).unwrap();
        let end = self.map.find(&Cell::End).unwrap();

        let (graph, nodes) = self.build_graph();

        let (cost, _) = petgraph::algo::astar(
            &graph,
            *nodes.get(&(start, Direction::East)).unwrap(),
            |node| graph.node_weight(node).unwrap().0 == end,
            |e| *e.weight(),
            |node| {
                graph
                    .node_weight(node)
                    .unwrap()
                    .0
                    .manhattan_distance_to(end) as f32
            },
        )?;
        Some(cost as usize)
    }

    fn part2(&self) -> Option<usize> {
        let start = self.map.find(&Cell::Start).unwrap();
        let end = self.map.find(&Cell::End).unwrap();

        let (graph, nodes) = self.build_graph();

        let res = aoclib::petgraph_bellman_ford_multi::bellman_ford_multi_predecessors(
            &graph,
            *nodes.get(&(start, Direction::East)).unwrap(),
        )
        .unwrap();
        let mut seen = BTreeSet::new();
        seen.insert(start);

        let mut distances_to_targets = BTreeMap::new();

        for direction in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            let target = (end, direction);

            for (i, distance) in res.distances.iter().enumerate() {
                if graph.node_weight(graph.from_index(i)) == Some(&target) {
                    distances_to_targets.insert(target, *distance as u64);
                }
            }
        }

        let best_distance: u64 = *distances_to_targets.values().min()?;
        let mut work = distances_to_targets
            .into_iter()
            .filter(|(_, v)| *v == best_distance)
            .map(|(t, _)| t)
            .collect::<VecDeque<(Point, Direction)>>();

        while let Some((p, d)) = work.pop_front() {
            seen.insert(p);
            let n = nodes.get(&(p, d)).unwrap();
            let i = graph.to_index(*n);
            if let Some(ref preds) = res.predecessors[i] {
                for pred in preds {
                    let next_node = graph.node_weight(*pred).unwrap();
                    for cell in self.cells_between(next_node.0, next_node.1, p) {
                        seen.insert(cell);
                    }
                    work.push_back(*next_node);
                }
            }
        }
        Some(seen.len())
    }
}

fn main() -> anyhow::Result<()> {
    let problem = Problem::read().context("reading problem")?;
    println!(
        "part 1: {:?}",
        problem
            .part1()
            .ok_or(anyhow::anyhow!("failed to solve part 1"))?
    );
    println!(
        "part 2: {:?}",
        problem
            .part2()
            .ok_or(anyhow::anyhow!("failed to solve part 2"))?
    );
    Ok(())
}
