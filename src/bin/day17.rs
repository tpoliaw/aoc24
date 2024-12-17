use aoc24::input;

pub fn main() {
    let src = input(17).as_value::<String>();
    let (regs, ops) = src.split_once("\n\n").unwrap();
    let mut regs = regs
        .lines()
        .map(|ln| ln.split_once(": ").unwrap().1.parse().unwrap());
    let a = regs.next().unwrap();
    let b = regs.next().unwrap();
    let c = regs.next().unwrap();

    let ops: Vec<u8> = ops
        .trim()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|op| op.parse().unwrap())
        .collect();

    let mut mach = Machine {
        a,
        b,
        c,
        ops: ops.clone(),
        inst: 0,
        stdout: vec![],
    };
    mach.run();
    println!(
        "Part 1: {}",
        mach.stdout
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    // 0 => adv
    // 1 => bxl
    // 2 => bst
    // 3 => jnz
    // 4 => bxc
    // 5 => out
    // 6 => bdv
    // 7 => cdv
    //
    // (2,4), (1,5), (7,5), (1,6), (4,3), (5,5), (0,3), (3,0)
    // 2,4 => set b to a%8
    // 1,5 => set b to b ^ 5
    //        5 == 101
    //        b = match a % 8 {
    //          0 => 5,
    //          1 => 4,
    //          2 => 7,
    //          3 => 6,
    //          4 => 1,
    //          5 => 0,
    //          6 => 3,
    //          7 => 2
    //        }
    // 7,5 => set c to a / 2**b
    //        c = match a % 8 {
    //          0 => a/2**5 = a/32 = a >> 5
    //          1 => a/2**4 = a/16 = a >> 4,
    //          2 => a/2**7 = a/128 = a >> 7,
    //          3 => a/2**6 = a/64 = a >> 6,
    //          4 => a/2**1 = a/2 = a >> 1,
    //          5 => a/2**0 = a = a,
    //          6 => a/2**3 = a/8 = a >> 3,
    //          7 => a/2**2 = a/4 = a >> 2,
    //        }
    // 1,6 => set b to b^6 == b^110
    //        b = match a % 8 {
    //          0 => 3,
    //          1 => 2,
    //          2 => 1,
    //          3 => 0,
    //          4 => 7,
    //          5 => 6,
    //          6 => 5,
    //          7 => 4,
    //        }
    // 4,3 => set b to b^c
    //        b = match a % 8 {
    //          0 => 3^(a >> 5), =>3
    //          1 => 2^(a >> 4), =>2
    //          2 => 1^(a >> 7), =>1
    //          3 => 0^(a >> 6), =>0
    //          4 => 7^(a >> 1), =>5
    //          5 => 6^(a), == 3, =>3
    //          6 => 5^(a >> 3), =>5
    //          7 => 4^(a >> 2), =>5
    //        }
    // 5,5 => output b
    // 0,3 => set a to a/8 == a >> 3
    //        For 16 outputs 8**15 <= a < 8**16
    // 3,0 => if a> 0 jump to 0, else exit
    //

    let mut previous = vec![0];
    for op in ops.iter().rev() {
        previous = digit(*op, previous);
    }
    println!("Part 2: {}", previous.iter().min().unwrap());
}

fn digit(req: u8, previous: Vec<i64>) -> Vec<i64> {
    previous
        .iter()
        .flat_map(|p| {
            (0..8)
                .map(move |c| ((p << 3) + c))
                .filter(move |c| calc(*c) == req)
        })
        .collect()
}

/// The first output from a given starting a value
fn calc(a: i64) -> u8 {
    (match a % 8 {
        0 => 3 ^ (a >> 5),
        1 => 2 ^ (a >> 4),
        2 => 1 ^ (a >> 7),
        3 => 0 ^ (a >> 6),
        4 => 7 ^ (a >> 1),
        5 => 6 ^ (a),
        6 => 5 ^ (a >> 3),
        7 => 4 ^ (a >> 2),
        _ => unreachable!(),
    } % 8) as u8
}

#[derive(Debug)]
struct Machine {
    a: i64,
    b: i64,
    c: i64,
    ops: Vec<u8>,
    inst: usize,
    stdout: Vec<u8>,
}

impl Machine {
    fn run(&mut self) {
        while self.inst < self.ops.len() {
            let op = self.ops[self.inst];
            match op {
                0 => {
                    // ADV
                    let a = self.a;
                    let d = self.operand();
                    self.a = a / 2_i64.pow(d as u32);
                }
                1 => {
                    // BXL
                    let b = self.b;
                    let v = self.ops[self.inst + 1];
                    self.b = b ^ v as i64;
                }
                2 => {
                    // BST
                    let b = self.operand();
                    self.b = b % 8;
                }
                3 => {
                    // JNZ
                    if self.a != 0 {
                        self.inst = self.ops[self.inst + 1] as usize;
                        continue;
                    }
                }
                4 => {
                    // BXC
                    self.b = self.b ^ self.c;
                }
                5 => {
                    // OUT
                    let v = self.operand();
                    self.stdout.push((v % 8) as u8);
                }
                6 => {
                    // BDV
                    let a = self.a;
                    let d = self.operand();
                    self.b = a / 2_i64.pow(d as u32);
                }
                7 => {
                    // CDV
                    let a = self.a;
                    let d = self.operand();
                    self.c = a / 2_i64.pow(d as u32);
                }
                x => panic!("Invalid opcode: {x}"),
            }
            self.inst += 2;
        }
    }
    fn operand(&self) -> i64 {
        match self.ops[self.inst + 1] {
            op @ 0..=3 => op.into(),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            x => panic!("Invalid operand: {x}"),
        }
    }
}
