use aoc24::input;
pub fn main() {
    let data = input(2).map_by_line(|l| numbers(&l)).collect::<Vec<_>>();
    let simple = data.iter().filter(|ln| safe(ln.iter())).count();
    println!("Part 1: {simple}");
    let damp = data
        .iter()
        .filter(|ln| safe(ln.iter()) || dampened(ln))
        .count();
    println!("Part 2: {damp}");
}

fn numbers(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn safe<'a>(mut line: impl Iterator<Item = &'a i32>) -> bool {
    let mut inc = false;
    let mut dec = false;
    let mut prev = line.next().expect("Empty iterator");
    for v in line {
        match v - prev {
            -3..0 if !inc => dec = true,
            1..4 if !dec => inc = true,
            0 | _ => return false, // explicit 0 to silence clippy
        }
        prev = v;
    }
    true
}

fn dampened(line: &[i32]) -> bool {
    (0..line.len()).any(|i| {
        safe(
            line.iter()
                .enumerate()
                .filter(|(n, _)| *n != i)
                .map(|(_, v)| v),
        )
    })
}
