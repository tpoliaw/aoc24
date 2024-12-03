use std::fs;

use aoc24::input;

pub fn main() {
    let total: u32 = input(3).map_by_line(valid_products).sum();
    println!("Part 1: {total}");
}

fn valid_products(line: String) -> u32 {
    println!("\n{line:?}");
    let mut line = line.as_str();
    let mut total = 0;
    while let Some(idx) = line.find("mul") {
        // println!("\n{line:?}, {idx}");
        line = &line[idx + 3..];
        if let Some((l, r)) = read_mul(line) {
            total += l * r;
        }
    }
    println!("     {total}");
    total
}

fn read_mul(line: &str) -> Option<(u32, u32)> {
    // println!("    {line}");
    let mut chars = line.chars();
    let _ = chars.next().filter(|c| *c == '(')?;
    // println!("    open");
    let (left, c) = read_num(&mut chars)?;
    // println!("    left: {left}");
    if c != ',' {
        return None;
    }
    // println!("    comma");
    let (right, c) = read_num(&mut chars)?;
    // println!("    right: {right}");
    if c != ')' {
        return None;
    }
    // println!("    close");
    println!("    {left}, {right}");
    Some((left, right))
}

fn read_num(chars: &mut impl Iterator<Item = char>) -> Option<(u32, char)> {
    let mut num = 0;
    let mut found = 0;
    let mut chars = chars.by_ref().take(4);
    while let Some(c) = chars.next() {
        if let Some(d) = c.to_digit(10) {
            num = num * 10 + d;
            found += 1;
        } else {
            if 0 < found && found <= 3 {
                return Some((num, c));
            } else {
                return None;
            }
        }
    }
    None
}

#[test]
fn sample() {
    let inp = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(valid_products(inp.into()), 161)
}
