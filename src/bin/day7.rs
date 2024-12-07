use std::ops::ControlFlow;

use aoc24::input;

pub fn main() {
    let cals = input(7)
        .map_by_line(Calibration::from_line)
        .collect::<Vec<_>>();

    let mut simple = 0;
    let mut complex = 0;
    for cal in cals {
        if cal.possible(Op::roll) {
            simple += cal.aim;
        } else if cal.possible(Op::roll_concat) {
            complex += cal.aim;
        }
    }
    println!("Part 1: {simple}");
    println!("Part 2: {}", simple + complex);
}

/// The original calibration values read from the input
struct Calibration {
    aim: u64,
    values: Vec<u64>,
}

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Op {
    fn eval(&self, l: u64, r: u64) -> u64 {
        match self {
            Op::Add => l + r,
            Op::Mul => l * r,
            Op::Concat => {
                let digits = r.ilog10() + 1;
                l * 10u64.pow(digits) + r
            }
        }
    }
    fn roll(&mut self) -> ControlFlow<()> {
        let (next, ovr) = match self {
            Self::Add => (Self::Mul, ControlFlow::Break(())),
            Self::Mul => (Self::Add, ControlFlow::Continue(())),
            Self::Concat => panic!("shouldn't be concat for simple rolls"),
        };
        *self = next;
        ovr
    }
    fn roll_concat(&mut self) -> ControlFlow<()> {
        let (next, ovr) = match self {
            Self::Add => (Self::Mul, ControlFlow::Break(())),
            Self::Mul => (Self::Concat, ControlFlow::Break(())),
            Self::Concat => (Self::Add, ControlFlow::Continue(())),
        };
        *self = next;
        ovr
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
    fn possible<F: Fn(&mut Op) -> ControlFlow<()>>(&self, f: F) -> bool {
        let mut ops = Some(vec![Op::Add; self.values.len() - 1]);
        while let Some(mut eq) = ops {
            let res = self.values[1..]
                .iter()
                .zip(eq.iter())
                .fold(self.values[0], |r, (v, op)| op.eval(r, *v));
            if res == self.aim {
                return true;
            }
            ops = match eq.iter_mut().try_for_each(&f) {
                ControlFlow::Continue(_) => None,
                ControlFlow::Break(_) => Some(eq),
            }
        }
        false
    }
}
