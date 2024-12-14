use std::collections::BTreeMap;
use std::io::Read;

use aoclib::{DenseGrid, Point};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace1, newline};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Point,
    velocity: Point,
}

fn parse_point(s: &str) -> IResult<&str, Point> {
    map(
        separated_pair(
            nom::character::complete::i64,
            tag(","),
            nom::character::complete::i64,
        ),
        |(x, y)| Point::new(x, y),
    )(s)
}

impl Robot {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                preceded(tag("p="), parse_point),
                multispace1,
                preceded(tag("v="), parse_point),
            ),
            |(position, velocity)| Robot { position, velocity },
        )(s)
    }

    fn advance(&mut self, steps: u32, width: i64, height: i64) {
        self.position = Point::new(
            (self.position.x + self.velocity.x * steps as i64).rem_euclid(width),
            (self.position.y + self.velocity.y * steps as i64).rem_euclid(height),
        )
    }
}

#[derive(Debug, Clone)]
struct Problem {
    robots: Vec<Robot>,
    width: i64,
    height: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Problem {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            tuple((
                terminated(nom::character::complete::i64, newline),
                terminated(nom::character::complete::i64, newline),
                separated_list1(newline, Robot::parse),
            )),
            |(width, height, robots)| Problem {
                robots,
                width,
                height,
            },
        )(s)
    }

    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let (remainder, obj) = Self::parse(s.trim()).unwrap();
        if !remainder.is_empty() {
            anyhow::bail!("unparsed input: {:?}", remainder);
        }
        Ok(obj)
    }

    fn count_by_quadrant(&self) -> usize {
        let mut robots_by_quadrant = BTreeMap::new();
        let vdiv = self.width / 2;
        let hdiv = self.height / 2;
        for robot in &self.robots {
            let quadrant = match (
                robot.position.x < vdiv,
                robot.position.x > vdiv,
                robot.position.y < hdiv,
                robot.position.y > hdiv,
            ) {
                (true, false, true, false) => Some(Quadrant::TopLeft),
                (false, true, true, false) => Some(Quadrant::TopRight),
                (true, false, false, true) => Some(Quadrant::BottomLeft),
                (false, true, false, true) => Some(Quadrant::BottomRight),
                _ => None,
            };
            if let Some(quadrant) = quadrant {
                *robots_by_quadrant.entry(quadrant).or_insert(0) += 1
            }
        }
        println!("{:?}", robots_by_quadrant);
        robots_by_quadrant.values().fold(1, |a, b| a * b)
    }

    fn debug_grid(&self) -> DenseGrid<u32> {
        let mut grid = DenseGrid::new(Point::new(0, 0), Point::new(self.width, self.height));
        let mut robots_by_position: BTreeMap<Point, u32> = BTreeMap::new();
        for robot in &self.robots {
            *robots_by_position.entry(robot.position).or_insert(0) += 1
        }
        for (position, count) in robots_by_position.iter() {
            grid.set(*position, *count);
        }
        grid
    }

    fn debug(&self) {
        let grid = self.debug_grid();
        grid.dump_with(|f| match f {
            0 => '.',
            i if *i < 10 => char::from_digit(*i, 10).unwrap(),
            _ => 'h',
        })
    }

    fn debug_image(&self, s: String) {
        let grid = self.debug_grid();
        let max_count = grid.iter().map(|(_, v)| v).max().unwrap() as f32;
        grid.save_to_image(
            |f| {
                if *f == 0 {
                    image::Rgb([255, 255, 255])
                } else {
                    let frac = (255.0 * *f as f32 / max_count) as u8;
                    image::Rgb([0, frac, 0])
                }
            },
            &s,
        )
        .unwrap();
    }

    fn part1(&mut self) -> usize {
        self.debug();
        for robot in self.robots.iter_mut() {
            robot.advance(100, self.width, self.height)
        }
        println!("{:?}", self);
        self.debug();
        self.count_by_quadrant()
    }

    fn part2(&mut self) -> usize {
        for i in 1..=65536 {
            for robot in self.robots.iter_mut() {
                robot.advance(1, self.width, self.height);
            }
            let num_distinct_points = self.robots.iter().map(|r| r.position).unique().count();
            if num_distinct_points == self.robots.len() {
                self.debug_image(format!("aoc14_step{}.png", i));
                return i;
            }
        }
        panic!("failed");
    }
}

fn main() -> anyhow::Result<()> {
    let problem = Problem::read()?;
    println!("part 1: {}", problem.clone().part1());
    println!("part 2: {}", problem.clone().part2());
    Ok(())
}
