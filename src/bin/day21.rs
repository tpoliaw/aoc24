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

    let mut digit_map = map_pairs(&digits);

    digit_map
        .iter_mut()
        .filter(|((b1, _), _)| *b1 == '0')
        .for_each(|(_, v)| v.retain(|p| p.first().is_none_or(|m| *m != Move::Left)));
    digit_map
        .iter_mut()
        .filter(|((b1, _), _)| *b1 == 'A')
        .for_each(|(_, v)| {
            v.retain(|p| {
                p.first().is_none_or(|m| *m != Move::Left)
                    || p.get(1).is_none_or(|m| *m != Move::Left)
            })
        });

    digit_map
        .iter_mut()
        .filter(|((b1, _), _)| *b1 == '1')
        .for_each(|(_, v)| v.retain(|p| p.first().is_none_or(|m| *m != Move::Down)));
    digit_map
        .iter_mut()
        .filter(|((b1, _), _)| *b1 == '4')
        .for_each(|(_, v)| {
            v.retain(|p| {
                p.first().is_none_or(|m| *m != Move::Down)
                    || p.get(1).is_none_or(|m| *m != Move::Down)
            })
        });
    digit_map
        .iter_mut()
        .filter(|((b1, _), _)| *b1 == '7')
        .for_each(|(_, v)| {
            v.retain(|p| {
                p.first().is_none_or(|m| *m != Move::Down)
                    || p.get(1).is_none_or(|m| *m != Move::Down)
                    || p.get(2).is_none_or(|m| *m != Move::Down)
            })
        });
    let digit_map = digit_map
        .into_iter()
        .map(|(k, mut v)| {
            (k, {
                v.sort_unstable();
                v.remove(0)
            })
        })
        .collect::<HashMap<(char, char), Vec<Move>>>();

    // moves should be grouped and sorted left to right where possible, the blank space means this
    // isn't always possible
    let arrow_map: HashMap<(char, char), Vec<Move>> = [
        // A -> ?
        (('A', 'A'), ""),
        (('A', '<'), "v<<"), // blank space
        (('A', 'v'), "<v"),
        (('A', '>'), "v"),
        (('A', '^'), "<"),
        // < -> ?
        (('<', 'A'), ">>^"), // blank space
        (('<', '<'), ""),
        (('<', 'v'), ">"),
        (('<', '^'), ">^"), // blank space
        // v -> ?
        (('v', 'A'), "^>"),
        (('v', '<'), "<"),
        (('v', 'v'), ""),
        (('v', '>'), ">"),
        // > -> ?
        (('>', 'A'), "^"),
        (('>', 'v'), "<"),
        (('>', '>'), ""),
        (('>', '^'), "<^"),
        // ^ -> ?
        (('^', 'A'), ">"),
        (('^', '<'), "v<"),
        (('^', '>'), "v>"), // blank space
        (('^', '^'), ""),
    ]
    .into_iter()
    .map(|(p, v)| (p, v.chars().map(Move::from_char).collect::<Vec<_>>()))
    .collect();

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

fn map_pairs(buttons: &HashMap<char, (u8, u8)>) -> HashMap<(char, char), Vec<Vec<Move>>> {
    let mut pairs = HashMap::new();
    for (b1, (r1, c1)) in buttons {
        for (b2, (r2, c2)) in buttons {
            if b1 == b2 {
                pairs.insert((*b1, *b2), vec![vec![]]);
                continue;
            }

            let mut cols = vec![];
            if c1 < c2 {
                (0..(c2 - c1)).for_each(|_| cols.push(Move::Right));
            } else if c2 < c1 {
                (0..(c1 - c2)).for_each(|_| cols.push(Move::Left));
            }
            let mut rows = vec![];
            if r1 < r2 {
                (0..(r2 - r1)).for_each(|_| rows.push(Move::Down));
            } else if r2 < r1 {
                (0..(r1 - r2)).for_each(|_| rows.push(Move::Up));
            }

            if rows.is_empty() {
                pairs.insert((*b1, *b2), vec![cols]);
            } else if cols.is_empty() {
                pairs.insert((*b1, *b2), vec![rows]);
            } else {
                let h = rows.iter().chain(cols.iter()).cloned().collect();
                let v = cols.iter().chain(rows.iter()).cloned().collect();
                pairs.insert((*b1, *b2), vec![h, v]);
            }
        }
    }
    pairs
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
