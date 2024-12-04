use aoclib::{DenseGrid, Point};
use itertools::Itertools;
use std::io::Read;

fn is_mas(g: &DenseGrid<char>, point1: Point, point2: Point, point3: Point) -> u32 {
    if g.get(point1) == Some('M') && g.get(point2) == Some('A') && g.get(point3) == Some('S') {
        1
    } else {
        0
    }
}

fn count_around_part1(g: &DenseGrid<char>, point: Point) -> u32 {
    let mut total = 0;

    // left
    total += is_mas(
        g,
        point + Point::new(-1, 0),
        point + Point::new(-2, 0),
        point + Point::new(-3, 0),
    );

    // left-up
    total += is_mas(
        g,
        point + Point::new(-1, -1),
        point + Point::new(-2, -2),
        point + Point::new(-3, -3),
    );

    // up
    total += is_mas(
        g,
        point + Point::new(0, -1),
        point + Point::new(0, -2),
        point + Point::new(0, -3),
    );

    // right-up
    total += is_mas(
        g,
        point + Point::new(1, -1),
        point + Point::new(2, -2),
        point + Point::new(3, -3),
    );

    // right
    total += is_mas(
        g,
        point + Point::new(1, 0),
        point + Point::new(2, 0),
        point + Point::new(3, 0),
    );

    // right-down
    total += is_mas(
        g,
        point + Point::new(1, 1),
        point + Point::new(2, 2),
        point + Point::new(3, 3),
    );

    // down
    total += is_mas(
        g,
        point + Point::new(0, 1),
        point + Point::new(0, 2),
        point + Point::new(0, 3),
    );

    // left-down
    total += is_mas(
        g,
        point + Point::new(-1, 1),
        point + Point::new(-2, 2),
        point + Point::new(-3, 3),
    );

    total
}

fn part1(g: &DenseGrid<char>) -> anyhow::Result<u32> {
    let part1: u32 = g
        .iter()
        .filter(|(_, value)| *value == 'X')
        .map(|(point, _)| count_around_part1(g, point))
        .sum();
    Ok(part1)
}

fn is_as(g: &DenseGrid<char>, point1: Point, point2: Point) -> bool {
    g.get(point1) == Some('A') && g.get(point2) == Some('S')
}

fn find_diagonal_as(g: &DenseGrid<char>, point: Point) -> Vec<Point> {
    let mut rv = vec![];

    // left-up
    if is_as(g, point + Point::new(-1, -1), point + Point::new(-2, -2)) {
        rv.push(point + Point::new(-1, -1));
    }

    // right-up
    if is_as(g, point + Point::new(1, -1), point + Point::new(2, -2)) {
        rv.push(point + Point::new(1, -1));
    }

    // right-down
    if is_as(g, point + Point::new(1, 1), point + Point::new(2, 2)) {
        rv.push(point + Point::new(1, 1));
    }

    // left-down
    if is_as(g, point + Point::new(-1, 1), point + Point::new(-2, 2)) {
        rv.push(point + Point::new(-1, 1));
    }
    rv
}

fn part2(g: &DenseGrid<char>) -> anyhow::Result<usize> {
    let diagonal_a_coords: Vec<Point> = g
        .iter()
        .filter(|(_, value)| *value == 'M')
        .flat_map(|(point, _)| find_diagonal_as(g, point))
        .collect();
    let part2 = diagonal_a_coords
        .iter()
        .counts()
        .iter()
        .filter(|(_, v)| **v > 1)
        .count();
    Ok(part2)
}

fn main() -> anyhow::Result<()> {
    let mut stdin = std::io::stdin().lock();
    let mut s = String::new();
    stdin.read_to_string(&mut s)?;
    let g = DenseGrid::from_input(&s, |c| c);
    println!("Part 1: {}", part1(&g)?);
    println!("Part 2: {}", part2(&g)?);
    Ok(())
}
