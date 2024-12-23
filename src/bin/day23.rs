use std::collections::{HashMap, HashSet};

use aoc24::input;

pub fn main() {
    let src = input(23).string();
    let mut links: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in src.lines() {
        let (l, r) = line.split_once('-').unwrap();
        links.entry(l).or_default().push(r);
        links.entry(r).or_default().push(l);
    }
    let p1 = links
        .iter()
        .filter(|(n, _)| n.starts_with('t'))
        .flat_map(|(node, conns)| triples(node, conns, &links))
        .collect::<HashSet<_>>();
    println!("Part 1: {}", p1.len());
}

fn triples<'l>(
    node: &'l str,
    conns: &[&'l str],
    links: &HashMap<&'l str, Vec<&'l str>>,
) -> Vec<Triple<'l>> {
    let mut sets = Vec::new();
    for (n1, c1) in conns.iter().enumerate() {
        for c2 in &conns[n1..] {
            if links[c1].contains(c2) {
                sets.push(Triple::new(node, *c1, *c2));
            }
        }
    }
    sets
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Triple<'s>(&'s str, &'s str, &'s str);

impl<'s> Triple<'s> {
    fn new(s1: &'s str, s2: &'s str, s3: &'s str) -> Self {
        match (s1 < s2, s2 < s3, s1 < s3) {
            (true, true, true) => Self(s1, s2, s3),
            (true, true, false) => unreachable!(),
            (true, false, true) => Self(s1, s3, s2),
            (true, false, false) => Self(s3, s1, s2),
            (false, true, true) => Self(s2, s1, s3),
            (false, true, false) => Self(s2, s3, s1),
            (false, false, true) => unreachable!(),
            (false, false, false) => Self(s3, s2, s1),
        }
    }
}
