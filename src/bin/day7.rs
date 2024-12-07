use aoc24::input;

pub fn main() {
    let cals = input(7)
        .map_by_line(Calibration::from_line)
        .collect::<Vec<_>>();

    let mut simple = 0;
    let mut complex = 0;
    for cal in cals {
        if cal.possible::<Simple>() {
            simple += cal.aim;
        } else if cal.possible::<Comp>() {
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

/// A possible combination of operators to evaluate the calibration values
struct Equation<O> {
    root: u64,
    ops: Vec<O>,
}

/// Unifying trait over the two sets of operations
trait Op {
    fn init(val: u64) -> Self;
    fn eval(&self, val: u64) -> u64;
    fn roll(&mut self) -> Overflow;
}

enum Overflow {
    None,
    Carry,
}

enum Simple {
    Add(u64),
    Mul(u64),
}

enum Comp {
    Add(u64),
    Mul(u64),
    Concat(u64),
}

impl Op for Simple {
    fn init(val: u64) -> Self {
        Self::Add(val)
    }
    fn eval(&self, val: u64) -> u64 {
        match self {
            Simple::Add(a) => val + a,
            Simple::Mul(m) => val * m,
        }
    }
    fn roll(&mut self) -> Overflow {
        let (next, ovr) = match self {
            Self::Add(a) => (Self::Mul(*a), Overflow::None),
            Self::Mul(m) => (Self::Add(*m), Overflow::Carry),
        };
        *self = next;
        ovr
    }
}

impl Op for Comp {
    fn init(val: u64) -> Self {
        Self::Add(val)
    }
    fn eval(&self, val: u64) -> u64 {
        match self {
            Comp::Add(a) => val + a,
            Comp::Mul(m) => val * m,
            Comp::Concat(c) => {
                let digits = c.ilog10() + 1;
                val * 10u64.pow(digits) + c
            }
        }
    }
    fn roll(&mut self) -> Overflow {
        let (next, ovr) = match self {
            Self::Add(a) => (Self::Mul(*a), Overflow::None),
            Self::Mul(m) => (Self::Concat(*m), Overflow::None),
            Self::Concat(c) => (Self::Add(*c), Overflow::Carry),
        };
        *self = next;
        ovr
    }
}

impl<O: Op> Equation<O> {
    fn evaluate(&self) -> u64 {
        let mut res = self.root;
        for op in &self.ops {
            res = op.eval(res);
        }
        res
    }
    fn next(mut self) -> Option<Self> {
        for op in self.ops.iter_mut() {
            if let Overflow::None = op.roll() {
                return Some(self);
            }
        }
        None
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
    fn possible<O: Op>(&self) -> bool {
        let mut next = Some(self.init::<O>());
        while let Some(eq) = next {
            if eq.evaluate() == self.aim {
                return true;
            }
            next = eq.next();
        }
        false
    }
    fn init<O: Op>(&self) -> Equation<O> {
        let root = self.values[0];
        let ops = self.values[1..].iter().map(|v| O::init(*v)).collect();
        Equation { root, ops }
    }
}
