use aoc24::input;

pub fn main() {
    let src = input(17).string();
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
    // 7,5 => set c to a / 2**b
    // 1,6 => set b to b^6 == b^110
    // 4,3 => set b to b^c
    // 5,5 => output b
    // 0,3 => set a to a/8 == a >> 3
    //        For 16 outputs 8**15 <= a < 8**16
    // 3,0 => if a> 0 jump to 0, else exit

    let mut options = vec![0];
    for op in ops.iter().rev() {
        options = options_for_digit(*op, options);
    }
    println!("Part 2: {}", options.iter().min().unwrap());
}

/// Generate the numbers that will produce the required number while not affecting the previous
/// numbers
fn options_for_digit(req: u8, previous: Vec<i64>) -> Vec<i64> {
    previous
        .iter()
        .flat_map(|p| (0..8).map(|c| ((*p << 3) + c)).filter(|c| calc(*c) == req))
        .collect()
}

/// The first output from a given starting a value
fn calc(a: i64) -> u8 {
    let b = (a % 8) ^ 5;
    (((b ^ 6) ^ (a >> b)) % 8) as u8
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
                    let d = self.operand();
                    self.a = self.a / 2_i64.pow(d as u32);
                }
                1 => {
                    // BXL
                    let v = self.ops[self.inst + 1];
                    self.b = self.b ^ v as i64;
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
                    let d = self.operand();
                    self.b = self.a / 2_i64.pow(d as u32);
                }
                7 => {
                    // CDV
                    let d = self.operand();
                    self.c = self.a / 2_i64.pow(d as u32);
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
