use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    ops::Add,
};

use aoc24::input;

pub fn main() {
    let src = input(20).string();
    let all = src
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(col, _)| Pos {
                    row: row as i32,
                    col: col as i32,
                })
        })
        .collect::<Vec<_>>();

    let walls = all.iter().cloned().collect::<HashSet<_>>();
    println!("Walls: {}", walls.len());

    let width = src.find('\n').unwrap();
    let start = src.find('S').unwrap();
    let end = src.find('E').unwrap();
    let start = Pos {
        row: (start / (width + 1)) as i32,
        col: (start % (width + 1)) as i32,
    };
    let end = Pos {
        row: (end / (width + 1)) as i32,
        col: (end % (width + 1)) as i32,
    };

    let from_start = min_dists(start, end, &walls);
    let from_end = min_dists(end, start, &walls);
    assert!(from_end[&start] == from_start[&end]);

    let baseline = from_end[&start];
    println!("Baseline: {baseline}");

    let p1 = count_cheats(&from_start, &from_end, baseline, 2);
    println!("Part 1: {p1}");
    let p2 = count_cheats(&from_start, &from_end, baseline, 20);
    println!("Part 2: {p2}");
}

fn count_cheats(
    from_start: &BTreeMap<Pos, usize>,
    from_end: &BTreeMap<Pos, usize>,
    baseline: usize,
    skips: u32,
) -> usize {
    let mut cheats: HashMap<usize, usize> = HashMap::new();
    for (cell, sdist) in from_start.iter() {
        for (end, edist) in from_end
            .iter()
            .filter(|(_, v)| **v < baseline - sdist - skips as usize)
        {
            let offset = cell.row.abs_diff(end.row) + cell.col.abs_diff(end.col);
            if offset <= skips {
                let dist = sdist + edist + offset as usize;
                if dist < baseline {
                    *cheats.entry(baseline - dist).or_default() += 1;
                }
            }
        }
    }
    cheats
        .iter()
        .filter(|(k, _)| **k >= 100)
        .map(|(_, v)| v)
        .sum::<usize>()
}

fn min_dists(start: Pos, end: Pos, walls: &HashSet<Pos>) -> BTreeMap<Pos, usize> {
    let mut options: BTreeSet<(usize, Pos)> = [(0, start)].into();
    let mut visited = BTreeMap::new();
    while let Some((d, pos)) = options.pop_first() {
        if visited.get(&pos).is_some_and(|v| *v < d) {
            continue;
        }
        if walls.contains(&pos) {
            continue;
        }
        visited.insert(pos, d);
        if end == pos {
            break;
        }
        options.extend(Dir::each().map(|dr| (d + 1, pos + dr)));
    }

    visited
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    row: i32,
    col: i32,
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    fn each() -> impl Iterator<Item = Self> {
        [Dir::Up, Dir::Down, Dir::Right, Dir::Left].into_iter()
    }
}
impl Pos {
    fn from_line(ln: String) -> Self {
        let (r, c) = ln.split_once(',').unwrap();
        Self {
            row: r.parse().unwrap(),
            col: c.parse().unwrap(),
        }
    }
    fn in_limits(&self, limit: Pos) -> bool {
        0 <= self.row && self.row <= limit.row && 0 <= self.col && self.col <= limit.col
    }
    fn neighbours(&self, walls: &HashSet<Pos>) -> usize {
        Dir::each()
            .map(|d| *self + d)
            .filter(|p| walls.contains(p))
            .count()
    }
}

impl Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, rhs: Dir) -> Self::Output {
        let Pos { row, col } = self;
        match rhs {
            Dir::Up => Pos { row: row - 1, col },
            Dir::Down => Pos { row: row + 1, col },
            Dir::Left => Pos { row, col: col - 1 },
            Dir::Right => Pos { row, col: col + 1 },
        }
    }
}
