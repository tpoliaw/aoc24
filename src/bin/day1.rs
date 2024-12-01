use std::collections::HashMap;

use aoc24::input;

pub fn main() {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let mut freq: HashMap<i32, i32> = HashMap::new();
    for line in input(1).by_line() {
        let (l, r) = numbers(line);
        left.push(l);
        right.push(r);
        *freq.entry(r).or_default() += 1;
    }
    left.sort_unstable();
    right.sort_unstable();
    let mut err = 0;
    let mut sim = 0;
    for (l, r) in left.into_iter().zip(right.into_iter()) {
        err += (l - r).abs();
        sim += l * freq.get(&l).unwrap_or(&0);
    }
    println!("Part 1: {err}");
    println!("Part 2: {sim}");
}

fn numbers(line: String) -> (i32, i32) {
    let mut words = line.split_whitespace();
    (
        words.next().unwrap().parse().unwrap(),
        words.next().unwrap().parse().unwrap(),
    )
}
