use aoclib::DijkstraMetric;
use aoclib::{DenseGrid, Point};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
enum Cell {
    Wall,
    Empty,
    Start,
    End,
}

impl aoclib::HasEmpty for Cell {
    fn empty_value() -> Self {
        Self::Empty
    }
}

impl Cell {
    fn parse(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            'S' => Cell::Start,
            'E' => Cell::End,
            _ => panic!("unhandled input {}", c),
        }
    }

    fn traversible(&self) -> bool {
        match self {
            Self::Wall => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    map: DenseGrid<Cell>,
}

impl Problem {
    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let map = DenseGrid::from_input(&s, Cell::parse);
        Ok(Problem { map })
    }

    fn solve(&self, max_shortcut_len: usize, threshold: usize) -> anyhow::Result<usize> {
        let end = self.map.find(&Cell::End).unwrap();

        let (dmap, _) = self.map.dijkstra(
            end,
            |g, p| g.get(p).map(|c| c.traversible()).unwrap_or(false),
            |_, _, _| 1,
        )?;

        let msl = max_shortcut_len as i64;

        // for every cell, measure the time distance between it and the
        // lowest value with manhattan distance 20
        let mut shortcuts = BTreeMap::new();
        for (point, value) in self.map.iter() {
            if value == Cell::Wall {
                continue;
            }
            let Some(DijkstraMetric::Finite(my_time)) = dmap.get(point) else {
                continue;
            };
            for xdelta in (-1 * msl)..=msl {
                for ydelta in (-1 * msl)..=msl {
                    let other = point + Point::new(xdelta, ydelta);
                    let distance = point.manhattan_distance_to(other);
                    if distance > max_shortcut_len {
                        continue;
                    }
                    match self.map.get(other) {
                        Some(Cell::Wall) | None => continue,
                        _ => {}
                    }
                    let Some(DijkstraMetric::Finite(their_time)) = dmap.get(other) else {
                        continue;
                    };
                    if their_time > my_time {
                        continue;
                    }
                    if their_time + distance > my_time {
                        continue;
                    }
                    let saved = my_time - their_time - distance;
                    if saved >= threshold {
                        shortcuts
                            .entry(saved)
                            .or_insert_with(BTreeSet::new)
                            .insert((point, other));
                    }
                }
            }
        }
        let shortcut_count: usize = shortcuts.values().map(|v| v.len()).sum();
        Ok(shortcut_count)
    }
}

fn main() -> anyhow::Result<()> {
    let problem = Problem::read()?;
    println!("part 1: {}", problem.solve(2, 100)?);
    println!("part 2: {}", problem.solve(20, 100)?);
    Ok(())
}
