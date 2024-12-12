use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use aoc24::input;

pub fn main() {
    let src = input(12).as_value::<String>();
    let src = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;
    let mut uncharted = src
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(col, plot)| {
                (
                    Plot {
                        row: row as i32,
                        col: col as i32,
                    },
                    plot,
                )
            })
        })
        .collect::<HashMap<_, _>>();
    let width = src.find('\n').unwrap();
    let height = src.len() / (width + 1);
    // println!("w: {width}, h: {height}");

    let regions = (0..width)
        .flat_map(|col| {
            (0..height).map(move |row| Plot {
                row: row as i32,
                col: col as i32,
            })
        })
        .flat_map(|p| map_region(p, &mut uncharted))
        .collect::<Vec<_>>();
    println!(
        "Part 1: {}",
        regions.iter().map(|r| r.price()).sum::<usize>()
    );

    let r = Region {
        plots: [Plot { row: 1, col: 1 }].into(),
        perimeter: 4,
        init: Plot { row: 1, col: 1 },
    };

    println!("{}", r.sides());

    println!(
        "Part 2: {}",
        regions.iter().map(|r| r.discount_price()).sum::<usize>()
    );
    // 615076 too low
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Plot {
    row: i32,
    col: i32,
}

struct Region {
    init: Plot,
    plots: HashSet<Plot>,
    perimeter: usize,
}

fn map_region(start: Plot, uncharted: &mut HashMap<Plot, char>) -> Option<Region> {
    let kind = uncharted.remove(&start)?;
    let mut region = Region::new(start);
    let mut next = start.steps().collect::<Vec<_>>();
    while !next.is_empty() {
        let p = next.pop().unwrap();
        if uncharted.get(&p) == Some(&kind) {
            uncharted.remove(&p);
            region.push(p);
            // region.perimeter += p.edges();
            next.extend(p.steps());
        } else if !region.plots.contains(&p) {
            region.perimeter += 1;
        }
    }

    println!("{kind}: {region:?}, {}", region.sides());
    Some(region)
}

#[derive(Clone, Copy)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}
impl Plot {
    fn mv(self, m: Move) -> Plot {
        let (row, col) = match m {
            Move::Left => (self.row, self.col - 1),
            Move::Right => (self.row, self.col + 1),
            Move::Up => (self.row - 1, self.col),
            Move::Down => (self.row + 1, self.col),
        };

        Self { row, col }
    }
    fn adjacent(&self) -> [Plot; 4] {
        let up = Plot {
            row: self.row - 1,
            col: self.col,
        };
        let down = Plot {
            row: self.row + 1,
            col: self.col,
        };
        let left = Plot {
            row: self.row,
            col: self.col - 1,
        };
        let right = Plot {
            row: self.row,
            col: self.col + 1,
        };
        [up, down, left, right]
    }
    fn steps(&self) -> impl Iterator<Item = Plot> {
        self.adjacent().into_iter()
    }

    // fn edges(&self) -> usize {
    //     self.adjacent().into_iter().filter(|p| p.is_none()).count()
    // }
}

impl Region {
    fn new(plot: Plot) -> Self {
        Self {
            init: plot,
            plots: HashSet::from([plot]),
            perimeter: 0,
        }
    }
    fn push(&mut self, plot: Plot) {
        self.plots.insert(plot);
    }
    fn price(&self) -> usize {
        self.perimeter * self.plots.len()
    }
    fn discount_price(&self) -> usize {
        self.plots.len() * self.sides()
    }
    fn sides(&self) -> usize {
        let mut sides = 1;
        let mut cell = self.init;
        // Move right until we meet an edge
        while self.plots.contains(&cell.mv(Move::Right)) {
            cell.col += 1;
        }
        let start = cell;
        let mut edge = Side::Right;
        loop {
            // println!("cell: {cell:?}, edge: {edge:?}");
            match edge {
                Side::Left => {
                    let u = self.plots.contains(&cell.mv(Move::Up));
                    let ul = self.plots.contains(&cell.mv(Move::Up).mv(Move::Left));
                    (cell, edge) = match (u, ul) {
                        (true, true) => (cell.mv(Move::Up).mv(Move::Left), Side::Bottom),
                        (true, false) => (cell.mv(Move::Up), Side::Left),
                        (false, _) => (cell, Side::Top),
                    };
                    if edge != Side::Left {
                        sides += 1;
                    }
                }
                Side::Top => {
                    let r = self.plots.contains(&cell.mv(Move::Right));
                    let ru = self.plots.contains(&cell.mv(Move::Right).mv(Move::Up));
                    (cell, edge) = match (r, ru) {
                        (true, true) => (cell.mv(Move::Right).mv(Move::Up), Side::Left),
                        (true, false) => (cell.mv(Move::Right), Side::Top),
                        (false, _) => (cell, Side::Right),
                    };
                    if edge != Side::Top {
                        sides += 1;
                    }
                }
                Side::Right => {
                    let d = self.plots.contains(&cell.mv(Move::Down));
                    let dr = self.plots.contains(&cell.mv(Move::Down).mv(Move::Right));
                    (cell, edge) = match (d, dr) {
                        (true, true) => (cell.mv(Move::Down).mv(Move::Right), Side::Top),
                        (true, false) => (cell.mv(Move::Down), Side::Right),
                        (false, _) => (cell, Side::Bottom),
                    };
                    if edge != Side::Right {
                        sides += 1;
                    }
                }
                Side::Bottom => {
                    let l = self.plots.contains(&cell.mv(Move::Left));
                    let ld = self.plots.contains(&cell.mv(Move::Left).mv(Move::Down));
                    (cell, edge) = match (l, ld) {
                        (true, true) => (cell.mv(Move::Left).mv(Move::Down), Side::Right),
                        (true, false) => (cell.mv(Move::Left), Side::Bottom),
                        (false, _) => (cell, Side::Left),
                    };
                    if edge != Side::Bottom {
                        sides += 1;
                    }
                }
            }
            // println!("    {cell:?}, {edge:?}");
            if cell == start {
                match edge {
                    Side::Left => sides += 2,
                    Side::Top => sides += 1,
                    Side::Right => {}
                    Side::Bottom => sides += 2,
                }
                break;
            }
        }

        sides
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Left,
    Top,
    Right,
    Bottom,
}

impl Side {
    // turn left while following perimeter clockwise
    fn turn_left(self) -> Self {
        match self {
            Side::Left => Self::Bottom,
            Side::Top => Self::Left,
            Side::Right => Self::Top,
            Side::Bottom => Self::Right,
        }
    }
}

impl Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Region{{ area: {}, perimeter: {} }}",
            self.plots.len(),
            self.perimeter
        )
    }
}
