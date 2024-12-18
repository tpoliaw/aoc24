use std::{
    collections::{BTreeSet, HashMap, HashSet},
    ops::Add,
};

use aoc24::input;

pub fn main() {
    let all = input(18).map_by_line(Pos::from_line).collect::<Vec<_>>();

    let start = Pos { row: 0, col: 0 };
    let end = Pos { row: 70, col: 70 };

    let mem = all[..1024].iter().cloned().collect();
    let visited = find_route(end, &mem, [(0, start)].into(), HashMap::new());
    println!("Part 1: {}", visited[&end]);

    let mut mem = all.iter().cloned().collect::<HashSet<_>>();
    let mut visited = find_route(end, &mem, [(0, start)].into(), HashMap::new());
    for byte in all.into_iter().rev() {
        mem.remove(&byte);
        visited = find_route(
            end,
            &mem,
            [
                visited.get(&(byte + Dir::Up)).map(|d| (*d, byte + Dir::Up)),
                visited
                    .get(&(byte + Dir::Left))
                    .map(|d| (*d, byte + Dir::Left)),
                visited
                    .get(&(byte + Dir::Down))
                    .map(|d| (*d, byte + Dir::Down)),
                visited
                    .get(&(byte + Dir::Right))
                    .map(|d| (*d, byte + Dir::Right)),
            ]
            .into_iter()
            .flatten()
            .collect(),
            visited,
        );
        if visited.contains_key(&end) {
            println!("Part 2: {},{}", byte.row, byte.col);
            break;
        }
    }
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

fn find_route(
    end: Pos,
    mem: &HashSet<Pos>,
    mut options: BTreeSet<(usize, Pos)>,
    mut visited: HashMap<Pos, usize>,
) -> HashMap<Pos, usize> {
    while let Some((d, pos)) = options.pop_first() {
        if !pos.in_limits(end) {
            continue;
        }
        if visited.get(&pos).is_some_and(|v| *v < d) {
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

    visited
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
