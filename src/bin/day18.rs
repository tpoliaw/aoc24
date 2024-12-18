use std::{
    collections::{BTreeSet, HashMap, HashSet},
    ops::Add,
};

use aoc24::input;

pub fn main() {
    let mem = input(18)
        .map_by_line(Pos::from_line)
        .take(1024)
        .collect::<HashSet<_>>();

    let start = Pos { row: 0, col: 0 };
    let end = Pos { row: 70, col: 70 };
    let p1 = find_route(start, end, &mem).unwrap();
    println!("Part 1: {}", p1);

    let all = input(18).map_by_line(Pos::from_line).collect::<Vec<_>>();
    let mut low = 1024;
    let mut high = all.len();
    while high > low + 1 {
        let mid = (high + low) / 2;
        let mem = all[..=mid].iter().cloned().collect::<HashSet<_>>();
        match find_route(start, end, &mem) {
            Some(_) => low = mid,
            None => high = mid,
        }
    }
    println!("Part 2: {},{}", all[high].row, all[high].col);
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

fn find_route(start: Pos, end: Pos, mem: &HashSet<Pos>) -> Option<usize> {
    let mut visited = HashMap::new();
    let mut options = BTreeSet::new();
    options.insert((0, start));
    while let Some((d, pos)) = options.pop_first() {
        if !pos.in_limits(end) {
            continue;
        }
        if visited.get(&pos).is_some_and(|v| *v <= d) {
            continue;
        }
        if mem.contains(&pos) {
            continue;
        }
        visited.insert(pos, d);
        if pos == end {
            break;
        }
        options.insert((d + 1, pos + Dir::Up));
        options.insert((d + 1, pos + Dir::Down));
        options.insert((d + 1, pos + Dir::Left));
        options.insert((d + 1, pos + Dir::Right));
    }

    visited.get(&end).cloned()
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
