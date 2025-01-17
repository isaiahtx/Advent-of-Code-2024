use crate::{
    direction::{Coords, Direction},
    graph::{get_dist, shortest_path},
    utils::LinesIterator,
};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
}

impl Tile {
    const fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            _ => Self::Empty,
        }
    }
}

fn parse_input(lines: &mut LinesIterator) -> (Vec<Vec<Tile>>, Coords, Coords) {
    let mut grid = Vec::new();
    let mut start: Coords = (0, 0);
    let mut end: Coords = (0, 0);

    for (r, row) in lines.map(Result::unwrap).enumerate() {
        grid.push(
            row.chars()
                .enumerate()
                .map(|(c, t)| {
                    if t == 'S' {
                        start = (r, c);
                    } else if t == 'E' {
                        end = (r, c);
                    }
                    Tile::from(t)
                })
                .collect::<Vec<_>>(),
        );
    }

    (grid, start, end)
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let (grid, src, tgt) = parse_input(lines);

    let threshold = 100;
    let height = grid.len();
    let width = grid[0].len();

    let get_empty_nbrs = |x: Coords| {
        let mut output = Vec::new();
        for dir in [Direction::N, Direction::E, Direction::S, Direction::W] {
            if let Some((r, c)) = dir.step_coords(x, height, width) {
                if matches!(grid[r][c], Tile::Empty) {
                    output.push((r, c));
                }
            }
        }
        output
    };

    let get_wall_nbrs = |x: Coords| {
        let mut output = Vec::new();
        for dir in [Direction::N, Direction::E, Direction::S, Direction::W] {
            if let Some((r, c)) = dir.step_coords(x, height, width) {
                if matches!(grid[r][c], Tile::Wall) {
                    output.push((r, c));
                }
            }
        }
        output
    };

    let path = shortest_path(src, tgt, get_empty_nbrs).unwrap();

    let dist = get_dist(src, get_empty_nbrs);

    let dist_rev = get_dist(tgt, get_empty_nbrs);

    let mut output = 0;

    for step in path {
        for wall_nbr in get_wall_nbrs(step) {
            for empty_nbr in get_empty_nbrs(wall_nbr) {
                if dist[&tgt] >= threshold + dist[&step] + 2 + dist_rev[&empty_nbr] {
                    output += 1;
                }
            }
        }
    }

    format!("{output}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    let (grid, src, tgt) = parse_input(lines);

    let threshold: usize = 100;
    let max_steps: usize = 20;
    let height = grid.len();
    let width = grid[0].len();

    let get_empty_nbrs = |x: Coords| {
        let mut output = Vec::new();
        for dir in [Direction::N, Direction::E, Direction::S, Direction::W] {
            if let Some((r, c)) = dir.step_coords(x, height, width) {
                if matches!(grid[r][c], Tile::Empty) {
                    output.push((r, c));
                }
            }
        }
        output
    };

    let get_wall_nbrs = |x: Coords| {
        let mut output = Vec::new();
        for dir in [Direction::N, Direction::E, Direction::S, Direction::W] {
            if let Some((r, c)) = dir.step_coords(x, height, width) {
                if matches!(grid[r][c], Tile::Wall) {
                    output.push((r, c));
                }
            }
        }
        output
    };

    let path = shortest_path(src, tgt, get_empty_nbrs).unwrap();

    let dist = get_dist(src, get_empty_nbrs);

    let dist_rev = get_dist(tgt, get_empty_nbrs);

    let mut output = 0;

    for cheat_start in path {
        let mut seen = HashSet::new();
        let mut to_check = VecDeque::new();
        to_check.push_back((cheat_start, 0));
        while !to_check.is_empty() {
            let (u, d) = to_check.pop_front().unwrap();

            for wall_nbr in get_wall_nbrs(u) {
                if d < max_steps && seen.insert(wall_nbr) {
                    to_check.push_back((wall_nbr, d + 1));
                }
            }

            for empty_nbr in get_empty_nbrs(u) {
                if d < max_steps && seen.insert(empty_nbr) {
                    to_check.push_back((empty_nbr, d + 1));
                    if threshold + dist[&cheat_start] + d + 1 + dist_rev[&empty_nbr] <= dist[&tgt] {
                        output += 1;
                    }
                }
            }
        }
    }

    format!("{output}")
}
