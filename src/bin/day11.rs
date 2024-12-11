use aoc24::input;

pub fn main() {
    let mut stones = input(11)
        .as_value::<String>()
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    blink(25, &mut stones);
    println!("Part 1: {}", stones.len());
}

fn blink(count: u64, stones: &mut Vec<u64>) {
    let mut next = Vec::new();
    for _ in 0..count {
        for s in stones.iter() {
            if *s == 0 {
                next.push(1);
            } else {
                let d = digits(*s);
                if d % 2 == 0 {
                    next.push(s.div_euclid(10u64.pow(d / 2)));
                    next.push(s.rem_euclid(10u64.pow(d / 2)));
                } else {
                    next.push(*s * 2024);
                }
            }
        }
        stones.clear();
        std::mem::swap(stones, &mut next);
    }
}

fn digits(n: u64) -> u32 {
    n.ilog10() + 1
}
