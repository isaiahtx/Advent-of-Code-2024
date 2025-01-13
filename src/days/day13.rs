use crate::graph::shortest_path_cost;
use crate::utils::LinesIterator;

#[derive(Debug)]
struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

const A_COST: usize = 3;
const B_COST: usize = 1;

impl Machine {
    const fn new(a: (u64, u64), b: (u64, u64), prize: (u64, u64)) -> Self {
        Self { a, b, prize }
    }

    fn cost(&self) -> Option<u64> {
        let mut get_edges = |coords: (u64, u64)| {
            let mut output = Vec::new();

            let ax = coords.0 + self.a.0;
            let ay = coords.1 + self.a.1;
            if ax <= self.prize.0 && ay <= self.prize.1 {
                output.push((A_COST, (ax, ay)));
            }

            let bx = coords.0 + self.b.0;
            let by = coords.1 + self.b.1;
            if bx <= self.prize.0 && by <= self.prize.1 {
                output.push((B_COST, (bx, by)));
            }

            output
        };

        shortest_path_cost((0, 0), |x| x == self.prize, &mut get_edges).map(|x| x as u64)
    }

    /// Write A = [a.0, b.0 ; a.1, b.1] and p = (prize.0, prize.1). Then a
    /// valid move to the prize is the data of any 2d vector n = (n.0, n.1)
    /// such that An = s.
    #[allow(clippy::redundant_else)]
    const fn cost2(&self) -> Option<u64> {
        let a1 = self.a.0;
        let a2 = self.a.1;

        let b1 = self.b.0;
        let b2 = self.b.1;

        let p1 = self.prize.0;
        let p2 = self.prize.1;

        let mut output = None;

        if a1 * b2 == a2 * b1 {
            // Originally I wrote the following (incorrect) algorithm, but then
            // I realized the input Advent of Code gave me does not contain the
            // singular case, so this can just be ignored.

            // let n = p1 / a1;
            // let m = p1 / b1;

            // // Greedy algorithm, not sure if it works

            // // To start, we get as close as we can to the target (p1) from
            // // below, as cheaply as possible.
            // if (n * A_COST as u64) < (m * B_COST as u64) {
            //     // In this case, it is cheaper to get right below using a
            //     // instead of b.

            //     // The remainder that we need to get using b, if it's possible.
            //     // So we need to figure out what m should be.

            //     // remainder = p1 - n * a1
            //     let remainder = p1.checked_sub(n * a1).unwrap();

            //     if (remainder % b1) == 0 {
            //         // if we can get the remainder using b...

            //         // then set m!
            //         let m = remainder / b1;

            //         // We found n and m for the first coordinate, make sure they
            //         // also work for the second coordinate.
            //         if n * a2 + m * b2 == p2 {
            //             output = Some(n * A_COST as u64 + m * B_COST as u64);
            //         }
            //     }
            // } else {
            //     // remainder = p1 - m * b1
            //     let remainder = p1.checked_sub(m * b1).unwrap();

            //     if (remainder % a1) == 0 {
            //         let m = remainder / a1;
            //         if n * a2 + m * b2 == p2 {
            //             output = Some(n * A_COST as u64 + m * B_COST as u64);
            //         }
            //     }
            // }
        } else {
            let det = (a1 * b2) as i128 - (a2 * b1) as i128;
            let m1 = (b2 * p1) as i128 - (b1 * p2) as i128;
            let m2 = (a1 * p2) as i128 - (a2 * p1) as i128;
            if m1 % det == 0 && m2 % det == 0 {
                let n1 = m1 / det;
                let n2 = m2 / det;
                if n1 >= 0 && n2 >= 0 {
                    output = Some((n1 as u64 * A_COST as u64) + (n2 as u64 * B_COST as u64));
                }
            }
        }
        output
    }
}

fn parse_input(lines: &mut LinesIterator) -> Vec<Machine> {
    let mut output = Vec::new();

    loop {
        let a_line = lines.next().unwrap().unwrap();
        let a1 = a_line[12..14].parse::<u64>().unwrap();
        let a2 = a_line[18..20].parse::<u64>().unwrap();

        let b_line = lines.next().unwrap().unwrap();
        let b1 = b_line[12..14].parse::<u64>().unwrap();
        let b2 = b_line[18..20].parse::<u64>().unwrap();

        let p_line = &lines.next().unwrap().unwrap()[9..];
        let mut p_line = p_line.split(", ");
        let p1 = p_line.next().unwrap().parse::<u64>().unwrap();
        let p2 = p_line.next().unwrap()[2..].parse::<u64>().unwrap();

        output.push(Machine::new((a1, a2), (b1, b2), (p1, p2)));

        if lines.next().is_none() {
            break;
        }
    }

    output
}

fn parse_input_2(lines: &mut LinesIterator) -> Vec<Machine> {
    let mut output = Vec::new();

    loop {
        let a_line = lines.next().unwrap().unwrap();
        let a1 = a_line[12..14].parse::<u64>().unwrap();
        let a2 = a_line[18..20].parse::<u64>().unwrap();

        let b_line = lines.next().unwrap().unwrap();
        let b1 = b_line[12..14].parse::<u64>().unwrap();
        let b2 = b_line[18..20].parse::<u64>().unwrap();

        let p_line = &lines.next().unwrap().unwrap()[9..];
        let mut p_line = p_line.split(", ");
        let p1 = p_line.next().unwrap().parse::<u64>().unwrap();
        let p2 = p_line.next().unwrap()[2..].parse::<u64>().unwrap();

        output.push(Machine::new(
            (a1, a2),
            (b1, b2),
            (p1 + 10_000_000_000_000, p2 + 10_000_000_000_000),
        ));

        if lines.next().is_none() {
            break;
        }
    }

    output
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let output: u64 = parse_input(lines).iter().filter_map(Machine::cost).sum();

    format!("{output}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    let output: u64 = parse_input_2(lines).iter().filter_map(Machine::cost2).sum();

    format!("{output}")
}
