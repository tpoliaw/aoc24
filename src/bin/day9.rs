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

    let mut mem = BTreeMap::<u32, Block>::new();
    let mut free = BTreeMap::new();
    let mut idx = 0;
    let mut id = 0;
    let mut disk_map = disk.trim().chars().map(|c| c.to_digit(10).unwrap());
    loop {
        let Some(len) = disk_map.next() else {
            break;
        };
        mem.insert(id, Block { len, idx });
        idx += len;
        id += 1;
        let Some(len) = disk_map.next() else {
            break;
        };
        free.insert(idx, len);
        idx += len;
    }
    // println!("{mem:#?}");
    // println!("{free:?}");
    for i in (0..id).rev() {
        let blk = mem.get_mut(&i).unwrap();
        let Some(tgt) = free
            .iter()
            .find(|(_, v)| *v >= &blk.len)
            .map(|(idx, _)| *idx)
        else {
            continue;
        };
        // println!("ID: {i}, idx: {}, len: {}, target: {tgt}", blk.idx, blk.len);
        if tgt > blk.idx {
            continue;
        }
        blk.idx = tgt;
        let space = free.remove(&tgt).unwrap();
        free.insert(tgt + blk.len, space - blk.len);
        // println!("  {free:?}");
        // println!("  {blk:?}");
    }
    // println!("{mem:#?}");
    let checksum = mem.iter().map(|(id, blk)| blk.checksum(*id)).sum::<u64>();
    println!("Part 2: {checksum}");
}

#[derive(Debug)]
struct Block {
    idx: u32,
    len: u32,
}

impl Block {
    fn checksum(&self, id: u32) -> u64 {
        (self.idx..self.idx + self.len)
            .map(|i| i as u64 * id as u64)
            .sum::<u64>()
    }
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
