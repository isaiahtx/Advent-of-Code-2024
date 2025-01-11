use crate::direction::{Coords, Direction};
use crate::graph::shortest_path_cost_multiple_tgts;
use crate::utils::LinesIterator;

const FWD_COST: usize = 1;
const TURN_COST: usize = 1000;

type Pos = (Coords, Direction);

enum Tile {
    Start,
    End,
    Empty,
    Wall,
}

impl Tile {
    const fn from(c: char) -> Self {
        match c {
            'S' => Self::Start,
            'E' => Self::End,
            '#' => Self::Wall,
            '.' => Self::Empty,
            _ => panic!(),
        }
    }
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let grid: Vec<Vec<Tile>> = lines
        .map(Result::unwrap)
        .map(|v| v.chars().map(Tile::from).collect())
        .collect();

    let height = grid.len();
    let width = grid.len();

    let src: Pos = ((height - 2, 1), Direction::E);

    let is_tgt = |p: Pos| p.0 == (1, width - 2);
    let get_edges = |p: Pos| {
        let mut output: Vec<(usize, Pos)> = Vec::new();

        for dir in [Direction::N, Direction::E, Direction::S, Direction::W] {
            let (tgt_r, tgt_c) = dir.step_coords_cardinal_unchecked(p.0);

            if !matches!(grid[tgt_r][tgt_c], Tile::Wall) {
                let cost: usize;
                if dir == p.1 {
                    cost = FWD_COST;
                } else if dir == p.1.reflect() {
                    cost = 2 * TURN_COST + FWD_COST;
                } else {
                    cost = TURN_COST + FWD_COST;
                }
                output.push((cost, ((tgt_r, tgt_c), dir)));
            }
        }

        output
    };

    let output = shortest_path_cost_multiple_tgts(src, is_tgt, get_edges).unwrap();

    format!("{output}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}
