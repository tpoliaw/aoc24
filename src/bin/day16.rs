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
                .map(move |(col, _)| Pos::new(row, col))
        })
        .collect::<HashSet<_>>();
    let (score, count) = min_route(start, end, &maze);
    println!("Part 1: {score}");
    println!("Part 2: {count}");
}

fn min_route(from: Pos, to: Pos, maze: &HashSet<Pos>) -> (usize, usize) {
    let mut visited: HashMap<(Pos, Dir), (usize, Vec<(Pos, Dir)>)> = HashMap::new();
    let mut options = BTreeSet::new();
    options.insert((0, from, Dir::Right, from));

    while let Some((score, pos, dir, prev)) = options.pop_first() {
        if pos == to {
            let mut cells = HashSet::new();
            trace_visited(&visited, &(prev, dir), from, &mut cells);
            return (score, cells.len());
        }
        match visited.get_mut(&(pos, dir)) {
            Some(record) if record.0 < score => {}
            Some(record) if record.0 == score => record.1.push((prev, dir)),
            _ => {
                visited.insert((pos, dir), (score, vec![(prev, dir)]));
                if maze.contains(&(pos + dir)) && (pos + dir) != prev {
                    options.insert((score + 1, pos + dir, dir, pos));
                }
                options.insert((score + 1000, pos, dir.left(), prev));
                options.insert((score + 1000, pos, dir.left().left().left(), prev));
            }
        }
    }
    panic!("Route not found")
}

fn trace_visited(
    route: &HashMap<(Pos, Dir), (usize, Vec<(Pos, Dir)>)>,
    start: &(Pos, Dir),
    end: Pos,
    counted: &mut HashSet<Pos>,
) {
    if start.0 == end || counted.contains(&start.0) {
        return;
    }
    counted.insert(start.0);
    let prev = &route[&start].1;
    prev.iter()
        .for_each(|loc| trace_visited(route, loc, end, counted));
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
