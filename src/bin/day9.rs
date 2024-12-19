use std::collections::{BTreeSet, VecDeque};

use aoc24::input;

pub fn main() {
    let disk = input(9).string();
    let checksum = shuffle_blocks(&disk);
    println!("Part 1: {checksum}");

    let checksum = shuffle_ids(&disk);
    println!("Part 2: {checksum}");
}

fn shuffle_blocks(disk: &str) -> usize {
    let mut disk_map = disk
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize);
    let mut mem = Vec::new();
    let mut free = VecDeque::new();
    let mut idx = 0;
    let mut id = 0;
    let mut checksum = 0;
    loop {
        let Some(len) = disk_map.next() else {
            break;
        };
        for i in idx..idx + len {
            mem.push((i, id));
        }
        idx += len;
        id += 1;
        let Some(len) = disk_map.next() else {
            break;
        };
        for i in idx..idx + len {
            free.push_back(i);
        }
        idx += len;
    }

    while free.front().is_some_and(|f| f < &mem.last().unwrap().0) {
        let tgt = free.pop_front().unwrap();
        let blk = mem.pop().unwrap();
        checksum += tgt * blk.1;
    }

    checksum += mem.iter().map(|(k, v)| k * v).sum::<usize>();
    checksum
}

fn shuffle_ids(disk: &str) -> usize {
    let mut mem = Vec::new();
    let mut free = vec![BTreeSet::new(); 10];
    let mut idx = 0;
    let mut disk_map = disk
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize);
    loop {
        let Some(len) = disk_map.next() else {
            break;
        };
        mem.push(Block { len, idx });
        idx += len;
        let Some(len) = disk_map.next() else {
            break;
        };
        free[len].insert(idx);
        idx += len;
    }
    let mut checksum = 0;
    for (id, mut blk) in mem.into_iter().enumerate().rev() {
        // For every memory span long enough for the block, find the earliest index
        // Of these, find the first, and check it is earlier than the block's current position
        let Some((len, idx)) = free
            .iter()
            .enumerate()
            .skip(blk.len as usize)
            .flat_map(|(len, opts)| opts.first().map(|idx| (len, *idx)))
            .min_by_key(|(_, idx)| *idx)
            .filter(|(_, idx)| idx < &blk.idx)
        else {
            // There is no suitable free space
            checksum += blk.checksum(id);
            continue;
        };
        // Remove the space from the free map
        free[len].remove(&idx);

        // Move the block
        blk.idx = idx;
        checksum += blk.checksum(id);

        // Add the remaining free space back to map
        if len > blk.len {
            free[len - blk.len].insert(idx + blk.len);
        }
    }
    checksum
}

#[derive(Debug)]
struct Block {
    idx: usize,
    len: usize,
}

impl Block {
    fn checksum(&self, id: usize) -> usize {
        (self.idx..self.idx + self.len)
            .map(|i| (i * id))
            .sum::<usize>()
    }
}
