use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Debug,
    ops::Add,
};

use aoc24::input;

pub fn main() {
    let src = input(16).as_value::<String>();
    let width = src.find('\n').unwrap();
    let start = src.find('S').unwrap();
    let end = src.find('E').unwrap();
    let start = Pos::new(start / (width + 1), start % (width + 1));
    let end = Pos::new(end / (width + 1), end % (width + 1));

    let maze = src
        .lines()
        .enumerate()
        .flat_map(|(row, ln)| {
            ln.chars()
                .enumerate()
                .filter(|(_, c)| *c != '#')
                .map(move |(col, c)| Pos::new(row, col))
        })
        .collect::<HashSet<_>>();
    println!("{start:?} -> {end:?}");
    println!("Part 1: {}", min_route(start, end, &maze));
}

fn min_route(from: Pos, to: Pos, maze: &HashSet<Pos>) -> usize {
    let mut pos = (from, Dir::Right);
    let mut visited = HashMap::new();
    let mut options = BTreeSet::new();
    options.insert((0, from, Dir::Right));

    while let Some((score, pos, dir)) = options.pop_first() {
        if pos == to {
            return score;
        }
        if visited.get(&(pos, dir)).is_none_or(|v| *v >= score) {
            visited.insert((pos, dir), score);
            if maze.contains(&(pos + dir)) {
                options.insert((score + 1, pos + dir, dir));
            }
            options.insert((score + 1000, pos, dir.left()));
            options.insert((score + 1000, pos, dir.left().left().left()));
            // forwards if not a wall
            // left
            // right
        }
    }
    panic!("Route not found")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    row: i32,
    col: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Pos {
    fn new<T: TryInto<i32>>(row: T, col: T) -> Self
    where
        <T as TryInto<i32>>::Error: Debug,
    {
        Self {
            row: row.try_into().unwrap(),
            col: col.try_into().unwrap(),
        }
    }
}

impl Dir {
    fn left(self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        }
    }
}

impl Add<Dir> for Pos {
    type Output = Pos;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::Up => Self {
                row: self.row - 1,
                col: self.col,
            },
            Dir::Right => Self {
                row: self.row,
                col: self.col + 1,
            },
            Dir::Down => Self {
                row: self.row + 1,
                col: self.col,
            },
            Dir::Left => Self {
                row: self.row,
                col: self.col - 1,
            },
        }
    }
}
