use std::collections::{HashMap, HashSet};

use aoc24::input;

pub fn main() {
    let src = input(5).as_value::<String>();
    let (rules, updates) = src.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|ln| ln.split_once('|').unwrap())
        .map(|(l, r)| Rule(l.parse().unwrap(), r.parse().unwrap()))
        .collect::<Vec<_>>();
    let updates = updates
        .lines()
        .map(|ln| ln.split(',').map(|u| u.parse().unwrap()).collect())
        .map(Update)
        .collect::<Vec<_>>();

    let valid = updates
        .iter()
        .filter_map(|upd| upd.validate(&rules))
        .sum::<u32>();
    println!("Part 1: {valid}");

    let mut before = HashMap::<u32, HashSet<u32>>::new();
    for rule in &rules {
        before.entry(rule.0).or_default().insert(rule.1);
    }

    let invalid = updates
        .into_iter()
        .filter(|u| u.validate(&rules).is_none())
        .map(|u| u.sort(&before))
        .map(|u| u.centre())
        .sum::<u32>();

    println!("Part 2: {invalid}");
}

struct Rule(u32, u32);

struct Update(Vec<u32>);

impl Update {
    fn validate(&self, rules: &[Rule]) -> Option<u32> {
        for r in rules {
            let mut lpos = None;
            let mut rpos = None;
            for (i, u) in self.0.iter().enumerate() {
                if *u == r.0 {
                    lpos = Some(i);
                } else if *u == r.1 {
                    rpos = Some(i);
                }
            }
            if let (Some(l), Some(r)) = (lpos, rpos) {
                if l >= r {
                    return None;
                }
            }
        }
        Some(self.centre())
    }

    fn sort(self, rules: &HashMap<u32, HashSet<u32>>) -> Self {
        let mut new = Vec::new();
        let empty = HashSet::new();

        for v in self.0 {
            let before = rules.get(&v).unwrap_or_else(|| &empty);
            let idx = new
                .iter()
                .enumerate()
                .find(|(_, n)| before.contains(n))
                .map(|(i, _)| i);
            match idx {
                Some(i) => new.insert(i, v),
                None => new.push(v),
            }
        }
        Self(new)
    }

    fn centre(&self) -> u32 {
        self.0[self.0.len() / 2]
    }
}
