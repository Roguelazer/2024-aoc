use aoclib::Point;
use std::cmp::max;
use std::collections::BTreeSet;
use std::io::Read;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_halfpi_clockwise(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Guard {
    position: Point,
    direction: Direction,
}

impl Guard {
    fn move_to(&mut self, end_position: Point) -> Vec<Self> {
        let mut res = vec![];
        match self.direction {
            Direction::Up => {
                assert!(self.position.x == end_position.x);
                assert!(self.position.y >= end_position.y);
            }
            Direction::Down => {
                assert!(self.position.x == end_position.x);
                assert!(self.position.y <= end_position.y);
            }
            Direction::Left => {
                assert!(self.position.y == end_position.y);
                assert!(self.position.x >= end_position.x);
            }
            Direction::Right => {
                assert!(self.position.y == end_position.y);
                assert!(self.position.x <= end_position.x);
            }
        }
        while self.position != end_position {
            res.push(self.clone());
            match self.direction {
                Direction::Up => self.position.y -= 1,
                Direction::Down => self.position.y += 1,
                Direction::Left => self.position.x -= 1,
                Direction::Right => self.position.x += 1,
            }
        }
        res.push(self.clone());
        self.direction = self.direction.rotate_halfpi_clockwise();
        res
    }
}

#[derive(Debug)]
struct Map {
    obstructions: Vec<Point>,
    guard: Guard,
    max_x: i64,
    max_y: i64,
}

impl Map {
    fn from_str(s: &str) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut guard = None;
        let mut obstructions = vec![];
        for (y, line) in s.lines().enumerate() {
            max_y = max(max_y, y as i64);
            for (x, cell) in line.chars().enumerate() {
                max_x = max(max_x, x as i64);
                let this = Point::new(x as i64, y as i64);
                if cell == '#' {
                    obstructions.push(this);
                } else if cell == '^' {
                    guard = Some(Guard {
                        position: this,
                        direction: Direction::Up,
                    })
                }
            }
        }
        Map {
            max_x,
            max_y,
            obstructions,
            guard: guard.unwrap(),
        }
    }

    fn find_next_obstruction(&self, position: Point, direction: Direction) -> (Point, bool) {
        let end = match direction {
            Direction::Up => self
                .obstructions
                .iter()
                .filter(|p| p.x == position.x && p.y <= position.y)
                .max_by_key(|p| p.y),
            Direction::Left => self
                .obstructions
                .iter()
                .filter(|p| p.y == position.y && p.x <= position.x)
                .max_by_key(|p| p.x),
            Direction::Down => self
                .obstructions
                .iter()
                .filter(|p| p.x == position.x && p.y >= position.y)
                .min_by_key(|p| p.y),
            Direction::Right => self
                .obstructions
                .iter()
                .filter(|p| p.y == position.y && p.x >= position.x)
                .min_by_key(|p| p.x),
        };
        if let Some(end) = end {
            let pt = match direction {
                Direction::Up => Point::new(end.x, end.y + 1),
                Direction::Down => Point::new(end.x, end.y - 1),
                Direction::Left => Point::new(end.x + 1, end.y),
                Direction::Right => Point::new(end.x - 1, end.y),
            };
            (pt, false)
        } else {
            let pt = match direction {
                Direction::Up => Point::new(position.x, 0),
                Direction::Down => Point::new(position.x, self.max_y),
                Direction::Left => Point::new(0, position.y),
                Direction::Right => Point::new(self.max_x, position.y),
            };
            (pt, true)
        }
    }

    fn get_path_and_loop(&self) -> (BTreeSet<Point>, bool) {
        let mut visited = BTreeSet::new();
        let mut visited_guards = BTreeSet::new();
        let mut guard = self.guard.clone();
        visited_guards.insert(self.guard.clone());
        loop {
            let (end_point, done) = self.find_next_obstruction(guard.position, guard.direction);
            for point in guard.move_to(end_point) {
                visited.insert(point.position);
            }
            if visited_guards.contains(&guard) {
                return (BTreeSet::new(), true);
            }
            visited_guards.insert(guard.clone());
            if done {
                break;
            }
        }
        (visited, false)
    }

    fn simulate_part1(&self) -> usize {
        self.get_path_and_loop().0.len()
    }

    fn does_loop(&self) -> bool {
        self.get_path_and_loop().1
    }

    fn simulate_part2(&mut self) -> usize {
        let mut loop_points = BTreeSet::<Point>::new();

        // the only positions that matter are on the original path
        let candidates = self.get_path_and_loop().0;
        for candidate in candidates {
            // can't plop something down directly on top of the guard, or in
            // front of them
            if candidate == self.guard.position {
                continue;
            }
            if candidate == self.guard.position + Point::new(0, -1) {
                continue;
            }

            // simulate an obstruction there
            self.obstructions.push(candidate);
            // I bet you could do some kind of DP here to avoid resimulating
            // the whole thing every time...
            if self.does_loop() {
                loop_points.insert(candidate);
            }
            self.obstructions.pop();
        }
        loop_points.len()
    }
}

fn main() -> anyhow::Result<()> {
    let mut stdin = std::io::stdin().lock();
    let mut s = String::new();
    stdin.read_to_string(&mut s)?;
    let mut map = Map::from_str(&s);
    println!("part 1: {}", map.simulate_part1());
    println!("part 2: {}", map.simulate_part2());
    Ok(())
}
