#![allow(unused)]

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
    let codes = input(21)
        .map_by_line(|ln| {
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

    arrow_map // ^ -> <
        .iter_mut()
        .filter(|((b1, _), _)| *b1 == '^')
        .for_each(|(_, v)| v.retain(|p| p.first().is_none_or(|m| *m != Move::Left)));
    arrow_map // A -> <
        .get_mut(&('A', '<'))
        .unwrap()
        .retain(|p| *p.first().unwrap() == Move::Left);
    arrow_map // < -> ^/A
        .iter_mut()
        .filter(|((b1, _), _)| *b1 == '<')
        .for_each(|(_, v)| v.retain(|p| p.first().is_none_or(|m| *m != Move::Up)));

    let mut complex = 0;

    for (num, code) in codes {
        let mut all = vec![];
        let codes = paths(&code, &digit_map);
        for code in codes {
            let codes = paths(&code, &arrow_map);
            for code in codes {
                let codes = paths(&code, &arrow_map);
                all.extend(codes);
            }
        }
        let min = all.iter().min_by_key(|c| c.len()).unwrap();
        complex += min.len() * num;
    }
    println!("Part 1: {complex}");
}

fn paths(code: &[char], pairs: &HashMap<(char, char), Vec<Vec<Move>>>) -> Vec<Vec<char>> {
    let mut prev = 'A';
    let mut paths: Vec<Vec<Move>> = vec![vec![]];
    for b in code {
        let seqs = &pairs[&(prev, *b)];
        match &seqs[..] {
            [] => {}
            [one] => paths.iter_mut().for_each(|p| p.extend(one)),
            [one, two] => {
                paths = paths
                    .into_iter()
                    .flat_map(|mut p| {
                        let mut left = p.clone();
                        left.extend(one);
                        p.extend(two);
                        [p, left]
                    })
                    .collect()
            }
            _ => panic!(),
        }
        paths.iter_mut().for_each(|p| p.push(Move::Press));
        prev = *b;
    }
    paths
        .into_iter()
        .map(|p| p.into_iter().map(|m| m.to_char()).collect())
        .collect()
}

fn map_pairs(buttons: &HashMap<char, (u8, u8)>) -> HashMap<(char, char), Vec<Vec<Move>>> {
    let mut pairs = HashMap::new();
    for (b1, (r1, c1)) in buttons {
        for (b2, (r2, c2)) in buttons {
            if b1 == b2 {
                pairs.insert((*b1, *b2), vec![]);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Right,
    Up,
    Down,
    Left,
    Press,
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
}
