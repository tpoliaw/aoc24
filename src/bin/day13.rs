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

    let p2 = machines
        .into_iter()
        .map(|m| m.shift(10_000_000_000_000.0))
        .filter_map(|m| m.solve())
        .map(|(a, b)| 3 * a + b)
        .sum::<u64>();

    println!("Part 2: {p2:?}");
}

#[derive(Debug)]
struct Machine {
    // Button A
    x1: f64,
    y1: f64,

    // Button B
    x2: f64,
    y2: f64,

    // Prize location
    xp: f64,
    yp: f64,
}

impl Machine {
    /// Determine number of A/B button presses required if an integer solution exists
    fn solve(&self) -> Option<(u64, u64)> {
        let b = (self.x1 * self.yp - self.y1 * self.xp) / (self.x1 * self.y2 - self.y1 * self.x2);
        let a = (self.xp - self.x2 * b) / self.x1;
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
        let mut lines = value.split('\n');
        let (x1, y1) = parse_line(lines.next().unwrap(), '+');
        let (x2, y2) = parse_line(lines.next().unwrap(), '+');
        let (xp, yp) = parse_line(lines.next().unwrap(), '=');
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

fn parse_line(line: &str, sep: char) -> (f64, f64) {
    let (_, line) = line.split_once(": ").unwrap();
    let (l, r) = line.split_once(", ").unwrap();
    let l = l.split_once(sep).unwrap().1.parse().unwrap();
    let r = r.split_once(sep).unwrap().1.parse().unwrap();
    (l, r)
}
