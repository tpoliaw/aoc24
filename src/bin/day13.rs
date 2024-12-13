use aoc24::input;

pub fn main() {
    let src = input(13).as_value::<String>();
    let machines = src.split("\n\n").map(Machine::from).collect::<Vec<_>>();

    let p1 = machines
        .iter()
        .filter_map(|m| m.solve())
        .map(|(a, b)| 3 * a + b)
        .sum::<u64>();

    println!("Part 1: {p1:?}");

    let shift = 10000000000000.0;
    let p2 = machines
        .into_iter()
        .map(|m| m.shift(shift))
        .filter_map(|m| m.solve())
        .map(|(a, b)| 3 * a + b)
        .sum::<u64>();

    println!("Part 2: {p2:?}");
}

#[derive(Debug)]
struct Machine {
    x1: f64,
    y1: f64,

    x2: f64,
    y2: f64,

    xp: f64,
    yp: f64,
}

impl Machine {
    fn solve(&self) -> Option<(u64, u64)> {
        let Self {
            x1,
            y1,
            x2,
            y2,
            xp,
            yp,
        } = self;
        let b = (x1 * yp - y1 * xp) / (x1 * y2 - y1 * x2);
        let a = (xp - x2 * b) / x1;
        let a = (a == a.trunc()).then_some(a as u64)?;
        let b = (b == b.trunc()).then_some(b as u64)?;
        Some((a, b))
    }
    fn shift(mut self, delta: f64) -> Self {
        self.xp += delta;
        self.yp += delta;
        self
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut lines = value.split('\n').map(|ln| ln.split_once(": ").unwrap().1);
        let a = lines.next().unwrap();
        let b = lines.next().unwrap();
        let p = lines.next().unwrap();
        let (x1, y1) = parse_button(a);
        let (x2, y2) = parse_button(b);
        let (xp, yp) = parse_prize(p);
        Self {
            x1,
            y1,
            x2,
            y2,
            xp,
            yp,
        }
    }
}

fn parse_button(line: &str) -> (f64, f64) {
    let (x, y) = line.split_once(", ").unwrap();
    let x = x.split_once('+').unwrap().1.parse().unwrap();
    let y = y.split_once('+').unwrap().1.parse().unwrap();
    (x, y)
}

fn parse_prize(line: &str) -> (f64, f64) {
    let (x, y) = line.split_once(", ").unwrap();
    let x = x.split_once('=').unwrap().1.parse().unwrap();
    let y = y.split_once('=').unwrap().1.parse().unwrap();
    (x, y)
}
