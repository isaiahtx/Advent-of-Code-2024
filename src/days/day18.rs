use crate::direction::{Coords, Direction};
use crate::graph::{exists_path, shortest_path, shortest_path_length};
use crate::utils::LinesIterator;

const HEIGHT: usize = 71;
const WIDTH: usize = 71;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Free,
    Corrupted,
}

/// # Panics
fn parse_input(lines: &mut LinesIterator) -> Vec<Coords> {
    let mut output = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split(',');
        let c = parts.next().unwrap().parse::<usize>().unwrap();
        let r = parts.next().unwrap().parse::<usize>().unwrap();

        output.push((r, c));
    }

    output
}

/// # Panics
pub fn run1(lines: &mut LinesIterator) -> String {
    let bytes = parse_input(lines);
    let mut grid = [[Tile::Free; WIDTH]; HEIGHT];

    for (r, c) in bytes.into_iter().take(1024) {
        grid[r][c] = Tile::Corrupted;
    }

    let get_children = |x: Coords| {
        let mut output = Vec::new();
        for dir in [Direction::N, Direction::E, Direction::S, Direction::W] {
            if let Some((r, c)) = dir.step_coords(x, HEIGHT, WIDTH) {
                if !matches!(grid[r][c], Tile::Corrupted) {
                    output.push((r, c));
                }
            }
        }

        output
    };

    let is_tgt = |x: Coords| x == (HEIGHT - 1, WIDTH - 1);

    format!(
        "{}",
        shortest_path_length((0, 0), is_tgt, get_children).unwrap()
    )
}

/// There is definitely a faster way to do this...
///
/// # Panics
pub fn run2(lines: &mut LinesIterator) -> String {
    let mut bytes = parse_input(lines).into_iter();
    let mut grid = [[Tile::Free; WIDTH]; HEIGHT];

    for _ in 0..1024 {
        let (r, c) = bytes.next().unwrap();
        grid[r][c] = Tile::Corrupted;
    }

    let is_tgt = |x: Coords| x == (HEIGHT - 1, WIDTH - 1);

    for (r, c) in bytes {
        grid[r][c] = Tile::Corrupted;

        let get_children = |x: Coords| {
            let mut output = Vec::new();
            for dir in [Direction::N, Direction::E, Direction::S, Direction::W] {
                if let Some((r, c)) = dir.step_coords(x, HEIGHT, WIDTH) {
                    if !matches!(grid[r][c], Tile::Corrupted) {
                        output.push((r, c));
                    }
                }
            }

            output
        };

        if !exists_path((0, 0), is_tgt, get_children) {
            return format!("{c},{r}");
        }
    }

    "never blocked".to_string()
}
