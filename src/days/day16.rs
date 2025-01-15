use crate::direction::{Coords, Direction};
use crate::graph::{get_nodes_in_cheapest_paths, shortest_path_cost};
use crate::memoizer::Memoizer;
use crate::utils::LinesIterator;
use std::collections::HashSet;
use std::hash::Hash;

const FWD_COST: usize = 1;
const TURN_COST: usize = 1000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos(Coords, Direction);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Start,
    End,
    Empty,
    Wall,
}

fn make_get_children(grid: &[Vec<Tile>]) -> impl Fn(Pos) -> Vec<(usize, Pos)> + use<'_> {
    |p: Pos| {
        let mut output: Vec<(usize, Pos)> = Vec::new();

        for dir in [Direction::N, Direction::E, Direction::S, Direction::W] {
            let (tgt_r, tgt_c) = dir.step_coords_unchecked(p.0);

            if !matches!(grid[tgt_r][tgt_c], Tile::Wall) {
                let cost: usize;
                if dir == p.1 {
                    cost = FWD_COST;
                } else if dir == p.1.reflect() {
                    cost = 2 * TURN_COST + FWD_COST;
                } else {
                    cost = TURN_COST + FWD_COST;
                }
                output.push((cost, Pos((tgt_r, tgt_c), dir)));
            }
        }

        output
    }
}

fn make_get_parents(grid: &[Vec<Tile>]) -> impl Fn(Pos) -> Vec<(usize, Pos)> + use<'_> {
    |p: Pos| {
        // We are given a starting position, and we want to return a list of all the positions that can reach it. Note the only coordinates that can reach it is the one in the opposite of the direction that the starting position is currently facing.

        let mut output: Vec<(usize, Pos)> = Vec::new();

        let (side_r, side_c) = p.1.reflect().step_coords_unchecked(p.0);

        if !matches!(grid[side_r][side_c], Tile::Wall) {
            output.push((FWD_COST, Pos((side_r, side_c), p.1)));
            output.push((
                TURN_COST + FWD_COST,
                Pos((side_r, side_c), p.1.turn_right()),
            ));
            output.push((
                2 * TURN_COST + FWD_COST,
                Pos((side_r, side_c), p.1.reflect()),
            ));
            output.push((TURN_COST + FWD_COST, Pos((side_r, side_c), p.1.turn_left())));
        }

        output
    }
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

    // const fn to_char(self) -> char {
    //     match self {
    //         Self::Start => 'S',
    //         Self::End => 'E',
    //         Self::Wall => '#',
    //         Self::Empty => '.',
    //     }
    // }
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let grid: Vec<Vec<Tile>> = lines
        .map(Result::unwrap)
        .map(|v| v.chars().map(Tile::from).collect())
        .collect();

    let height = grid.len();
    let width = grid.len();

    let src = Pos((height - 2, 1), Direction::E);
    let is_tgt = |p: Pos| p.0 == (1, width - 2);

    let mut get_children_memoizer = Memoizer::new(make_get_children(&grid));
    let mut get_children = |x| get_children_memoizer.call(x);

    let output = shortest_path_cost(src, is_tgt, &mut get_children);

    output.map_or_else(|| "No path found".to_string(), |cost| format!("{cost}"))
}

/// # Panics
pub fn run2(lines: &mut LinesIterator) -> String {
    let grid: Vec<Vec<Tile>> = lines
        .map(Result::unwrap)
        .map(|v| v.chars().map(Tile::from).collect())
        .collect();

    let height = grid.len();
    let width = grid.len();

    let src = Pos((height - 2, 1), Direction::E);

    let mut get_children_memoizer = Memoizer::new(make_get_children(&grid));
    let mut get_children = |x| get_children_memoizer.call(x);

    let mut get_parents_memoizer = Memoizer::new(make_get_parents(&grid));
    let mut get_parents = |x| get_parents_memoizer.call(x);

    let mut is_tgt = |x: Pos| x.0 == (1, width - 2);

    let visited: HashSet<Coords> =
        get_nodes_in_cheapest_paths(src, &mut is_tgt, &mut get_children, &mut get_parents)
            .unwrap()
            .iter()
            .map(|Pos(a, _)| *a)
            .collect();

    let output = visited.len();

    format!("{output}")
}
