#![allow(unused)]
use std::collections::{BTreeSet, HashMap, HashSet};

use aoc24::input;

pub fn main() {
    let src = input(23).string();
    let mut links: HashMap<&str, BTreeSet<&str>> = HashMap::new();
    for line in src.lines() {
        let (l, r) = line.split_once('-').unwrap();
        links.entry(l).or_default().insert(r);
        links.entry(r).or_default().insert(l);
    }
    let p1 = links
        .iter()
        .filter(|(n, _)| n.starts_with('t'))
        .flat_map(|(node, conns)| triples(node, conns, &links))
        .collect::<BTreeSet<_>>();
    println!("Part 1: {}", p1.len());

    let mut cache = HashMap::<BTreeSet<&str>, BTreeSet<&str>>::new();

    let cluster = links
        .keys()
        .map(|node| max_cluster([*node].into(), &links, &mut cache))
        .max_by_key(|cl| cl.len())
        .unwrap();
    let mut cluster = cluster.into_iter().collect::<Vec<_>>();
    cluster.sort();
    let p2 = cluster.join(",");
    println!("Part 2: {p2}");
}

fn triples<'l>(
    node: &'l str,
    conns: &BTreeSet<&'l str>,
    links: &HashMap<&'l str, BTreeSet<&'l str>>,
) -> Vec<Triple<'l>> {
    let mut sets = Vec::new();
    for c1 in conns.iter() {
        for c2 in conns.iter() {
            if links[c1].contains(c2) {
                sets.push(Triple::new(node, *c1, *c2));
            }
        }
    }
    sets
}

fn max_cluster<'l, 'c>(
    nodes: BTreeSet<&'l str>,
    links: &HashMap<&'l str, BTreeSet<&'l str>>,
    cache: &'c mut HashMap<BTreeSet<&'l str>, BTreeSet<&'l str>>,
) -> BTreeSet<&'l str> {
    if let Some(cluster) = cache.get(&nodes) {
        return cluster.clone();
    }
    let mut max = None;
    // println!("max_cluster: {nodes:?}");
    for node in common(&nodes, &links) {
        // println!("    common: {node}");
        let next = nodes.iter().map(|m| *m).chain(Some(node)).collect();
        let cluster = max_cluster(next, links, cache);
        if cluster.len() > nodes.len() {
            max = Some(cluster);
        }
    }
    let cluster = match max {
        Some(mx) => mx.clone(),
        None => nodes.clone(),
    };
    cache.insert(nodes, cluster.clone());
    cluster
}

fn common<'l>(
    current: &BTreeSet<&'l str>,
    links: &HashMap<&'l str, BTreeSet<&'l str>>,
) -> BTreeSet<&'l str> {
    current
        .into_iter()
        .map(|n| &links[n])
        // .inspect(|ln| println!("      {ln:?}"))
        // .reduce(|l, r| l.intersection(&r).cloned().collect::<HashSet<_>>())
        .fold(Option::<BTreeSet<&str>>::None, |l, r| {
            match l {
                Some(l) => Some(l.intersection(r).cloned().collect()),
                None => Some(r.clone()),
            }
            // l.intersection(r).map(|m| *m).collect()
        })
        .unwrap()
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[test]
fn test_common() {
    let links = [
        ("ab", ["cd", "ef", "gh"].into()),
        ("cd", ["ab"].into()),
        ("ef", ["ab"].into()),
        ("gh", ["ab"].into()),
    ]
    .into();

    let com = common(&["ab"].into(), &links);
    assert_eq!(com, ["cd", "ef", "gh"].into());

    assert_eq!(common(&["cd", "ef", "gh"].into(), &links), ["ab"].into())
}
