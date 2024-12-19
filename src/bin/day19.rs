use std::collections::HashMap;

use aoc24::input;

pub fn main() {
    let src = input(19).as_value::<String>();
    let (towels, designs) = src.split_once("\n\n").unwrap();
    let towels = towels.split(", ").collect::<Vec<_>>();
    let designs = designs.lines().collect::<Vec<_>>();

    let p1 = designs
        .iter()
        .filter(|d| possible(d, &towels, &mut HashMap::new()) > 0)
        .count();
    println!("Part 1: {p1}");

    let p2 = designs
        .iter()
        .map(|d| possible(d, &towels, &mut HashMap::new()))
        .sum::<usize>();
    println!("Part 2: {p2}");
}

fn possible<'d>(design: &'d str, options: &[&str], history: &mut HashMap<&'d str, usize>) -> usize {
    if design.is_empty() {
        return 1;
    } else if let Some(v) = history.get(design) {
        return *v;
    }
    let mut count = 0;
    for opt in options {
        if design.starts_with(opt) {
            count += possible(&design[opt.len()..], options, history);
        }
    }
    history.insert(design, count);
    count
}
