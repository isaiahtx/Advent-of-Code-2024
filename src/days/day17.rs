use crate::utils::LinesIterator;
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Output {
    None,
    Out(i32),
    Halt,
}

impl Output {
    const fn to_option(self) -> Option<i32> {
        if let Self::Out(x) = self {
            Some(x)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Computer {
    a: i32,
    b: i32,
    c: i32,
    program: Vec<i32>,
    ip: usize,
}

impl Computer {
    const fn combo(&self, operand: i32) -> i32 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand"),
        }
    }

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
                println!("adv {operand} (combo: {})", self.combo(operand));
                self.a = (f64::from(self.a) / 2.0_f64.powi(self.combo(operand))) as i32;
                self.ip += 2;
            }

            // bxl: Bitwise XOR B with literal, store to B.
            1 => {
                println!("bxl {operand}");
                self.b ^= operand;
                self.ip += 2;
            }

            // bst: B = combo modulo 8.
            2 => {
                println!("bst {operand} (combo: {})", self.combo(operand));
                self.b = self.combo(operand).rem_euclid(8);
                self.ip += 2;
            }

            // jnz: Do nothing if A == 0, otherwise jump to operand literal.
            // Only increment instruction pointer if doesn't jump.
            3 => {
                println!("jnz {operand}");
                if self.a != 0 {
                    self.ip = operand as usize;
                } else {
                    self.ip += 2;
                }
            }

            // bxc: B = B XOR C.
            4 => {
                println!("bxc");
                self.b ^= self.c;
                self.ip += 2;
            }

            // out: Output combo operand.
            5 => {
                println!("out {operand} (combo: {})", self.combo(operand));
                self.ip += 2;
                return Output::Out(self.combo(operand).rem_euclid(8));
            }

            // bdv: adv but store to B instead of A.
            6 => {
                println!("bdv {operand} (combo: {})", self.combo(operand));
                self.b = (f64::from(self.a) / 2.0_f64.powi(self.combo(operand))) as i32;
                self.ip += 2;
            }

            // cdv: adv but store to B instead of A.
            7 => {
                println!("cdv {operand} (combo: {})", self.combo(operand));
                self.c = (f64::from(self.a) / 2.0_f64.powi(self.combo(operand))) as i32;
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
            println!("\t{step_result:?}");
            println!("{self:?}");
            println!();
            output.push(step_result);
            if matches!(step_result, Output::Halt) {
                break;
            }
        }

        output
    }
}

fn parse_input(lines: &mut LinesIterator) -> Computer {
    let a = lines.next().unwrap().unwrap()[12..].parse::<i32>().unwrap();
    let b = lines.next().unwrap().unwrap()[12..].parse::<i32>().unwrap();
    let c = lines.next().unwrap().unwrap()[12..].parse::<i32>().unwrap();

    lines.next();

    let program = lines.next().unwrap().unwrap()[9..]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
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

pub fn run1(lines: &mut LinesIterator) -> String {
    let mut comp = parse_input(lines);

    println!("{comp:?}");

    let result =
        comp.run()
            .iter()
            .filter_map(|x| x.to_option())
            .fold(String::new(), |mut output, x| {
                let _ = write!(output, "{x},");
                output
            });
    let result = result[..result.len() - 1].to_string();

    println!("{comp:?}");

    result
}

pub fn run2(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}
