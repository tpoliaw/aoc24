use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use aoc24::input;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: i32,
    col: i32,
}

pub fn main() {
    let src = input(8).as_value::<String>();
    let width = src.find('\n').unwrap();
    let height = (src.len() / (width + 1)) as i32;
    let width = width as i32;

    println!("w: {width}, h: {height}");

    let mut antennas = HashMap::<char, Vec<Pos>>::new();
    for (loc, c) in src.chars().enumerate() {
        match c {
            '.' | '\n' => {}
            c => antennas.entry(c).or_default().push(Pos {
                row: loc as i32 / (width + 1),
                col: loc as i32 % (width + 1),
            }),
        }
    }
    // println!("{antennas:#?}");

    let mut nodes = HashSet::new();
    for (_, locs) in antennas {
        for (i, ant1) in locs.iter().enumerate() {
            for ant2 in locs[i + 1..].iter() {
                let (a1, a2) = antinodes(*ant1, *ant2);
                // println!("{ant1:?} x {ant2:?} => {a1:?} + {a2:?}");
                nodes.insert(a1);
                nodes.insert(a2);
            }
        }
    }

    // println!("{nodes:#?}");
    let count = nodes
        .iter()
        .filter(|p| 0 <= p.row && 0 <= p.col && p.row < height && p.col < width)
        .count();
    println!("Part 1: {count}");
}

fn antinodes(l1: Pos, l2: Pos) -> (Pos, Pos) {
    let dr = l2.row - l1.row;
    let dc = l2.col - l1.col;
    (
        Pos {
            row: l1.row - dr,
            col: l1.col - dc,
        },
        Pos {
            row: l2.row + dr,
            col: l2.col + dc,
        },
    )
}

#[test]
fn test_antinodes() {
    let l1 = Pos { row: 8, col: 8 };
    let l2 = Pos { row: 9, col: 9 };
    let (p1, p2) = antinodes(l1, l2);
    assert_eq!(p1, Pos { row: 7, col: 7 });
    assert_eq!(p2, Pos { row: 10, col: 10 });

    let l1 = Pos { row: 8, col: 8 };
    let l2 = Pos { row: 9, col: 9 };
    let (p1, p2) = antinodes(l2, l1);
    assert_eq!(p1, Pos { row: 10, col: 10 });
    assert_eq!(p2, Pos { row: 7, col: 7 });
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}
