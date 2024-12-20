use std::{collections::HashSet, ops::Add};

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

    let width = src.find('\n').unwrap();
    let start = src.find('S').unwrap();
    let start = Pos {
        row: (start / (width + 1)) as i32,
        col: (start % (width + 1)) as i32,
    };

    let track = plot_track(start, &walls);

    let p1 = count_cheats(&track, 2);
    println!("Part 1: {p1}");

    let p2 = count_cheats(&track, 20);
    println!("Part 2: {p2}");
}

fn count_cheats(track: &[Pos], skips: usize) -> usize {
    let mut cheats = 0;

    for (dist, cell) in track.iter().enumerate() {
        let mut check = dist + 100;
        while check < track.len() {
            let end = track[check];
            let d = cell.dist(&end);
            if d <= skips && dist + d + 100 <= check {
                cheats += 1;
            }
            check += d.saturating_sub(skips).max(1);
        }
    }
    cheats
}

fn plot_track(start: Pos, walls: &HashSet<Pos>) -> Vec<Pos> {
    let mut prev = start;
    let mut next = Some(start);
    let mut track = vec![];
    while let Some(pos) = next {
        if walls.contains(&pos) {
            continue;
        }
        track.push(pos);
        next = Dir::each()
            .map(|dr| pos + dr)
            .filter(|c| c != &prev && !walls.contains(c))
            .next();
        prev = pos;
    }
    track
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
    fn dist(&self, other: &Self) -> usize {
        (self.row.abs_diff(other.row) + self.col.abs_diff(other.col)) as usize
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
