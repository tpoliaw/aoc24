use std::collections::HashMap;

use aoc24::input;

pub fn main() {
    let src = input(19).as_value::<String>();
    let (towels, designs) = src.split_once("\n\n").unwrap();
    let towels = towels.split(", ").collect::<Vec<_>>();
    let designs = designs.lines().collect::<Vec<_>>();

    let mut history = HashMap::new();
    let p1 = designs
        .iter()
        .filter(|d| possible(d, &towels, &mut history) > 0)
        .count();
    println!("Part 1: {p1}");

    let p2 = designs
        .iter()
        .map(|d| possible(d, &towels, &mut history))
        .sum::<usize>();
    println!("Part 2: {p2}");
}

fn possible<'d>(design: &'d str, options: &[&str], history: &mut HashMap<&'d str, usize>) -> usize {
    if design.is_empty() {
        return 1;
    } else if let Some(v) = history.get(design) {
        return *v;
    }
    let count = options
        .iter()
        .filter_map(|o| design.strip_prefix(o))
        .map(|r| possible(r, options, history))
        .sum::<usize>();
    history.insert(design, count);
    count
}
