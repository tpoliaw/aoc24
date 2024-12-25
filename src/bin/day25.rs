use aoc24::input;

pub fn main() {
    let src = input(25).string();
    let locks = src
        .split("\n\n")
        .filter(|blk| blk.starts_with('#'))
        .map(|blk| Lock::new(blk))
        .collect::<Vec<_>>();
    let keys = src
        .split("\n\n")
        .filter(|blk| blk.starts_with('.'))
        .map(|blk| Key::new(blk))
        .collect::<Vec<_>>();
    let mut count = 0;

    for key in &keys {
        for lock in &locks {
            let h = lock.5 .0;
            if key.0 + lock.0 > h
                || key.1 + lock.1 > h
                || key.2 + lock.2 > h
                || key.3 + lock.3 > h
                || key.4 + lock.4 > h
            {
                continue;
            }
            count += 1;
        }
    }

    println!("Part 1: {count}");
}

#[derive(Debug, Clone, Copy)]
struct Space(u8);

#[derive(Debug)]
struct Lock(u8, u8, u8, u8, u8, Space);

#[derive(Debug)]
struct Key(u8, u8, u8, u8, u8);

impl Lock {
    fn new(blk: &str) -> Self {
        let rows = Space((blk.lines().count() as u8) - 2);
        let mut lock = Lock(0, 0, 0, 0, 0, rows);
        for row in blk.lines().skip(1) {
            let mut c = row.chars();
            if c.next().unwrap() == '#' {
                lock.0 += 1;
            }
            if c.next().unwrap() == '#' {
                lock.1 += 1;
            }
            if c.next().unwrap() == '#' {
                lock.2 += 1;
            }
            if c.next().unwrap() == '#' {
                lock.3 += 1;
            }
            if c.next().unwrap() == '#' {
                lock.4 += 1;
            }
        }
        lock
    }
}

impl Key {
    fn new(blk: &str) -> Self {
        let rows = (blk.lines().count() as u8) - 1;
        let mut key = Key(rows, rows, rows, rows, rows);
        for row in blk.lines().take(rows as usize) {
            let mut c = row.chars();
            if c.next().unwrap() == '.' {
                key.0 -= 1;
            }
            if c.next().unwrap() == '.' {
                key.1 -= 1;
            }
            if c.next().unwrap() == '.' {
                key.2 -= 1;
            }
            if c.next().unwrap() == '.' {
                key.3 -= 1;
            }
            if c.next().unwrap() == '.' {
                key.4 -= 1;
            }
        }
        key
    }
}
