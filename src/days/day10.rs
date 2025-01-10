use crate::graph::{num_paths, num_reachable_targets};
use crate::utils::{lines_to_grid_of_usize, LinesIterator};

fn make_get_edges(map: &[Vec<usize>]) -> impl Fn((usize, usize)) -> Vec<(usize, usize)> + '_ {
    let height = map.len();
    let width = map[0].len();
    move |t: (usize, usize)| {
        let r = t.0;
        let c = t.1;
        let cur = map[r][c];
        let mut edges: Vec<(usize, usize)> = Vec::new();

        if r + 1 < height && map[r + 1][c] == cur + 1 {
            edges.push((r + 1, c));
        }

        if c + 1 < width && map[r][c + 1] == cur + 1 {
            edges.push((r, c + 1));
        }

        if r > 0 && map[r - 1][c] == cur + 1 {
            edges.push((r - 1, c));
        }

        if c > 0 && map[r][c - 1] == cur + 1 {
            edges.push((r, c - 1));
        }

        edges
    }
}

fn make_is_9(map: &[Vec<usize>]) -> impl Fn((usize, usize)) -> bool + '_ {
    |t: (usize, usize)| map[t.0][t.1] == 9
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let map: Vec<_> = lines_to_grid_of_usize(lines).collect();

    let get_edges = make_get_edges(&map);
    let is_9 = make_is_9(&map);

    let mut output = 0;

    for (r, row) in map.iter().enumerate() {
        for (c, character) in row.iter().enumerate() {
            if *character == 0 {
                output += num_reachable_targets((r, c), &is_9, &get_edges);
            }
        }
    }

    format!("{output}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    let map: Vec<_> = lines_to_grid_of_usize(lines).collect();

    let get_edges = make_get_edges(&map);
    let is_9 = make_is_9(&map);

    let mut output = 0;

    for (r, row) in map.iter().enumerate() {
        for (c, character) in row.iter().enumerate() {
            if *character == 0 {
                output += num_paths((r, c), &is_9, &get_edges);
            }
        }
    }

    format!("{output}")
}
