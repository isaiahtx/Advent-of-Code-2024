use crate::uptree::UpTree;
use crate::utils::{lines_to_grid_of_chars, LinesIterator};
use std::collections::{HashSet, VecDeque};

/// # Panics
///
/// stfu
pub fn run1(lines: &mut LinesIterator) -> String {
    let grid: Vec<Vec<char>> = lines_to_grid_of_chars(lines).collect();
    let mut ut: UpTree<(usize, usize)> = UpTree::new();
    let mut q = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    let height = grid.len();
    let width = grid[0].len();

    let mut nbrs: Vec<Vec<Vec<(usize, usize)>>> = Vec::with_capacity(height);
    for (i, row) in grid.iter().enumerate() {
        nbrs.push(Vec::with_capacity(width));
        for (j, cur) in row.iter().enumerate() {
            nbrs[i].push(Vec::new());
            q.push_back((i, j));
            ut.insert_root((i, j));

            if i > 0 && grid[i - 1][j] == *cur {
                nbrs[i][j].push((i - 1, j));
            }

            if j + 1 < width && grid[i][j + 1] == *cur {
                nbrs[i][j].push((i, j + 1));
            }

            if i + 1 < height && grid[i + 1][j] == *cur {
                nbrs[i][j].push((i + 1, j));
            }

            if j > 0 && grid[i][j - 1] == *cur {
                nbrs[i][j].push((i, j - 1));
            }
        }
    }

    while !q.is_empty() {
        let v = q.pop_back().unwrap();
        for w in &nbrs[v.0][v.1] {
            if !seen.contains(w) {
                ut.union(w, &v);
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
            perimeter += 4 - nbrs[v.0][v.1].len();
        }
        output += area * perimeter;
    }

    format!("{output}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}
