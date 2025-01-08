use crate::uptree::UpTree;
use crate::utils::{lines_to_grid_of_chars, LinesIterator};
use std::collections::{HashSet, VecDeque};

fn make_get_num_edges(grid: &[Vec<char>]) -> impl Fn((usize, usize)) -> usize + '_ {
    let height = grid.len();
    let width = grid[0].len();
    move |t: (usize, usize)| {
        let r = t.0;
        let c = t.1;
        let cur = grid[r][c];
        let mut output = 4;

        if r + 1 < height && grid[r + 1][c] == cur {
            output -= 1;
        }

        if c + 1 < width && grid[r][c + 1] == cur {
            output -= 1;
        }

        if r > 0 && grid[r - 1][c] == cur {
            output -= 1;
        }

        if c > 0 && grid[r][c - 1] == cur {
            output -= 1;
        }

        output
    }
}

fn make_get_nbrs(grid: &[Vec<char>]) -> impl Fn((usize, usize)) -> Vec<(usize, usize)> + '_ {
    let height = grid.len();
    let width = grid[0].len();
    move |t: (usize, usize)| {
        let r = t.0;
        let c = t.1;
        let cur = grid[r][c];
        let mut nbrs: Vec<(usize, usize)> = Vec::new();

        if r + 1 < height && grid[r + 1][c] == cur {
            nbrs.push((r + 1, c));
        }

        if c + 1 < width && grid[r][c + 1] == cur {
            nbrs.push((r, c + 1));
        }

        if r > 0 && grid[r - 1][c] == cur {
            nbrs.push((r - 1, c));
        }

        if c > 0 && grid[r][c - 1] == cur {
            nbrs.push((r, c - 1));
        }

        nbrs
    }
}

/// # Panics
///
/// stfu
pub fn run1(lines: &mut LinesIterator) -> String {
    let grid: Vec<_> = lines_to_grid_of_chars(lines).collect();
    let get_nbrs = make_get_nbrs(&grid);
    let get_num_edges = make_get_num_edges(&grid);
    let mut ut: UpTree<(usize, usize)> = UpTree::new();

    let height = grid.len();
    let width = grid[0].len();

    let mut q = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..height {
        for j in 0..width {
            q.push_back((i, j));
            ut.insert_root((i, j));
        }
    }

    while !q.is_empty() {
        let v = q.pop_back().unwrap();
        for w in get_nbrs(v) {
            if !seen.contains(&w) {
                ut.union(&w, &v);
            }
        }
        seen.insert(v);
    }

    let regions = ut.flatten();

    let mut output = 0;

    for region in regions {
        let area = region.len();
        let mut perimeter = 0;
        for (v, ()) in region {
            perimeter += get_num_edges(*v);
        }
        output += area * perimeter;
    }

    format!("{output}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}
