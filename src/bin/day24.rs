#![allow(unused)]
use std::collections::HashMap;

use aoc24::input;

pub fn main() {
    let src = input(24).string();
    // let src = sample_2();

    let (inputs, gates) = src.split_once("\n\n").unwrap();
    // let inputs = inputs.to_string();
    // let gates = gates.to_string();
    let inputs = inputs
        .lines()
        .map(|ln| ln.split_once(": ").unwrap())
        .map(|(k, v)| (k.to_string(), v == "1"))
        .collect::<HashMap<_, _>>();

    let gates = gates
        .lines()
        .map(Gate::from_line)
        .collect::<HashMap<_, _>>();
    let mut wires = inputs
        .into_iter()
        .map(|(wire, state)| (wire, Wire::Resolved(state)))
        .chain(
            gates
                .into_iter()
                .map(|(wire, gate)| (wire, Wire::Gate(gate))),
        )
        .collect::<HashMap<_, _>>();
    let mut value: u64 = 0;
    for i in 0.. {
        let z = format!("z{i:02}");
        let Some(z) = resolve(z, &mut wires) else {
            break;
        };
        if z {
            value += 1 << i;
        }
    }
    // println!("{wires:#?}");
    println!("Part 1: {value}");
}

#[derive(Debug)]
struct Gate {
    left: String,
    right: String,
    op: Op,
}

#[derive(Debug)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
enum Wire {
    Resolved(bool),
    Gate(Gate),
}

impl Gate {
    fn from_line(ln: &str) -> (String, Self) {
        let (inp, out) = ln.split_once(" -> ").unwrap();
        let mut words = inp.split_whitespace();
        let left = words.next().unwrap().into();
        let op = words.next().unwrap();
        let right = words.next().unwrap().into();
        let gate = match op {
            "AND" => Gate {
                left,
                right,
                op: Op::And,
            },
            "OR" => Gate {
                left,
                right,
                op: Op::Or,
            },
            "XOR" => Gate {
                left,
                right,
                op: Op::Xor,
            },
            _ => panic!(),
        };
        (out.to_string(), gate)
    }
}

// impl Wire {
fn resolve(name: String, wires: &mut HashMap<String, Wire>) -> Option<bool> {
    let wire = wires.remove(&name)?;
    let v = match wire {
        Wire::Resolved(s) => s,
        Wire::Gate(g) => {
            let Gate { left, right, op } = g;
            let left = resolve(left, wires).unwrap();
            let right = resolve(right, wires).unwrap();
            // let left = wires.remove(left).unwrap().resolve(wires);
            // let right = wires.remove(right).unwrap().resolve(wires);

            match op {
                Op::And => left && right,
                Op::Or => left || right,
                Op::Xor => left ^ right,
            }
        }
    };
    wires.insert(name, Wire::Resolved(v));
    Some(v)
}
// }
