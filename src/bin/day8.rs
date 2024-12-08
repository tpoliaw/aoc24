use std::collections::{HashMap, HashSet};

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

    let mut nodes = HashSet::new();
    for locs in antennas.values() {
        for (i, a1) in locs.iter().enumerate() {
            for a2 in locs[i + 1..].iter() {
                let (n1, n2) = antinodes(*a1, *a2);
                nodes.insert(n1);
                nodes.insert(n2);
            }
        }
    }

    let count = nodes.iter().filter(|p| p.in_area(height, width)).count();
    println!("Part 1: {count}");

    let mut nodes = HashSet::new();

    for locs in antennas.values() {
        for (i, a1) in locs.iter().enumerate() {
            for a2 in locs[i + 1..].iter() {
                let (ns1, ns2) = antinode_stream(*a1, *a2);
                ns1.take_while(|p| p.in_area(height, width))
                    .for_each(|p| _ = nodes.insert(p));
                ns2.take_while(|p| p.in_area(height, width))
                    .for_each(|p| _ = nodes.insert(p));
            }
        }
    }
    println!("Part 2: {}", nodes.len());
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

fn antinode_stream(l1: Pos, l2: Pos) -> (impl Iterator<Item = Pos>, impl Iterator<Item = Pos>) {
    let dr = l2.row - l1.row;
    let dc = l2.col - l1.col;
    let (dr, dc) = reduce_hcf(dr, dc);
    (
        (0..).map(move |i| Pos {
            row: l1.row + i * dr,
            col: l1.col + i * dc,
        }),
        (1..).map(move |i| Pos {
            row: l1.row - i * dr,
            col: l1.col - i * dc,
        }),
    )
}

fn reduce_hcf(a: i32, b: i32) -> (i32, i32) {
    let (mut h, mut l) = (a.max(b), a.min(b));
    let mut rem = h.rem_euclid(l);
    while rem > 0 {
        (h, l) = (l, rem);
        rem = h.rem_euclid(l);
    }

    (a / l, b / l)
}

impl Pos {
    fn in_area(self, height: i32, width: i32) -> bool {
        0 <= self.row && self.row < height && 0 <= self.col && self.col < width
    }
}
