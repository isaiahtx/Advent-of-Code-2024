use crate::graph::shortest_path_cost;
use crate::utils::LinesIterator;

#[derive(Debug)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

const A_COST: usize = 3;
const B_COST: usize = 1;

impl Machine {
    const fn new(a: (usize, usize), b: (usize, usize), prize: (usize, usize)) -> Self {
        Self { a, b, prize }
    }

    fn cost(&self) -> Option<usize> {
        let get_edges = |coords: (usize, usize)| {
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

        shortest_path_cost((0, 0), self.prize, get_edges)
    }
}

fn parse_input(lines: &mut LinesIterator) -> Vec<Machine> {
    let mut output = Vec::new();

    loop {
        let a_line = lines.next().unwrap().unwrap();
        let a1 = a_line[12..14].parse::<usize>().unwrap();
        let a2 = a_line[18..20].parse::<usize>().unwrap();

        let b_line = lines.next().unwrap().unwrap();
        let b1 = b_line[12..14].parse::<usize>().unwrap();
        let b2 = b_line[18..20].parse::<usize>().unwrap();

        let p_line = &lines.next().unwrap().unwrap()[9..];
        let mut p_line = p_line.split(", ");
        let p1 = p_line.next().unwrap().parse::<usize>().unwrap();
        let p2 = p_line.next().unwrap()[2..].parse::<usize>().unwrap();

        output.push(Machine::new((a1, a2), (b1, b2), (p1, p2)));

        if lines.next().is_none() {
            break;
        }
    }

    output
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let output: usize = parse_input(lines).iter().filter_map(Machine::cost).sum();

    format!("{output}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}
