use std::collections::HashMap;

use aoc24::input;

fn main() {
    let secrets = input(22)
        .map_by_line(|ln| SecNum(ln.parse().unwrap()))
        .collect::<Vec<_>>();
    let p1 = secrets
        .iter()
        .cloned()
        .map(|mut sn| {
            for _ in 0..2000 {
                sn = sn.next();
            }
            sn.0
        })
        .sum::<u64>();
    println!("Part 1: {p1}");

    let seqs = secrets
        .iter()
        .map(|sn| sn.prices(2000))
        .map(first_appearances)
        .collect::<Vec<_>>();

    let totals = seqs
        .into_iter()
        .fold(HashMap::new(), |base, other| merge(base, other));
    let p2 = totals.values().max().unwrap();
    println!("Part 2: {p2}");
}

fn merge(mut base: HashMap<Seq, usize>, other: HashMap<Seq, i8>) -> HashMap<Seq, usize> {
    for (k, v) in other.into_iter() {
        *base.entry(k).or_default() += v as usize;
    }
    base
}

fn first_appearances(prices: Vec<(i8, i8)>) -> HashMap<Seq, i8> {
    let mut seq = Seq(prices[0].1, prices[1].1, prices[2].1, prices[3].1);
    let mut seqs = HashMap::new();
    seqs.entry(seq).or_insert(prices[3].0);

    for (p, d) in &prices[4..] {
        seq = seq.push(*d);
        seqs.entry(seq).or_insert(*p);
    }

    seqs
}

#[derive(Clone, Copy)]
struct SecNum(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Seq(i8, i8, i8, i8);

impl SecNum {
    fn mix(self, other: u64) -> Self {
        Self(self.0 ^ other)
    }
    fn prune(self) -> Self {
        Self(self.0 % 16777216)
    }
    fn next(self) -> Self {
        let p1 = self.mix(self.0 * 64).prune();
        let p2 = p1.mix(p1.0 >> 5).prune();
        p2.mix(p2.0 * 2048).prune()
    }
    fn prices(mut self, len: usize) -> Vec<(i8, i8)> {
        let mut prices = Vec::with_capacity(len);
        let mut prev = (self.0 % 10) as i8;
        for _ in 0..len {
            self = self.next();
            let p = (self.0 % 10) as i8;
            let d = p - prev;
            prev = p;
            prices.push((p, d));
        }
        prices
    }
}

impl Seq {
    fn push(self, next: i8) -> Self {
        Self(self.1, self.2, self.3, next)
    }
}
