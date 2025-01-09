use crate::uptree::UpTree;
use crate::utils::{lines_to_grid_of_chars, LinesIterator};
use std::collections::{HashSet, VecDeque};

type Coords = (usize, usize);
type Nbr = (Direction, Coords);
type NbrsGrid = Vec<Vec<Nbrs>>;
type RegionsList = Vec<Vec<Coords>>;

#[derive(Debug, Clone)]
struct Nbrs {
    // [north,northeast,east,southeast,south,southwest,west,northwest]
    nbrs: [Option<Coords>; 8],
    count: usize,
}

impl Nbrs {
    const fn new() -> Self {
        Self {
            nbrs: [None; 8],
            count: 0,
        }
    }

    // fn remove(&mut self, dir: Direction) -> bool {
    //     let entry = &mut self.nbrs[dir.to_num()];
    //     if entry.is_some() {
    //         *entry = None;
    //         self.count -= 1;
    //         true
    //     } else {
    //         false
    //     }
    // }

    fn add(&mut self, dir: Direction, coords: Coords) -> bool {
        let output = self.nbrs[dir.to_num()].is_none();
        self.nbrs[dir.to_num()] = Some(coords);
        if output {
            self.count += 1;
        }
        output
    }

    fn num_cardinals(&self) -> usize {
        usize::from(self.nbrs[0].is_some())
            + usize::from(self.nbrs[2].is_some())
            + usize::from(self.nbrs[4].is_some())
            + usize::from(self.nbrs[6].is_some())

        // Alternatively:
        //
        // self.nbrs
        //     .into_iter()
        //     .step_by(2)
        //     .filter(Option::is_some)
        //     .count()
    }

    fn get_cardinals(&self) -> Vec<Nbr> {
        let mut output = Vec::new();
        for i in [0, 2, 4, 6] {
            if let Some(coords) = self.nbrs[i] {
                output.push((Direction::from(i).unwrap(), coords));
            }
        }
        output

        // Alternatively:
        //
        // self.nbrs
        //     .into_iter()
        //     .enumerate()
        //     .step_by(2)
        //     .filter_map(|(i, c)| c.map(|coords| (Direction::from(i).unwrap(), coords)))
        //     .collect()
    }
}

// impl AsRef<[Option<Coords>; 8]> for Nbrs {
//     fn as_ref(&self) -> &[Option<Coords>; 8] {
//         &self.nbrs
//     }
// }

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn from(n: usize) -> Result<Self, String> {
        match n {
            0 => Ok(Self::N),
            1 => Ok(Self::NE),
            2 => Ok(Self::E),
            3 => Ok(Self::SE),
            4 => Ok(Self::S),
            5 => Ok(Self::SW),
            6 => Ok(Self::W),
            7 => Ok(Self::NW),
            _ => Err("invalid entry".to_string()),
        }
    }

    const fn turn_right(&mut self) {
        *self = match self {
            Self::N => Self::E,
            Self::NE => Self::SE,
            Self::E => Self::S,
            Self::SE => Self::SW,
            Self::S => Self::W,
            Self::SW => Self::NW,
            Self::W => Self::N,
            Self::NW => Self::NE,
        }
    }

    const fn to_num(self) -> usize {
        match self {
            Self::N => 0,
            Self::NE => 1,
            Self::E => 2,
            Self::SE => 3,
            Self::S => 4,
            Self::SW => 5,
            Self::W => 6,
            Self::NW => 7,
        }
    }
}

fn get_all_nbrs_grid(grid: &[Vec<char>]) -> NbrsGrid {
    let height = grid.len();
    let width = grid[0].len();

    let mut all_nbrs_grid = Vec::with_capacity(height);

    for i in 0..height {
        all_nbrs_grid.push(Vec::with_capacity(width));
        for j in 0..width {
            all_nbrs_grid[i].push(Nbrs::new());

            let mut north = false;
            let mut south = false;

            if i > 0 {
                all_nbrs_grid[i][j].add(Direction::N, (i - 1, j));
                north = true;
            }

            if i + 1 < height {
                all_nbrs_grid[i][j].add(Direction::S, (i + 1, j));
                south = true;
            }

            if j + 1 < width {
                all_nbrs_grid[i][j].add(Direction::E, (i, j + 1));
                if north {
                    all_nbrs_grid[i][j].add(Direction::NE, (i - 1, j + 1));
                }
                if south {
                    all_nbrs_grid[i][j].add(Direction::SE, (i + 1, j + 1));
                }
            }

            if j > 0 {
                all_nbrs_grid[i][j].add(Direction::W, (i, j - 1));
                if north {
                    all_nbrs_grid[i][j].add(Direction::NW, (i - 1, j - 1));
                }
                if south {
                    all_nbrs_grid[i][j].add(Direction::SW, (i + 1, j - 1));
                }
            }
        }
    }

    all_nbrs_grid
}

fn get_components(grid: &[Vec<char>]) -> (NbrsGrid, RegionsList) {
    let height = grid.len();
    let width = grid[0].len();

    let mut ut: UpTree<Coords> = UpTree::new();
    let mut q = VecDeque::new();
    let mut seen: HashSet<Coords> = HashSet::new();
    let mut filtered_nbrs_grid: NbrsGrid = Vec::with_capacity(height);
    for (i, row) in grid.iter().enumerate() {
        filtered_nbrs_grid.push(Vec::with_capacity(width));
        for (j, cur) in row.iter().enumerate() {
            filtered_nbrs_grid[i].push(Nbrs::new());
            q.push_back((i, j));
            ut.insert_root((i, j));

            let cur = *cur;

            if i > 0 && grid[i - 1][j] == cur {
                filtered_nbrs_grid[i][j].add(Direction::N, (i - 1, j));
            }

            if i + 1 < height && grid[i + 1][j] == cur {
                filtered_nbrs_grid[i][j].add(Direction::S, (i + 1, j));
            }

            if j + 1 < width && grid[i][j + 1] == cur {
                filtered_nbrs_grid[i][j].add(Direction::E, (i, j + 1));
            }

            if j > 0 && grid[i][j - 1] == cur {
                filtered_nbrs_grid[i][j].add(Direction::W, (i, j - 1));
            }
        }
    }

    while !q.is_empty() {
        let v = q.pop_back().unwrap();
        for (_, w) in filtered_nbrs_grid[v.0][v.1].get_cardinals() {
            if !seen.contains(&w) {
                ut.union(&w, &v);
            }
        }
        seen.insert(v);
    }

    (
        filtered_nbrs_grid,
        ut.flatten()
            .iter()
            .map(|component| component.iter().map(|(&coords, ())| coords).collect())
            .collect(),
    )
}

/// # Panics
///
/// stfu
pub fn run1(lines: &mut LinesIterator) -> String {
    let grid: Vec<_> = lines_to_grid_of_chars(lines).collect();
    let (filtered_nbrs_grid, regions) = get_components(&grid);

    let mut output = 0;

    for region in regions {
        let area = region.len();
        let mut perimeter = 0;
        for v in region {
            perimeter += 4 - filtered_nbrs_grid[v.0][v.1].num_cardinals();
        }
        output += area * perimeter;
    }

    format!("{output}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    // let grid: Vec<_> = lines_to_grid_of_chars(lines).collect();
    // let (nbrs, regions) = get_components(&grid);

    // for region in regions {
    //     let mut start: Coords = (0, 0);
    //     let mut direction: Direction = Direction::N;
    //     for v in region {
    //         if nbrs[v.0][v.1].num_cardinals() < 4 {
    //             start = v;
    //             break;
    //         }
    //     }
    // }

    let mut output = 0;

    format!("{output}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut nbrs = Nbrs::new();
        nbrs.add(Direction::N, (0, 0));
        nbrs.add(Direction::NE, (0, 0));
        nbrs.add(Direction::E, (0, 0));
        nbrs.add(Direction::SE, (0, 0));
        nbrs.add(Direction::S, (0, 0));
        nbrs.add(Direction::SW, (0, 0));
        nbrs.add(Direction::W, (0, 0));
        nbrs.add(Direction::NW, (0, 0));

        assert_eq!(nbrs.count, 8);
    }
}
