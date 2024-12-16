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
    options.insert((0, from, Dir::Right, Option::<(Pos, Dir)>::None));
    let mut pb = Option::<(usize, Vec<(Pos, Dir)>)>::None;

    while let Some((score, pos, dir, prev)) = options.pop_first() {
        if pb.as_ref().is_some_and(|v| v.0 < score) {
            break;
        }
        if pos == to {
            match pb.as_mut() {
                Some((sc, _)) if *sc < score => {}
                Some((sc, prev)) if *sc == score => prev.push((pos, dir)),
                _ => pb = Some((score, vec![(pos, dir)])),
            }
        }
        match visited.get_mut(&(pos, dir)) {
            Some(record) if record.0 < score => {}
            Some(record) if record.0 == score => record.1.push(prev.unwrap()),
            _ => {
                visited.insert((pos, dir), (score, prev.into_iter().collect()));
                if maze.contains(&(pos + dir)) {
                    options.insert((score + 1, pos + dir, dir, Some((pos, dir))));
                }

                let left = dir.left();
                if maze.contains(&(pos + left)) {
                    options.insert((score + 1000, pos, left, Some((pos, dir))));
                }
                let right = dir.right();
                if maze.contains(&(pos + right)) {
                    options.insert((score + 1000, pos, right, Some((pos, dir))));
                }
            }
        }
    }
    let Some((score, prev)) = pb else {
        panic!("Route not found");
    };
    let mut cells = HashSet::new();
    for p in prev {
        trace_visited(&visited, &p, from, &mut cells);
    }
    (score, cells.len())
}

fn trace_visited(
    route: &HashMap<(Pos, Dir), (usize, Vec<(Pos, Dir)>)>,
    start: &(Pos, Dir),
    end: Pos,
    counted: &mut HashSet<Pos>,
) {
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
    fn right(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
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
