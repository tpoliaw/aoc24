use std::fmt::Debug;

use aoc24::input;

pub fn main() {
    let cals = input(7)
        .map_by_line(Calibration::from_line)
        .collect::<Vec<_>>();

    let mut simple = 0;
    let mut complex = 0;
    for cal in cals {
        if cal.options(false).iter().any(|eq| eq.evaluate() == cal.aim) {
            simple += cal.aim;
        } else if cal.options(true).iter().any(|eq| eq.evaluate() == cal.aim) {
            complex += cal.aim
        }
    }
    println!("Part 1: {simple}");
    println!("Part 2: {}", simple + complex);
}

#[derive(Debug)]
struct Calibration {
    aim: u64,
    values: Vec<u64>,
}

struct Equation {
    root: u64,
    ops: Vec<Op>,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(u64),
    Mul(u64),
    Concat(u64),
}

impl Equation {
    fn evaluate(&self) -> u64 {
        let mut res = self.root;
        for op in &self.ops {
            match op {
                Op::Add(a) => res += a,
                Op::Mul(m) => res *= m,
                Op::Concat(c) => res = concat(res, *c),
            }
        }
        res
    }
    fn push(&self, op: Op) -> Self {
        let mut ops = self.ops.clone();
        ops.push(op);
        Self {
            root: self.root,
            ops,
        }
    }
}

impl Calibration {
    fn from_line<S: AsRef<str>>(line: S) -> Self {
        let line = line.as_ref();
        let (aim, args) = line.split_once(": ").unwrap();
        let aim = aim.parse().unwrap();
        let values = args
            .split(' ')
            .map(|a| a.parse())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        Self { aim, values }
    }

    fn options(&self, concat: bool) -> Vec<Equation> {
        let root = self.values[0];
        let capacity = 2usize.pow((self.values.len() - 1).try_into().unwrap());
        let mut init = Vec::with_capacity(capacity);
        let mut opts = Vec::with_capacity(capacity);
        init.push(Equation { root, ops: vec![] });
        for v in &self.values[1..] {
            opts.clear();
            for eq in init.iter() {
                opts.push(eq.push(Op::Add(*v)));
                opts.push(eq.push(Op::Mul(*v)));
                if concat {
                    opts.push(eq.push(Op::Concat(*v)))
                }
            }
            std::mem::swap(&mut init, &mut opts);
        }
        init
    }
}

impl Debug for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Eq({}", self.root)?;
        for op in &self.ops {
            match op {
                Op::Add(a) => write!(f, " + {a}")?,
                Op::Mul(m) => write!(f, " * {m}")?,
                Op::Concat(c) => write!(f, " || {c}")?,
            }
        }
        f.write_str(")")
    }

fn concat(l: u64, r: u64) -> u64 {
    let digits = r.ilog10() + 1;
    l * 10u64.pow(digits) + r
}
