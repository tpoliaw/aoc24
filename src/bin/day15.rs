use std::collections::HashMap;

use aoc24::input;

pub fn main() {
    let src = input(15).as_value::<String>();
    let (area, moves) = src.split_once("\n\n").unwrap();

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

    let p1 = run_single(area, &moves);
    println!("Part 1: {p1}");
    let p2 = run_double(area, &moves);
    println!("Part 2: {p2}");
}

fn run_single(area: &str, moves: &[Dir]) -> i32 {
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

    for step in moves {
        shift(&mut bot, &mut area, *step);
    }

    area.iter()
        .filter(|(_, v)| *v == &Obj::Crate)
        .map(|(k, _)| k.gps())
        .sum::<i32>()
}

fn run_double(area: &str, moves: &[Dir]) -> i32 {
    let width = area.find('\n').unwrap();
    let bot = area.find('@').unwrap();
    let mut bot = Pos::new(bot / (width + 1), (bot % (width + 1)) * 2);

    let mut area = area
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| match c {
                    '#' => Some([
                        (Pos::new(row, col * 2), Obj::Wall),
                        (Pos::new(row, col * 2 + 1), Obj::Wall),
                    ]),
                    'O' => Some([
                        (Pos::new(row, col * 2), Obj::HalfLeft),
                        (Pos::new(row, col * 2 + 1), Obj::HalfRight),
                    ]),
                    _ => None,
                })
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    for step in moves {
        shift_double(&mut bot, &mut area, *step);
    }
    area.iter()
        .filter(|(_, v)| *v == &Obj::HalfLeft)
        .map(|(k, _)| k.gps())
        .sum::<i32>()
}

fn shift_double(bot: &mut Pos, area: &mut HashMap<Pos, Obj>, step: Dir) {
    let dest = bot.step(step);
    let front = match area.get(&dest) {
        Some(Obj::HalfLeft) if step.is_vert() => vec![dest, dest.step(Dir::Right)],
        Some(Obj::HalfRight) if step.is_vert() => vec![dest, dest.step(Dir::Left)],
        Some(Obj::Wall) => return,
        Some(_) => vec![dest],
        None => vec![],
    };
    if front.is_empty() || shift_all(front, area, step) {
        *bot = dest;
    }
}

fn shift_all(front: Vec<Pos>, area: &mut HashMap<Pos, Obj>, step: Dir) -> bool {
    let mut next = HashMap::new();
    for p in &front {
        let nxt = p.step(step);
        match area.get(&nxt) {
            Some(Obj::Wall) => {
                next.insert(nxt, Obj::Wall);
                break;
            }
            Some(Obj::HalfLeft) if step.is_vert() => {
                next.insert(nxt, Obj::HalfLeft);
                next.insert(nxt.step(Dir::Right), Obj::HalfRight);
            }
            Some(Obj::HalfRight) if step.is_vert() => {
                next.insert(nxt, Obj::HalfRight);
                next.insert(nxt.step(Dir::Left), Obj::HalfLeft);
            }
            Some(o) => _ = next.insert(nxt, *o),
            None => {}
        }
    }
    if next.values().any(|o| o == &Obj::Wall) {
        false
    } else if next.is_empty() {
        for p in front {
            let obj = area.remove(&p).unwrap();
            area.insert(p.step(step), obj);
        }
        true
    } else if shift_all(next.keys().cloned().collect(), area, step) {
        for p in front {
            let obj = area.remove(&p).unwrap();
            area.insert(p.step(step), obj);
        }
        true
    } else {
        false
    }
}

fn shift(bot: &mut Pos, area: &mut HashMap<Pos, Obj>, step: Dir) {
    let mut dest = bot.step(step);
    let tgt = loop {
        match area.get(&dest) {
            Some(Obj::Wall) => break None,
            Some(Obj::Crate) => dest = dest.step(step),
            Some(_) => panic!(""),
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
    HalfLeft,
    HalfRight,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    fn is_vert(&self) -> bool {
        matches!(self, Dir::Up | Dir::Down)
    }
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
