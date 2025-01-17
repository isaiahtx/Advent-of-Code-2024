use crate::{
    direction::{Coords, Direction},
    graph::{get_dist, shortest_path},
    utils::LinesIterator,
};
use std::collections::HashMap;

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

    assert_eq!(path.len() - 1, dist[&tgt]);

    let mut saves: HashMap<usize, usize> = HashMap::new();

    for step in path {
        for wall_nbr in get_wall_nbrs(step) {
            for empty_nbr in get_empty_nbrs(wall_nbr) {
                let cheat_dist = dist[&step] + 2 + dist_rev[&empty_nbr];
                if cheat_dist < dist[&tgt] {
                    *saves.entry(dist[&tgt] - cheat_dist).or_default() += 1;
                }
            }
        }
    }

    // let mut saves_list: Vec<(usize, usize)> = saves.clone().into_iter().collect();
    // saves_list.sort_unstable();
    // for (k, n) in saves_list {
    //     println!("There are {n} cheats that save {k} picoseconds.");
    // }

    let threshold = 100;
    let num: usize = saves
        .into_iter()
        .filter_map(|(k, n)| if k >= threshold { Some(n) } else { None })
        .sum();

    format!("{num}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}
