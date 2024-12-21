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
        .map(|ln| {
            (
                ln[..ln.len() - 1].parse::<usize>().unwrap(),
                ln.chars().collect::<Vec<_>>(),
            )
        })
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
    *digit_map.get_mut(&('7', '0')).unwrap() =
        vec![Move::Right, Move::Down, Move::Down, Move::Down];
    *digit_map.get_mut(&('7', 'A')).unwrap() =
        vec![Move::Right, Move::Right, Move::Down, Move::Down, Move::Down];
    *digit_map.get_mut(&('4', '0')).unwrap() = vec![Move::Right, Move::Down, Move::Down];
    *digit_map.get_mut(&('4', 'A')).unwrap() =
        vec![Move::Right, Move::Right, Move::Down, Move::Down];
    *digit_map.get_mut(&('1', '0')).unwrap() = vec![Move::Right, Move::Down];
    *digit_map.get_mut(&('1', 'A')).unwrap() = vec![Move::Right, Move::Right, Move::Down];

    *digit_map.get_mut(&('0', '7')).unwrap() = vec![Move::Up, Move::Up, Move::Up, Move::Left];
    *digit_map.get_mut(&('A', '7')).unwrap() =
        vec![Move::Up, Move::Up, Move::Up, Move::Left, Move::Left];
    *digit_map.get_mut(&('0', '4')).unwrap() = vec![Move::Up, Move::Up, Move::Left];
    *digit_map.get_mut(&('A', '4')).unwrap() = vec![Move::Up, Move::Up, Move::Left, Move::Left];
    *digit_map.get_mut(&('0', '1')).unwrap() = vec![Move::Up, Move::Left];
    *digit_map.get_mut(&('A', '1')).unwrap() = vec![Move::Up, Move::Left, Move::Left];

    *arrow_map.get_mut(&('^', '<')).unwrap() = vec![Move::Down, Move::Left];
    *arrow_map.get_mut(&('A', '<')).unwrap() = vec![Move::Down, Move::Left, Move::Left];

    *arrow_map.get_mut(&('<', '^')).unwrap() = vec![Move::Right, Move::Up];
    *arrow_map.get_mut(&('<', 'A')).unwrap() = vec![Move::Right, Move::Right, Move::Up];

    let codes = codes
        .into_iter()
        .map(|(k, v)| (k, paths(&v, &digit_map, &mut HashMap::new())))
        .collect::<Vec<_>>();

    let mut pair_cache: HashMap<Vec<char>, Vec<Vec<char>>> = Default::default();
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
    code: &[char],
    pairs: &HashMap<(char, char), Vec<Move>>,
    pair_cache: &mut HashMap<Vec<char>, Vec<Vec<char>>>,
    len_cache: &mut HashMap<(Vec<char>, usize), usize>,
    steps: usize,
) -> usize {
    if steps == 0 {
        return code.len();
    }
    if let Some(l) = len_cache.get(&(code.to_vec(), steps)) {
        return *l;
    }
    let segments = paths(code, pairs, pair_cache);
    let len = segments
        .iter()
        .map(|s| chained(&s, pairs, pair_cache, len_cache, steps - 1))
        .sum();
    len_cache.insert((code.to_vec(), steps), len);
    len
}

fn paths(
    code: &[char],
    pairs: &HashMap<(char, char), Vec<Move>>,
    cache: &mut HashMap<Vec<char>, Vec<Vec<char>>>,
) -> Vec<Vec<char>> {
    if let Some(p) = cache.get(code) {
        return p.clone();
    }
    let mut prev = 'A';
    let mut paths: Vec<Vec<Move>> = vec![];
    for b in code {
        let seq = &pairs[&(prev, *b)];
        paths.push(seq.clone());
        prev = *b;
    }
    paths.iter_mut().for_each(|p| p.push(Move::Press));
    let paths: Vec<Vec<char>> = paths
        .into_iter()
        .map(|p| p.into_iter().map(|m| m.to_char()).collect())
        .collect();
    cache.insert(code.into(), paths.clone());
    paths
}

fn map_pairs(buttons: &HashMap<char, (u8, u8)>) -> HashMap<(char, char), Vec<Move>> {
    let mut pairs = HashMap::new();
    for (b1, (r1, c1)) in buttons {
        for (b2, (r2, c2)) in buttons {
            if b1 == b2 {
                pairs.insert((*b1, *b2), vec![]);
                continue;
            }

            let mut pair = vec![];
            if c2 < c1 {
                (0..(c1 - c2)).for_each(|_| pair.push(Move::Left));
            }
            if r1 < r2 {
                (0..(r2 - r1)).for_each(|_| pair.push(Move::Down));
            } else if r2 < r1 {
                (0..(r1 - r2)).for_each(|_| pair.push(Move::Up));
            }
            if c1 < c2 {
                (0..(c2 - c1)).for_each(|_| pair.push(Move::Right));
            }
            pairs.insert((*b1, *b2), pair);
        }
    }
    pairs
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Press = 0,
    Left = 1,
    Down = 2,
    Up = 3,
    Right = 4,
}

impl Move {
    fn to_char(self) -> char {
        match self {
            Move::Right => '>',
            Move::Up => '^',
            Move::Down => 'v',
            Move::Left => '<',
            Move::Press => 'A',
        }
    }
    fn from_char(c: char) -> Self {
        match c {
            '>' => Self::Right,
            '<' => Self::Left,
            'v' => Self::Down,
            '^' => Self::Up,
            'A' => Self::Press,
            _ => panic!(),
        }
    }
}
