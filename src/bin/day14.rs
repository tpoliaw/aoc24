use aoc24::input;

pub fn main() {
    let mut robots = input(14)
        .map_by_line(|ln| Robot::new(&ln))
        .collect::<Vec<_>>();
    for _ in 0..100 {
        robots.iter_mut().for_each(|r| r.step((101, 103)));
    }
    let mut quads = [0; 4];
    for r in robots {
        match (r.x, r.y) {
            (0..50, 0..51) => quads[0] += 1,
            (0..50, 52..) => quads[1] += 1,
            (51.., 0..51) => quads[2] += 1,
            (51.., 52..) => quads[3] += 1,
            (50, _) | (_, 51) => {}
        }
    }
    let p1 = quads[0] * quads[1] * quads[2] * quads[3];
    println!("Part 1: {p1}");
}

#[derive(Debug)]
struct Robot {
    x: u16,
    y: u16,
    dx: i16,
    dy: i16,
}

impl Robot {
    fn new(line: &str) -> Self {
        // p=0,4 v=3,-3
        let (pos, vel) = line.split_once(' ').unwrap();
        let (x, y) = pos.strip_prefix("p=").unwrap().split_once(',').unwrap();
        let (dx, dy) = vel.strip_prefix("v=").unwrap().split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            dx: dx.parse().unwrap(),
            dy: dy.parse().unwrap(),
        }
    }
    fn step(&mut self, bounds: (i16, i16)) {
        self.x = self.dx.wrapping_add_unsigned(self.x).rem_euclid(bounds.0) as u16;
        self.y = self.dy.wrapping_add_unsigned(self.y).rem_euclid(bounds.1) as u16;
    }
}
