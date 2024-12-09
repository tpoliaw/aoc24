use std::collections::{BTreeMap, BTreeSet};

use aoc24::input;

pub fn main() {
    let disk = input(9).as_value::<String>();
    let s = std::time::Instant::now();
    let checksum = shuffle_blocks(&disk);
    println!("Part 1: {checksum}");
    println!("Part 1: {:?}", s.elapsed());

    let s = std::time::Instant::now();
    let checksum = shuffle_ids(&disk);
    println!("Part 2: {checksum}");
    println!("Part 2: {:?}", s.elapsed());
}

fn shuffle_blocks(disk: &str) -> u64 {
    let mut disk_map = disk.trim().chars().map(|c| c.to_digit(10).unwrap() as u64);
    let mut mem = BTreeMap::<u64, u64>::new();
    let mut free = BTreeSet::new();
    let mut idx = 0;
    let mut id = 0;
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

    while free
        .first()
        .is_some_and(|f| f < mem.last_key_value().unwrap().0)
    {
        let tgt = free.pop_first().unwrap();
        let blk = mem.pop_last().unwrap();
        mem.insert(tgt, blk.1);
    }

    let checksum = mem.iter().map(|(k, v)| k * v).sum::<u64>();
    checksum
}

fn shuffle_ids(disk: &str) -> u64 {
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
    for i in (0..id).rev() {
        let blk = mem.get_mut(&i).unwrap();
        let Some(tgt) = free
            .iter()
            .find(|(_, v)| *v >= &blk.len)
            .map(|(idx, _)| *idx)
            .filter(|idx| idx < &blk.idx)
        else {
            continue;
        };
        blk.idx = tgt;
        let space = free.remove(&tgt).unwrap();
        free.insert(tgt + blk.len, space - blk.len);
    }
    let checksum = mem.iter().map(|(id, blk)| blk.checksum(*id)).sum::<u64>();
    checksum
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
