use std::{collections::HashMap, mem};

use aoc24::input;

pub fn main() {
    let stones = input(11)
        .as_value::<String>()
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let count = blink(25, &stones);
    println!("Part 1: {}", count);
    let count = blink(75, &stones);
    println!("Part 2: {}", count);
}

enum Step {
    One(u64),
    Two(u64, u64),
}

fn blink(count: u32, stones: &[u64]) -> u64 {
    let mut counts: HashMap<u64, u64> = HashMap::new();
    for s in stones {
        *counts.entry(*s).or_default() += 1;
    }

    let mut next = HashMap::new();
    for _ in 0..count {
        for (s, cnt) in counts.iter() {
            match step(*s) {
                Step::One(n) => *next.entry(n).or_default() += cnt,
                Step::Two(n, m) => {
                    *next.entry(n).or_default() += cnt;
                    *next.entry(m).or_default() += cnt;
                }
            }
        }
        mem::swap(&mut counts, &mut next);
        next.clear();
    }

    counts.values().sum::<u64>()
}

fn step(stone: u64) -> Step {
    if stone == 0 {
        Step::One(1)
    } else {
        let d = stone.ilog10() + 1;
        if d % 2 == 0 {
            Step::Two(
                stone.div_euclid(10u64.pow(d / 2)),
                stone.rem_euclid(10u64.pow(d / 2)),
            )
        } else {
            Step::One(stone * 2024)
        }
    }
}
