use std::collections::HashMap;

use aoc24::input;

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
//
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

pub fn main() {
    let src = input(21).string();
    let codes = src
        .lines()
        .map(|ln| (ln[..ln.len() - 1].parse::<usize>().unwrap(), ln))
        .collect::<Vec<_>>();

    let digits = [
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let arrows = [
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let mut digit_map = map_pairs(&digits);
    let mut arrow_map = map_pairs(&arrows);

    // Pairs are mapped with moves grouped and sorted left to right on the key pad, the pairs where
    // this is not possible due to the blank space need to be modified manually to go the 'wrong'
    // way.
    *digit_map.get_mut(&('7', '0')).unwrap() = ">vvv".to_string();
    *digit_map.get_mut(&('7', 'A')).unwrap() = ">>vvv".to_string();
    *digit_map.get_mut(&('4', '0')).unwrap() = ">vv".to_string();
    *digit_map.get_mut(&('4', 'A')).unwrap() = ">>vv".to_string();
    *digit_map.get_mut(&('1', '0')).unwrap() = ">v".to_string();
    *digit_map.get_mut(&('1', 'A')).unwrap() = ">>v".to_string();

    *digit_map.get_mut(&('0', '7')).unwrap() = "^^^<".to_string();
    *digit_map.get_mut(&('A', '7')).unwrap() = "^^^<<".to_string();
    *digit_map.get_mut(&('0', '4')).unwrap() = "^^<".to_string();
    *digit_map.get_mut(&('A', '4')).unwrap() = "^^<<".to_string();
    *digit_map.get_mut(&('0', '1')).unwrap() = "^<".to_string();
    *digit_map.get_mut(&('A', '1')).unwrap() = "^<<".to_string();

    *arrow_map.get_mut(&('^', '<')).unwrap() = "v<".to_string();
    *arrow_map.get_mut(&('A', '<')).unwrap() = "v<<".to_string();

    *arrow_map.get_mut(&('<', '^')).unwrap() = ">^".to_string();
    *arrow_map.get_mut(&('<', 'A')).unwrap() = ">>^".to_string();

    let codes = codes
        .into_iter()
        .map(|(k, v)| (k, paths(&v, &digit_map, &mut HashMap::new())))
        .collect::<Vec<_>>();

    let mut pair_cache: HashMap<String, Vec<String>> = Default::default();
    let mut len_cache = HashMap::new();

    let mut complex = 0;
    for (num, code) in &codes {
        let mut count = 0;
        for segment in code {
            let len = chained(segment, &arrow_map, &mut pair_cache, &mut len_cache, 2);
            count += len;
        }
        complex += count * num;
    }
    println!("Part 1: {complex}");

    let mut complex = 0;
    for (num, code) in &codes {
        let mut count = 0;
        for segment in code {
            let len = chained(segment, &arrow_map, &mut pair_cache, &mut len_cache, 25);
            count += len;
        }
        complex += count * num;
    }
    println!("Part 2: {complex}");
}

fn chained(
    code: &str,
    pairs: &HashMap<(char, char), String>,
    pair_cache: &mut HashMap<String, Vec<String>>,
    len_cache: &mut HashMap<String, HashMap<usize, usize>>,
    steps: usize,
) -> usize {
    if steps == 0 {
        return code.len();
    }
    if let Some(l) = len_cache.get(code).and_then(|l| l.get(&steps)) {
        return *l;
    }
    let segments = paths(code, pairs, pair_cache);
    let len = segments
        .iter()
        .map(|s| chained(&s, pairs, pair_cache, len_cache, steps - 1))
        .sum();
    len_cache.entry(code.into()).or_default().insert(steps, len);
    len
}

fn paths(
    code: &str,
    pairs: &HashMap<(char, char), String>,
    cache: &mut HashMap<String, Vec<String>>,
) -> Vec<String> {
    if let Some(p) = cache.get(code) {
        return p.clone();
    }
    let mut prev = 'A';
    let mut paths: Vec<String> = vec![];
    for b in code.chars() {
        let seq = &pairs[&(prev, b)];
        paths.push(seq.clone());
        prev = b;
    }
    paths.iter_mut().for_each(|p| p.push('A'));
    cache.insert(code.into(), paths.clone());
    paths
}

fn map_pairs(buttons: &HashMap<char, (u8, u8)>) -> HashMap<(char, char), String> {
    let mut pairs = HashMap::new();
    for (b1, (r1, c1)) in buttons {
        for (b2, (r2, c2)) in buttons {
            if b1 == b2 {
                pairs.insert((*b1, *b2), String::new());
                continue;
            }

            let mut pair = String::new();
            if c2 < c1 {
                (0..(c1 - c2)).for_each(|_| pair.push('<'));
            }
            if r1 < r2 {
                (0..(r2 - r1)).for_each(|_| pair.push('v'));
            } else if r2 < r1 {
                (0..(r1 - r2)).for_each(|_| pair.push('^'));
            }
            if c1 < c2 {
                (0..(c2 - c1)).for_each(|_| pair.push('>'));
            }
            pairs.insert((*b1, *b2), pair);
        }
    }
    pairs
}
