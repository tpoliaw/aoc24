use std::collections::{HashMap, HashSet};

use aoc24::input;

pub fn main() {
    let src = input(12).as_value::<String>();
    let regions = build_regions(&src);
    println!(
        "Part 1: {}",
        regions.iter().map(|r| r.price()).sum::<usize>()
    );

    println!(
        "Part 2: {}",
        regions.iter().map(|r| r.discount_price()).sum::<usize>()
    );
}

fn build_regions(src: &str) -> Vec<Region> {
    let mut uncharted = src
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, plot)| (Plot::new(row as i32, col as i32), plot))
        })
        .collect::<HashMap<_, _>>();
    let width = src.find('\n').unwrap() as i32;
    let height = src.len() as i32 / (width + 1);

    let regions = (0..width)
        .flat_map(|col| (0..height).map(move |row| Plot::new(row, col)))
        .flat_map(|p| map_region(p, &mut uncharted))
        .collect::<Vec<_>>();
    regions
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Plot {
    row: i32,
    col: i32,
}

#[derive(Default)]
struct Region {
    plots: HashSet<Plot>,
    perimeter: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Side {
    Left,
    Top,
    Right,
    Bottom,
}

fn map_region(start: Plot, uncharted: &mut HashMap<Plot, char>) -> Option<Region> {
    let kind = uncharted.remove(&start)?;
    let mut region = Region::default();
    region.push(start);
    let mut next = start.adjacent().to_vec();
    while !next.is_empty() {
        let p = next.pop().unwrap();
        if uncharted.get(&p) == Some(&kind) {
            uncharted.remove(&p);
            region.push(p);
            next.extend(p.adjacent());
        } else if !region.plots.contains(&p) {
            region.perimeter += 1;
        }
    }

    Some(region)
}
impl Plot {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
    fn mv(self, m: Side) -> Plot {
        let (row, col) = match m {
            Side::Left => (self.row, self.col - 1),
            Side::Right => (self.row, self.col + 1),
            Side::Top => (self.row - 1, self.col),
            Side::Bottom => (self.row + 1, self.col),
        };
        Self { row, col }
    }
    fn adjacent(&self) -> [Plot; 4] {
        [
            self.mv(Side::Top),
            self.mv(Side::Bottom),
            self.mv(Side::Left),
            self.mv(Side::Right),
        ]
    }
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }
    fn push(&mut self, plot: Plot) {
        self.plots.insert(plot);
    }
    fn price(&self) -> usize {
        self.perimeter * self.area()
    }
    fn discount_price(&self) -> usize {
        self.area() * self.sides()
    }
    fn edge_pieces(&self) -> HashSet<(Plot, Side)> {
        let mut edges = HashSet::new();
        for p in self.plots.iter() {
            for dir in [Side::Left, Side::Right, Side::Top, Side::Bottom] {
                if !self.plots.contains(&p.mv(dir)) {
                    edges.insert((*p, dir));
                }
            }
        }
        edges
    }

    fn sides(&self) -> usize {
        let mut sides = 0;
        let mut edges = self.edge_pieces();
        while let Some((mut cell, mut edge)) = edges.iter().next() {
            let start = (cell, edge);
            loop {
                edges.remove(&(cell, edge));
                let (turn, cont) = match edge {
                    Side::Left => (Side::Bottom, Side::Top),
                    Side::Top => (Side::Left, Side::Right),
                    Side::Right => (Side::Top, Side::Bottom),
                    Side::Bottom => (Side::Right, Side::Left),
                };
                let adj = cell.mv(cont);
                let diag = cell.mv(cont).mv(edge);
                let prev = edge;
                (cell, edge) = match (self.plots.contains(&adj), self.plots.contains(&diag)) {
                    (true, true) => (diag, turn),
                    (true, false) => (adj, edge),
                    (false, _) => (cell, cont),
                };
                if edge != prev {
                    sides += 1;
                }
                if (cell, edge) == start {
                    break;
                }
            }
        }
        sides
    }
}
