use aoclib::{DenseGrid, Point};
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Cell {
    Empty,
    Antenna(char),
}

impl Default for Cell {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug)]
struct Problem {
    grid: DenseGrid<Cell>,
    antennas: Vec<(Point, char)>,
}

impl Problem {
    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let grid = DenseGrid::from_input(&s, |c| match c {
            '.' => Cell::Empty,
            other => Cell::Antenna(other),
        });
        let antennas = grid
            .iter()
            .filter_map(|(p, v)| match v {
                Cell::Empty => None,
                Cell::Antenna(v) => Some((p, v)),
            })
            .map(|(p, v)| (p, v))
            .collect::<Vec<_>>();
        Ok(Self { grid, antennas })
    }
}

fn part1(p: &Problem) -> usize {
    let mut antinodes = BTreeSet::new();
    for (cell, _) in p.grid.iter() {
        for (other, value) in &p.antennas {
            let other = *other;
            if other == cell {
                continue;
            }
            let distance = other - cell;
            let doubled = other + distance;
            if p.grid.get(doubled) == Some(Cell::Antenna(*value)) {
                antinodes.insert(cell);
            }
        }
    }
    antinodes.len()
}

fn part2(p: &Problem) -> usize {
    let mut antinodes: BTreeSet<Point> = BTreeSet::new();
    for (p, _) in &p.antennas {
        antinodes.insert(*p);
    }
    let mut by_char = BTreeMap::new();
    for (point, value) in &p.antennas {
        by_char
            .entry(value)
            .or_insert_with(Vec::<Point>::new)
            .push(*point);
    }
    for (_, points) in by_char {
        for (lhs, rhs) in points.iter().tuple_combinations() {
            let rhs = *rhs;
            let lhs = *lhs;
            let right = rhs - lhs;
            let mut test_point = rhs + right;
            while p.grid.contains(test_point) {
                antinodes.insert(test_point);
                test_point = test_point + right;
            }
            let left = lhs - rhs;
            let mut test_point = lhs + left;
            while p.grid.contains(test_point) {
                antinodes.insert(test_point);
                test_point = test_point + left;
            }
        }
    }
    antinodes.len()
}

fn main() -> anyhow::Result<()> {
    let problem = Problem::read()?;
    println!("part 1: {}", part1(&problem));
    println!("part 2: {}", part2(&problem));
    Ok(())
}
