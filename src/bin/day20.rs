use std::{
    collections::{BTreeSet, HashMap, HashSet},
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

    let mut walls = all.iter().cloned().collect::<HashSet<_>>();
    println!("Walls: {}", walls.len());

    let width = src.find('\n').unwrap();
    let height = src.len() / (width + 1);
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

    let limit = Pos {
        row: height as i32,
        col: width as i32,
    };
    let baseline = min_route(start, end, &walls, limit);
    println!("Baseline: {baseline}");
    let mut cheats: HashMap<usize, usize> = HashMap::new();
    let mut step = 0;
    for wall in all {
        print!("\r{step}/{}", walls.len());
        if wall.neighbours(&walls) >= 2 {
            walls.remove(&wall);
            let min = min_route(start, end, &walls, limit);
            print!("  = {min}");
            if min < baseline {
                *cheats.entry(baseline - min).or_default() += 1;
            }
            walls.insert(wall);
        }
        step += 1;
    }
    println!();
    let p1 = cheats
        .iter()
        .filter(|(k, _)| **k >= 100)
        .map(|(_, v)| v)
        .sum::<usize>();
    println!("Part 1: {p1}");
}

fn min_route(start: Pos, end: Pos, walls: &HashSet<Pos>, limit: Pos) -> usize {
    let mut options: BTreeSet<(usize, Pos)> = [(0, start)].into();
    let mut visited = HashMap::new();
    while let Some((d, pos)) = options.pop_first() {
        if visited.get(&pos).is_some_and(|v| *v < d) {
            continue;
        }
        if !pos.in_limits(limit) {
            continue;
        }
        if walls.contains(&pos) {
            //} && !jump.is_some_and(|p| p == pos) {
            continue;
        }
        visited.insert(pos, d);
        if pos == end {
            break;
        }
        options.extend(Dir::each().map(|dr| (d + 1, pos + dr)));
    }

    *visited.get(&end).unwrap()
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
