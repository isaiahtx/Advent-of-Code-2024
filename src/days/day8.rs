use crate::utils::{lines_to_grid_of_chars, LinesIterator};
use num::CheckedSub;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct P<T>(T, T);

impl<T> Add for P<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
impl<T> CheckedSub for P<T>
where
    T: CheckedSub,
{
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        let first = self.0.checked_sub(&v.0)?;
        let second = self.1.checked_sub(&v.1)?;
        Some(Self(first, second))
    }
}

impl<T> Sub for P<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let grid: Vec<_> = lines_to_grid_of_chars(lines).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut map: HashMap<char, Vec<P<usize>>> = HashMap::new();

    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != '.' {
                map.entry(*c).or_default().push(P(i, j));
            }
        }
    }

    let mut antinodes: HashSet<P<usize>> = HashSet::new();

    for antennae in map.values() {
        let antennae_pairs = antennae
            .iter()
            .enumerate()
            .flat_map(|(i, item1)| antennae.iter().skip(i + 1).map(|item2| (*item1, *item2)));

        for (pos1, pos2) in antennae_pairs {
            for x in get_antinodes(&pos1, &pos2, height, width) {
                antinodes.insert(x);
            }
        }
    }

    format!("{}", antinodes.len())
}

const fn in_bounds(pos: &P<usize>, height: usize, width: usize) -> bool {
    pos.0 < height && pos.1 < width
}

fn get_antinodes(pos1: &P<usize>, pos2: &P<usize>, height: usize, width: usize) -> Vec<P<usize>> {
    let mut output = vec![];

    if let Some(pos) = (*pos2 + *pos2).checked_sub(pos1) {
        if in_bounds(&pos, height, width) {
            output.push(pos);
        }
    }

    if let Some(pos) = (*pos1 + *pos1).checked_sub(pos2) {
        if in_bounds(&pos, height, width) {
            output.push(pos);
        }
    }

    output
}

pub fn run2(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}
