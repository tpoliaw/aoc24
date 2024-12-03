use aoc24::input;

pub fn main() {
    let total: u32 = input(3).map_by_line(valid_products).sum();
    println!("Part 1: {total}");
    let full: String = input(3).as_value();
    let cond = cond_valid_products(&full);
    println!("Part 2: {cond}");
}

fn cond_valid_products(mut line: &str) -> u32 {
    let mut enabled = true;
    let mut total = 0;
    loop {
        // println!("\n{line}");
        let (word, rest) = read_word(line);
        if rest.is_empty() {
            break;
        }
        if word.is_empty() {
            line = &line[1..];
            continue;
        }
        line = rest;
        if word.ends_with("do") {
            match read_paren(rest) {
                Ok(rest) => {
                    // println!("    enabled");
                    enabled = true;
                    line = rest;
                }
                Err(rest) => {
                    line = rest;
                }
            }
        } else if word.ends_with("don't") {
            match read_paren(rest) {
                Ok(rest) => {
                    // println!("    disabled");
                    enabled = false;
                    line = rest;
                }
                Err(rest) => {
                    line = rest;
                }
            }
        } else if word.ends_with("mul") && enabled {
            let Ok(rest) = read_single(rest, '(') else {
                continue;
            };
            let Ok((left, rest)) = read_number(rest) else {
                continue;
            };
            let Ok(rest) = read_single(rest, ',') else {
                continue;
            };
            let Ok((right, rest)) = read_number(rest) else {
                continue;
            };
            let Ok(_) = read_single(rest, ')') else {
                continue;
            };
            // println!("    mul({left}, {right})");
            total += left * right;
        }
    }
    total
}

fn read_single(line: &str, c: char) -> Result<&str, &str> {
    match line.chars().next() {
        Some(v) if v == c => Ok(&line[1..]),
        _ => Err(line),
    }
}

fn read_paren(line: &str) -> Result<&str, &str> {
    match &line[..2] {
        "()" => Ok(&line[2..]),
        _ => Err(&line),
    }
}

fn read_word(line: &str) -> (String, &str) {
    let mut chars = line.chars();
    let mut word = String::new();
    while let Some(c) = chars.next() {
        if c.is_alphabetic() || c == '\'' {
            word.push(c);
        } else {
            break;
        }
    }
    let l = word.len();
    (word, &line[l..])
}

fn read_number(line: &str) -> Result<(u32, &str), &str> {
    let mut chars = line.chars();
    let mut n = 0;
    let mut len = 0;
    while let Some(c) = chars.next() {
        match c.to_digit(10) {
            Some(d) => {
                n = n * 10 + d;
                len += 1;
            }
            None => break,
        }
    }
    match len {
        0 => Err(line),
        l => Ok((n, &line[l..])),
    }
}

fn valid_products(line: String) -> u32 {
    // println!("\n{line:?}");
    let mut line = line.as_str();
    let mut total = 0;
    while let Some(idx) = line.find("mul") {
        // println!("\n{line:?}, {idx}");
        line = &line[idx + 3..];
        if let Some((l, r)) = read_mul(line) {
            total += l * r;
        }
    }
    // println!("     {total}");
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
    // println!("    {left}, {right}");
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
fn p1() {
    let inp = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(valid_products(inp.into()), 161)
}
#[test]
fn p2() {
    let inp = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(cond_valid_products(inp.into()), 48)
}
