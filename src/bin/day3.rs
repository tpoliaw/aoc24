use aoc24::input;

pub fn main() {
    let full: String = input(3).as_value();
    println!("Part 1: {}", mul_product(&full, true));
    println!("Part 2: {}", mul_product(&full, false));
}

fn mul_product(mut line: &str, force: bool) -> u32 {
    let mut total = 0;
    let mut enabled = true;
    while let Some((word, rest)) = line.split_once('(') {
        line = rest;
        if word.ends_with("mul") && (force || enabled) {
            let Some((args, rest)) = rest.split_once(')') else {
                break;
            };
            let Some((left, right)) = args.split_once(',') else {
                continue;
            };
            let Ok(left) = left.parse::<u32>() else {
                continue;
            };
            let Ok(right) = right.parse::<u32>() else {
                continue;
            };
            total += left * right;
            line = rest;
        } else if word.ends_with("do") {
            if let Some(("", _)) = rest.split_once(')') {
                enabled = true;
            }
        } else if word.ends_with("don't") {
            if let Some(("", _)) = rest.split_once(')') {
                enabled = false;
            }
        }
    }
    total
}

#[test]
fn p1() {
    let inp = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(mul_product(inp.into(), true), 161)
}
#[test]
fn p2() {
    let inp = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(mul_product(inp.into(), false), 48)
}
