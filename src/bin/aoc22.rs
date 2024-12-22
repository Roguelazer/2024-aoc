use std::{
    collections::{BTreeMap, HashMap},
    io::Read,
};

use itertools::Itertools;

const PRUNE: i64 = (1 << 24) - 1;

fn iterate(mut s: i64) -> i64 {
    let mut v = s;
    v = v << 6;
    v = v ^ s;
    v = v & PRUNE;
    s = v;
    v = v >> 5;
    v = v ^ s;
    v = v & PRUNE;
    s = v;
    v = v << 11;
    v = v ^ s;
    v = v & PRUNE;
    v
}

#[derive(Debug, Clone)]
struct Problem {
    seeds: Vec<i64>,
}

impl Problem {
    fn read() -> anyhow::Result<Self> {
        let mut stdin = std::io::stdin().lock();
        let mut s = String::new();
        stdin.read_to_string(&mut s)?;
        let seeds = s
            .lines()
            .map(|p| p.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Problem { seeds })
    }

    fn part1(&self) -> i64 {
        self.seeds
            .iter()
            .map(|seed| {
                let mut i = *seed;
                for _ in 0..2000 {
                    i = iterate(i);
                }
                i
            })
            .sum()
    }

    fn part2(&self) -> (i64, [i32; 4]) {
        let mut windows: HashMap<[i32; 4], BTreeMap<usize, (usize, i64)>> = HashMap::new();
        for (seed_idx, seed) in self.seeds.iter().enumerate() {
            let mut i = *seed;
            let mut vals = vec![];
            for _ in 0..2000 {
                vals.push(i % 10);
                i = iterate(i);
            }
            let deltas = vals
                .iter()
                .tuple_windows()
                .map(|(a, b)| (b - a) as i32)
                .collect::<Vec<_>>();
            let mut idx = 0;
            for (a, b, c, d) in deltas.into_iter().tuple_windows() {
                let value = vals[idx + 4];
                let sw = windows.entry([a, b, c, d]).or_insert_with(BTreeMap::new);
                if !sw.contains_key(&seed_idx) {
                    sw.insert(seed_idx, (idx, value));
                }
                idx += 1;
            }
        }
        windows
            .iter()
            .filter(|(_, v)| v.len() > 1)
            .map(|(k, matches)| {
                let vs = matches.values().map(|(_, v)| v).sum::<i64>();
                (vs, *k)
            })
            .max()
            .unwrap()
    }
}

fn main() -> anyhow::Result<()> {
    let problem = Problem::read()?;
    println!("part 1: {}", problem.part1());
    println!("part 2: {:?}", problem.part2());
    Ok(())
}
