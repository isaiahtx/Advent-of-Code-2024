use crate::graph::shortest_path_multiple_tgts;
use crate::utils::LinesIterator;
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Output {
    None,
    Some(usize),
    Halt,
}

impl Output {
    const fn to_option(self) -> Option<usize> {
        if let Self::Some(x) = self {
            Some(x)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    program: Vec<usize>,
    ip: usize,
}

impl Computer {
    const fn combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand"),
        }
    }

    // fn print_program(&self) {
    //     for (i, opcode) in self.program.iter().enumerate().filter(|(i, _)| *i % 2 == 0) {
    //         match *opcode {
    //             // adv: Divide A by 2.pow(combo), store to A (truncate).
    //             0 => println!("adv {}", self.program[i + 1]),
    //             // bxl: Bitwise XOR B with literal, store to B.
    //             1 => println!("bxl {}", self.program[i + 1]),
    //             // bst: B = combo modulo 8.
    //             2 => println!("bst {}", self.program[i + 1]),
    //             // jnz: Do nothing if A == 0, otherwise jump to operand literal.
    //             // Only increment instruction pointer if doesn't jump.
    //             3 => println!("jnz {}", self.program[i + 1]),
    //             // bxc: B = B XOR C.
    //             4 => println!("bxc"),
    //             // out: Output combo operand.
    //             5 => println!("out {}", self.program[i + 1]),
    //             // bdv: adv but store to B instead of A.
    //             6 => println!("bdv {}", self.program[i + 1]),
    //             // cdv: adv but store to B instead of A.
    //             7 => println!("cdv {}", self.program[i + 1]),
    //             _ => panic!("Invalid opcode"),
    //         }
    //     }
    // }

    /// Returns true if the program halts.
    #[allow(clippy::cast_possible_truncation)]
    fn step(&mut self) -> Output {
        if self.ip >= self.program.len() {
            return Output::Halt;
        }

        let opcode = self.program[self.ip];
        let operand = self.program[self.ip + 1];

        match opcode {
            // adv: Divide A by 2.pow(combo), store to A (truncate).
            0 => {
                // println!("adv {operand} (combo: {})", self.combo(operand));
                self.a >>= self.combo(operand);
                self.ip += 2;
            }

            // bxl: Bitwise XOR B with literal, store to B.
            1 => {
                // println!("bxl {operand}");
                self.b ^= operand;
                self.ip += 2;
            }

            // bst: B = combo modulo 8.
            2 => {
                // println!("bst {operand} (combo: {})", self.combo(operand));
                self.b = self.combo(operand) & 7;
                self.ip += 2;
            }

            // jnz: Do nothing if A == 0, otherwise jump to operand literal.
            // Only increment instruction pointer if doesn't jump.
            3 => {
                // println!("jnz {operand}\n");
                if self.a != 0 {
                    self.ip = operand;
                } else {
                    self.ip += 2;
                }
            }

            // bxc: B = B XOR C.
            4 => {
                // println!("bxc");
                self.b ^= self.c;
                self.ip += 2;
            }

            // out: Output combo operand.
            5 => {
                // println!("out {operand} (combo: {})", self.combo(operand));
                self.ip += 2;
                return Output::Some(self.combo(operand) & 7);
            }

            // bdv: adv but store to B instead of A.
            6 => {
                // println!("bdv {operand} (combo: {})", self.combo(operand));
                self.b = self.a >> self.combo(operand);
                self.ip += 2;
            }

            // cdv: adv but store to B instead of A.
            7 => {
                // println!("cdv {operand} (combo: {})", self.combo(operand));
                self.c = self.a >> self.combo(operand);
                self.ip += 2;
            }

            _ => panic!("Invalid opcode"),
        }

        if self.ip >= self.program.len() {
            Output::Halt
        } else {
            Output::None
        }
    }

    fn run(&mut self) -> Vec<Output> {
        let mut output = Vec::new();

        loop {
            let step_result = self.step();
            output.push(step_result);
            if matches!(step_result, Output::Halt) {
                break;
            }
        }

        output
    }
}

fn parse_input(lines: &mut LinesIterator) -> Computer {
    let a = lines.next().unwrap().unwrap()[12..]
        .parse::<usize>()
        .unwrap();
    let b = lines.next().unwrap().unwrap()[12..]
        .parse::<usize>()
        .unwrap();
    let c = lines.next().unwrap().unwrap()[12..]
        .parse::<usize>()
        .unwrap();

    lines.next();

    let program = lines.next().unwrap().unwrap()[9..]
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let ip = 0;

    Computer {
        a,
        b,
        c,
        program,
        ip,
    }
}

fn my_prgrm(a: usize) -> Vec<usize> {
    let mut output = Vec::new();
    let mut a = a;
    while a != 0 {
        output.push(0b110 ^ (a ^ (a >> ((a & 0b111) ^ 0b11))) & 0b111);
        a >>= 3;
    }
    output
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let mut comp = parse_input(lines);

    let result =
        comp.run()
            .iter()
            .filter_map(|x| x.to_option())
            .fold(String::new(), |mut output, x| {
                let _ = write!(output, "{x},");
                output
            });

    result[..result.len() - 1].to_string()
}

pub fn run2(lines: &mut LinesIterator) -> String {
    let program = parse_input(lines).program;

    let get_children = |x: (usize, usize)| {
        let mut output: Vec<(usize, usize)> = Vec::new();

        for i in (x.0 << 3)..((x.0 << 3) + 0o10) {
            if i == 0 {
                continue;
            }
            if my_prgrm(i)[0] == program[x.1 - 1] {
                output.push((i, x.1 - 1));
            }
        }

        output
    };

    let is_tgt = |x: (usize, usize)| program == my_prgrm(x.0);

    shortest_path_multiple_tgts((0, program.len()), is_tgt, get_children).map_or_else(
        || "No input will result in program as output".to_string(),
        |path| format!("{}", path[path.len() - 1].0),
    )
}
