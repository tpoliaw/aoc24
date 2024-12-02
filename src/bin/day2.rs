use aoc24::input;
pub fn main() {
    let data = input(2).by_line().map(|l| numbers(&l)).collect::<Vec<_>>();
    let simple = data.iter().filter(|ln| safe(ln, None)).count();
    println!("Part 1: {simple}");
    let damp = data
        .iter()
        .filter(|ln| safe(ln, None) || dampened(ln))
        .count();
    println!("Part 2: {damp}");
}

fn numbers(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn safe(line: &[i32], skip: Option<usize>) -> bool {
    let mut inc = false;
    let mut dec = false;
    for i in 1..line.len() {
        let prev = match skip {
            Some(s) if s == i => continue,
            Some(0) if i == 1 => continue,
            Some(s) if s == i - 1 => i - 2,
            Some(_) => i - 1,
            None => i - 1,
        };
        match line[i] - line[prev] {
            -3..0 => dec = true,
            0 => return false, // duplicated to silence clippy
            1..4 => inc = true,
            _ => return false,
        }
        if inc && dec {
            return false;
        }
    }
    true
}

fn dampened(line: &[i32]) -> bool {
    (0..line.len()).any(|i| safe(line, Some(i)))
}
