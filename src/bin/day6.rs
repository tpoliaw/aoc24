use std::collections::HashSet;

use aoc24::input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Floor {
    Clear,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

pub fn main() {
    let start = input(6).as_value::<String>().find('^').unwrap();
    let area = input(6)
        .map_by_line(|ln| {
            ln.chars()
                .map(|c| match c {
                    '#' => Floor::Blocked,
                    '.' | '^' => Floor::Clear,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = area[0].len();
    let height = area.len();

    let start = Pos {
        row: start.div_euclid(width + 1),
        col: start.rem_euclid(width + 1),
    };

    let mut pos = start;

    let mut dir = Dir::Up;
    let mut visited = HashSet::new();
    visited.insert(pos);
    loop {
        let Some(nxt) = dir.step(pos) else {
            break;
        };
        if nxt.col >= width || nxt.row >= height {
            break;
        }
        match area[nxt.row][nxt.col] {
            Floor::Clear => {
                visited.insert(nxt);
                pos = nxt
            }
            Floor::Blocked => dir.turn(),
        }
    }

    println!("Part 1: {}", visited.len());

    // Can't put a block in the start position
    visited.remove(&start);
    let path = visited;

    let mut options = 0;
    let mut visited = HashSet::new();
    let mut exact = HashSet::new();
    for block in path {
        let mut pos = start;
        let mut dir = Dir::Up;
        visited.clear();
        exact.clear();
        visited.insert(pos);
        exact.insert((pos, dir));
        loop {
            let Some(nxt) = dir.step(pos) else {
                break;
            };
            if nxt.col >= width || nxt.row >= height {
                break;
            }
            match area[nxt.row][nxt.col] {
                Floor::Clear if nxt != block => {
                    visited.insert(nxt);
                    if !exact.insert((nxt, dir)) {
                        options += 1;
                        break;
                    }
                    pos = nxt
                }
                _ => dir.turn(),
            }
        }
    }

    println!("Part 2: {options}");
}

impl Dir {
    fn step(self, pos: Pos) -> Option<Pos> {
        match self {
            Dir::Up => pos.row.checked_sub(1).map(|y| Pos {
                row: y,
                col: pos.col,
            }),
            Dir::Right => Some(Pos {
                row: pos.row,
                col: pos.col + 1,
            }),
            Dir::Down => Some(Pos {
                row: pos.row + 1,
                col: pos.col,
            }),
            Dir::Left => pos.col.checked_sub(1).map(|c| Pos {
                row: pos.row,
                col: c,
            }),
        }
    }
    fn turn(&mut self) {
        *self = match &self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

impl Pos {}
