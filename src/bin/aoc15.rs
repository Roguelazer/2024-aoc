use std::collections::BTreeSet;
use std::io::Read;

use aoclib::{DenseGrid, Point};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
enum Cell {
    Robot,
    Box,
    Wall,
    Empty,
    BoxLeft,
    BoxRight,
}

impl aoclib::HasEmpty for Cell {
    fn empty_value() -> Self {
        Cell::Empty
    }
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            'O' => Cell::Box,
            '@' => Cell::Robot,
            '.' => Cell::Empty,
            '[' => Cell::BoxLeft,
            ']' => Cell::BoxRight,
            _ => panic!("unhandled input {}", c),
        }
    }
}

fn parse_instruction(c: char) -> Option<Point> {
    match c {
        '^' => Some(Point::new(0, -1)),
        'v' => Some(Point::new(0, 1)),
        '<' => Some(Point::new(-1, 0)),
        '>' => Some(Point::new(1, 0)),
        '\n' => None,
        _ => panic!("unhandled instruction '{}", c),
    }
}

#[derive(Debug, Clone)]
struct Problem {
    map: DenseGrid<Cell>,
    instructions: Vec<Point>,
    robot: Point,
}

impl Problem {
    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let (raw_map, raw_instructions) = s.split_once("\n\n").unwrap();
        let map = DenseGrid::from_input(raw_map, Cell::from_char);
        let instructions = raw_instructions
            .trim()
            .chars()
            .filter_map(parse_instruction)
            .collect();
        let robot = map.find(&Cell::Robot).unwrap();
        Ok(Problem {
            map,
            instructions,
            robot,
        })
    }

    fn score(&self) -> i64 {
        self.map
            .iter()
            .filter_map(|(p, v)| match v {
                Cell::Box => Some(p.y * 100 + p.x),
                Cell::BoxLeft => Some(p.y * 100 + p.x),
                _ => None,
            })
            .sum()
    }

    fn can_move_to(&self, direction: Point) -> Option<Vec<BTreeSet<Point>>> {
        let mut ptrs = BTreeSet::new();
        ptrs.insert(self.robot.clone());
        let mut result = vec![ptrs.clone()];
        loop {
            let mut next = BTreeSet::new();
            let mut empties = 0;
            for ptr in ptrs {
                if matches!(self.map.get(ptr), Some(Cell::Empty)) {
                    continue;
                }
                let ptr = ptr + direction;
                next.insert(ptr);
                match self.map.get(ptr) {
                    Some(Cell::Wall) => return None,
                    Some(Cell::Empty) => empties += 1,
                    Some(Cell::BoxLeft) => {
                        if direction.y != 0 {
                            next.insert(ptr + Point::new(1, 0));
                        }
                    }
                    Some(Cell::BoxRight) => {
                        if direction.y != 0 {
                            next.insert(ptr + Point::new(-1, 0));
                        }
                    }
                    Some(_) => {}
                    None => panic!("ran off the map!"),
                }
            }
            result.push(next.clone());
            if empties == next.len() {
                result.reverse();
                return Some(result);
            }
            ptrs = next;
        }
    }

    fn do_a_move(&mut self, steps: Vec<BTreeSet<Point>>, direction: Point) {
        let mirrored = direction.mirror();
        for (current, next) in steps.into_iter().tuple_windows() {
            for point in current {
                let np = point + mirrored;
                if next.contains(&np) {
                    let a = self.map.get(point).unwrap();
                    let b = self.map.get(np).unwrap();
                    self.map.set(point, b);
                    self.map.set(np, a);
                }
            }
        }
    }

    fn debug(&self) {
        self.map.dump_with(|f| match f {
            Cell::Wall => '#',
            Cell::Empty => '.',
            Cell::Box => 'O',
            Cell::Robot => '@',
            Cell::BoxLeft => '[',
            Cell::BoxRight => ']',
        })
    }

    fn simulate(mut self, debug_each: bool) -> i64 {
        let mut instructions = vec![];
        std::mem::swap(&mut self.instructions, &mut instructions);
        if debug_each {
            self.debug();
        }
        for (i, instruction) in instructions.into_iter().enumerate() {
            if debug_each {
                let instruction_c = if instruction.x == -1 {
                    '<'
                } else if instruction.x == 1 {
                    '>'
                } else if instruction.y == -1 {
                    '^'
                } else if instruction.y == 1 {
                    'v'
                } else {
                    '.'
                };
                println!("STEP {}; {}", i + 1, instruction_c);
            }
            if let Some(steps) = self.can_move_to(instruction) {
                self.do_a_move(steps, instruction);
                self.robot = self.robot + instruction;
            }
            if debug_each {
                self.debug();
            }
        }
        self.score()
    }

    fn double(&self) -> Self {
        let mut new_map = DenseGrid::new(
            Point::new(0, 0),
            Point::new(self.map.width() as i64 * 2, self.map.height() as i64),
        );
        for (p, v) in self.map.iter() {
            let p1 = Point::new(p.x * 2, p.y);
            let p2 = Point::new(p.x * 2 + 1, p.y);
            let c1 = match v {
                Cell::Box => Cell::BoxLeft,
                o => o,
            };
            let c2 = match v {
                Cell::Box => Cell::BoxRight,
                Cell::Robot => Cell::Empty,
                o => o,
            };
            new_map.set(p1, c1);
            new_map.set(p2, c2);
        }
        let robot = new_map.find(&Cell::Robot).unwrap();
        Problem {
            robot,
            map: new_map,
            instructions: self.instructions.clone(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let problem = Problem::read()?;
    println!("part 1: {}", problem.clone().simulate(false));
    println!("part 2: {}", problem.double().simulate(false));
    Ok(())
}
