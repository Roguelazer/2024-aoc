use std::io::Read;

use aoclib::{DenseGrid, HasEmpty};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    Empty,
    Filled,
}

impl HasEmpty for Cell {
    fn empty_value() -> Self {
        Cell::Empty
    }
}

#[derive(Debug, Clone)]
struct Schematic {
    grid: DenseGrid<Cell>,
    columns: Vec<usize>,
}

impl Schematic {
    fn new(grid: DenseGrid<Cell>, is_lock: bool) -> Self {
        let columns = grid
            .columns()
            .map(|c| {
                if is_lock {
                    c.iter().take_while(|i| matches!(i, Cell::Filled)).count()
                } else {
                    c.iter()
                        .rev()
                        .take_while(|i| matches!(i, Cell::Filled))
                        .count()
                }
            })
            .collect();
        Schematic { grid, columns }
    }
}

#[derive(Debug)]
struct Problem {
    keys: Vec<Schematic>,
    locks: Vec<Schematic>,
}

impl Problem {
    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let mut keys = Vec::new();
        let mut locks = Vec::new();
        for item in s.split("\n\n") {
            let g = DenseGrid::from_input(item, |c| match c {
                '#' => Cell::Filled,
                '.' => Cell::Empty,
                _ => {
                    panic!("unhandled input {}", c)
                }
            });
            let is_lock = g.rows().next().unwrap().iter().all(|c| *c == Cell::Filled);
            let schematic = Schematic::new(g, is_lock);
            if is_lock {
                locks.push(schematic);
            } else {
                keys.push(schematic);
            }
        }
        Ok(Problem { keys, locks })
    }

    fn part1(&self) -> usize {
        itertools::iproduct!(self.locks.iter(), self.keys.iter())
            .filter(|(lock, key)| {
                let height = lock.grid.height();
                lock.columns
                    .iter()
                    .zip(key.columns.iter())
                    .all(|(lc, kc)| lc + kc <= height)
            })
            .count()
    }
}

fn main() -> anyhow::Result<()> {
    let p = Problem::read()?;
    println!("part 1: {}", p.part1());
    Ok(())
}
