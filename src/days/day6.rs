use crate::utils::{lines_to_grid_of_chars, LinesIterator};

// tuple of either (-1,0), (0,1), (1,0), or (0,-1).
struct Direction(i8, i8);

struct Guard {
    dir: Direction,
    off_map: bool,
    r: usize,
    c: usize,
}

impl Direction {
    const fn turn_right(&self) -> Self {
        match self {
            Self(-1, 0) => Self(0, 1),
            Self(0, 1) => Self(1, 0),
            Self(1, 0) => Self(0, -1),
            Self(0, -1) => Self(-1, 0),
            _ => panic!("Should never happen!"),
        }
    }

    /*
    const fn get_dir(&self) -> char {
        match self {
            Self(-1, 0) => '^',
            Self(0, 1) => '>',
            Self(1, 0) => '∨',
            Self(0, -1) => '<',
            _ => panic!("Should never happen!"),
        }
    }
    */
}

impl Guard {
    const fn turn_right(&mut self) {
        self.dir = self.dir.turn_right();
    }

    // Causes guard to take a step, outputs true if a new square has been visited, false otherwise.
    fn step(&mut self, grid: &mut Vec<Vec<char>>, height: usize, width: usize) -> bool {
        let max_row = height - 1;
        let new_r: usize = match (self.dir.0, self.r) {
            (-1, 0) => {
                self.off_map = true;
                // grid[self.r][self.c] = 'X';
                return false;
            }
            (1, r) if r == max_row => {
                self.off_map = true;
                // grid[self.r][self.c] = 'X';
                return false;
            }
            (-1, r) if 0 < r && r <= max_row => r - 1,
            (1, r) if r < max_row => r + 1,
            (0, r) if r <= max_row => r,
            _ => panic!("invalid row and/or direction"),
        };

        let max_col = width - 1;
        let new_c: usize = match (self.dir.1, self.c) {
            (-1, 0) => {
                self.off_map = true;
                grid[self.r][self.c] = 'X';
                return false;
            }
            (1, c) if c == max_col => {
                self.off_map = true;
                grid[self.r][self.c] = 'X';
                return false;
            }
            (-1, c) if 0 < c && c <= max_col => c - 1,
            (1, c) if c < max_col => c + 1,
            (0, c) if c <= max_col => c,
            _ => panic!("invalid col and/or direction"),
        };

        let target = grid[new_r][new_c];

        if target == '#' {
            self.turn_right();
            // grid[self.r][self.c] = self.dir.get_dir();
            self.step(grid, height, width)
        } else {
            grid[self.r][self.c] = 'X';
            self.r = new_r;
            self.c = new_c;
            // grid[self.r][self.c] = self.dir.get_dir();
            target == '.'
        }
    }
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let mut grid: Vec<Vec<char>> = lines_to_grid_of_chars(lines).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut starting_r: usize = usize::MAX;
    let mut starting_c: usize = usize::MAX;

    'a: for (i, line) in grid.iter().enumerate() {
        for (j, character) in line.iter().enumerate() {
            if *character == '^' {
                starting_r = i;
                starting_c = j;
                break 'a;
            }
        }
    }

    let mut guard = Guard {
        dir: Direction(-1, 0),
        off_map: false,
        r: starting_r,
        c: starting_c,
    };

    let mut count = 1;

    while !guard.off_map {
        if guard.step(&mut grid, height, width) {
            count += 1;
        }
    }

    for line in &grid {
        for character in line {
            print!("{character}");
        }
        println!();
    }

    format!("{count}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}
