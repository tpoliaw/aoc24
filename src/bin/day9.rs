use std::collections::{BTreeMap, BTreeSet};

use aoc24::input;

pub fn main() {
    let mut mem = BTreeMap::<u64, u64>::new();
    let mut free = BTreeSet::new();
    let mut idx = 0;
    let mut id = 0;
    let disk = input(9).as_value::<String>();
    let mut disk_map = disk.trim().chars().map(|c| c.to_digit(10).unwrap() as u64);
    loop {
        let Some(len) = disk_map.next() else {
            break;
        };
        for i in idx..idx + len {
            mem.insert(i, id);
        }
        idx += len;
        id += 1;
        let Some(len) = disk_map.next() else {
            break;
        };
        for i in idx..idx + len {
            free.insert(i);
        }
        idx += len;
    }

    // show(&mem);

    while free
        .first()
        .is_some_and(|f| f < mem.last_key_value().unwrap().0)
    {
        let tgt = free.pop_first().unwrap();
        let blk = mem.pop_last().unwrap();
        mem.insert(tgt, blk.1);
    }
    // println!("last free: {:?}", free.last());
    // println!("")

    // show(&mem);

    let checksum = mem.iter().map(|(k, v)| k * v).sum::<u64>();
    println!("Part 1: {checksum}");
}

fn show(mem: &BTreeMap<u64, u64>) {
    for i in 0..=*mem.last_key_value().unwrap().0 {
        match mem.get(&i) {
            Some(b) => print!("{b}"),
            None => print!("."),
        }
    }
    println!();
}
