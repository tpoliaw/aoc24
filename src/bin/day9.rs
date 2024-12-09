use std::collections::{BTreeMap, BTreeSet};

use aoc24::input;

pub fn main() {
    let disk = input(9).as_value::<String>();
    let checksum = shuffle_blocks(&disk);
    println!("Part 1: {checksum}");

    let checksum = shuffle_ids(&disk);
    println!("Part 2: {checksum}");
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
    let mut free = BTreeMap::<u32, BTreeSet<u32>>::new();
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
        free.entry(len).or_default().insert(idx);
        idx += len;
    }
    for i in (0..id).rev() {
        let blk = mem.get_mut(&i).unwrap();
        // For every memory span long enough for the block, find the earliest index
        // Of these, find the first, and check it is earlier than the block's current position
        let Some((len, idx)) = free
            .range(blk.len..)
            .flat_map(|(len, opts)| opts.first().map(|idx| (*len, *idx)))
            .min_by_key(|(_, idx)| *idx)
            .filter(|(_, idx)| idx < &blk.idx)
        else {
            // There is no suitable free space
            continue;
        };
        // Remove the space from the free map
        free.get_mut(&len).unwrap().remove(&idx);

        // Move the block
        blk.idx = idx;

        // Add the remaining free space back to map
        if len > blk.len {
            free.entry(len - blk.len).or_default().insert(idx + blk.len);
        }
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
