use std::collections::HashMap;

use aoc24::input;

pub fn main() {
    let src = input(15).as_value::<String>();
    let (area, moves) = src.split_once("\n\n").unwrap();

    let width = area.find('\n').unwrap();
    let bot = area.find('@').unwrap();
    let mut bot = Pos::new(bot / (width + 1), bot % (width + 1));

    let mut area = area
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| match c {
                    '#' => Some((Pos::new(row, col), Obj::Wall)),
                    'O' => Some((Pos::new(row, col), Obj::Crate)),
                    _ => None,
                })
        })
        .collect::<HashMap<_, _>>();

    let moves = moves
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '>' => Dir::Right,
            '<' => Dir::Left,
            '^' => Dir::Up,
            'v' => Dir::Down,
            _ => panic!("Unexpected move: {c}"),
        })
        .collect::<Vec<_>>();

    for step in moves {
        shift(&mut bot, &mut area, step);
    }

    let p1 = area
        .iter()
        .filter(|(_, v)| *v == &Obj::Crate)
        .map(|(k, _)| k.gps())
        .sum::<i32>();
    println!("Part 1: {p1}");
}

fn shift(bot: &mut Pos, area: &mut HashMap<Pos, Obj>, step: Dir) {
    let mut dest = bot.step(step);
    let tgt = loop {
        match area.get(&dest) {
            Some(Obj::Wall) => break None,
            Some(Obj::Crate) => dest = dest.step(step),
            None => break Some(dest),
        }
    };
    if let Some(space) = tgt {
        *bot = bot.step(step);
        if let Some(_) = area.remove(&*bot) {
            area.insert(space, Obj::Crate);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    row: i16,
    col: i16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Obj {
    Wall,
    Crate,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self {
            col: col as i16,
            row: row as i16,
        }
    }
    fn step(self, step: Dir) -> Self {
        match step {
            Dir::Up => Self {
                row: self.row - 1,
                col: self.col,
            },
            Dir::Down => Self {
                row: self.row + 1,
                col: self.col,
            },
            Dir::Left => Self {
                row: self.row,
                col: self.col - 1,
            },
            Dir::Right => Self {
                row: self.row,
                col: self.col + 1,
            },
        }
    }
    fn gps(&self) -> i32 {
        self.row as i32 * 100 + self.col as i32
    }
}
