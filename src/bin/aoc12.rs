use aoclib::{DenseGrid, Point};

use std::collections::{BTreeSet, VecDeque};
use std::io::Read;

struct RegionsIter<'a> {
    grid: &'a DenseGrid<char>,
    points: BTreeSet<Point>,
}

impl<'a> RegionsIter<'a> {
    fn new(grid: &'a DenseGrid<char>) -> Self {
        Self {
            points: grid.iter().map(|(p, _)| p).collect::<BTreeSet<Point>>(),
            grid,
        }
    }
}

impl<'a> Iterator for RegionsIter<'a> {
    type Item = Vec<Point>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(start) = self.points.pop_last() else {
            return None;
        };

        let mut current_region = vec![];
        let current_crop = self.grid.get(start).unwrap();
        let mut work = vec![start];
        self.points.insert(start);
        while let Some(item) = work.pop() {
            if !self.points.contains(&item) {
                continue;
            }

            current_region.push(item);
            self.points.remove(&item);
            let (a, b, c, d) = item.ordinal_neighbors();
            for neighbor in &[a, b, c, d] {
                if self.grid.get(*neighbor) == Some(current_crop) {
                    work.push(*neighbor);
                }
            }
        }
        Some(current_region)
    }
}

fn part1(g: &DenseGrid<char>) -> usize {
    RegionsIter::new(g)
        .map(|current_region| {
            let area = current_region.len();
            let mut perimeter = 0;
            for point in &current_region {
                perimeter += 4;
                for neighbor in point.ordinal_neighbors_array() {
                    if current_region.contains(&neighbor) {
                        perimeter -= 1;
                    }
                }
            }
            area * perimeter
        })
        .sum()
}

#[derive(Debug)]
struct TopEdge {
    left: Point,
    right: Point,
}

#[derive(Debug)]
struct BottomEdge {
    left: Point,
    right: Point,
}

#[derive(Debug)]
struct LeftEdge {
    top: Point,
    bottom: Point,
}

#[derive(Debug)]
struct RightEdge {
    top: Point,
    bottom: Point,
}

#[derive(Debug)]
enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

fn part2(g: &DenseGrid<char>) -> usize {
    RegionsIter::new(g)
        .map(|current_region| {
            let current_value = g.get(*current_region.first().unwrap());
            let area = current_region.len();
            let mut top_edges: Vec<TopEdge> = vec![];
            let mut bottom_edges: Vec<BottomEdge> = vec![];
            let mut left_edges: Vec<LeftEdge> = vec![];
            let mut right_edges: Vec<RightEdge> = vec![];
            for point in &current_region {
                let (east, south, west, north) = point.ordinal_neighbors();
                if g.get(east) != current_value {
                    right_edges.push(RightEdge {
                        top: *point + Point::new(0, -1),
                        bottom: *point,
                    })
                }
                if g.get(west) != current_value {
                    left_edges.push(LeftEdge {
                        top: west + Point::new(0, -1),
                        bottom: west,
                    })
                }
                if g.get(north) != current_value {
                    top_edges.push(TopEdge {
                        left: north + Point::new(-1, 0),
                        right: north,
                    })
                }
                if g.get(south) != current_value {
                    bottom_edges.push(BottomEdge {
                        left: *point + Point::new(-1, 0),
                        right: *point,
                    })
                }
            }
            left_edges.sort_by_key(|n| (n.top.x, n.top.y));
            right_edges.sort_by_key(|n| (n.top.x, n.top.y));
            top_edges.sort_by_key(|n| (n.left.y, n.left.x));
            bottom_edges.sort_by_key(|n| (n.left.y, n.left.x));

            let mut left_edges = left_edges.into_iter().collect::<VecDeque<_>>();
            let mut right_edges = right_edges.into_iter().collect::<VecDeque<_>>();
            let mut top_edges = top_edges.into_iter().collect::<VecDeque<_>>();
            let mut bottom_edges = bottom_edges.into_iter().collect::<VecDeque<_>>();

            // coalesce the edges
            let mut edges: Vec<Edge> = vec![];

            if let Some(mut current) = left_edges.pop_front() {
                for edge in left_edges {
                    if edge.top == current.bottom {
                        current.bottom = edge.bottom;
                    } else {
                        edges.push(Edge::Left);
                        current = edge;
                    }
                }
                edges.push(Edge::Left);
            }
            if let Some(mut current) = right_edges.pop_front() {
                for edge in right_edges {
                    if edge.top == current.bottom {
                        current.bottom = edge.bottom;
                    } else {
                        edges.push(Edge::Right);
                        current = edge;
                    }
                }
                edges.push(Edge::Right);
            }
            if let Some(mut current) = top_edges.pop_front() {
                for edge in top_edges {
                    if edge.left == current.right {
                        current.right = edge.right;
                    } else {
                        edges.push(Edge::Top);
                        current = edge;
                    }
                }
                edges.push(Edge::Top);
            }
            if let Some(mut current) = bottom_edges.pop_front() {
                for edge in bottom_edges {
                    if edge.left == current.right {
                        current.right = edge.right;
                    } else {
                        edges.push(Edge::Bottom);
                        current = edge;
                    }
                }
                edges.push(Edge::Bottom);
            }
            area * edges.len()
        })
        .sum()
}

fn main() -> anyhow::Result<()> {
    let mut stdin = std::io::stdin().lock();
    let mut s = String::new();
    stdin.read_to_string(&mut s)?;
    let map = DenseGrid::from_input(&s, |f| f);
    println!("part 1: {}", part1(&map));
    println!("part 2: {}", part2(&map));
    Ok(())
}
