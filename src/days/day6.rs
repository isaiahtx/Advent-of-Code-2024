use crate::utils::{lines_to_grid_of_chars, LinesIterator};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::io::Write;

// tuple of either (-1,0), (0,1), (1,0), or (0,-1).
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Direction(i8, i8);

impl Direction {
    const fn turn_right(self) -> Self {
        match self {
            Self(-1, 0) => Self(0, 1),
            Self(0, 1) => Self(1, 0),
            Self(1, 0) => Self(0, -1),
            Self(0, -1) => Self(-1, 0),
            _ => panic!("Should never happen!"),
        }
    }
}

#[derive(Debug)]
enum StepOutput {
    Ok,
    TurnRight,
    Loop,
    OffMap,
}

struct Board {
    grid: Vec<Vec<char>>,
    guard_r: usize,
    guard_c: usize,
    guard_dir: Direction,
    guard_off_map: bool,
    guard_visited: HashMap<(usize, usize), HashSet<Direction>>,
    check_for_loops: bool,
}

impl Board {
    fn new(mut grid: Vec<Vec<char>>) -> Self {
        // Find the guard
        let mut guard_r: usize = usize::MAX;
        let mut guard_c: usize = usize::MAX;

        'a: for (i, line) in grid.iter().enumerate() {
            for (j, character) in line.iter().enumerate() {
                if *character == '^' {
                    guard_r = i;
                    guard_c = j;
                    break 'a;
                }
            }
        }

        // Replace the guard's position with a '.' (we won't change the board again)
        grid[guard_r][guard_c] = '.';

        // Keep track of the starting position, which we have visited with orientation (-1,0).
        let mut visited = HashMap::new();
        visited.insert((guard_r, guard_c), HashSet::new());
        visited
            .get_mut(&(guard_r, guard_c))
            .unwrap()
            .insert(Direction(-1, 0));

        // Initialize & return
        Self {
            grid,
            guard_r,
            guard_c,
            guard_dir: Direction(-1, 0),
            guard_off_map: false,
            guard_visited: visited,
            check_for_loops: true,
        }
    }

    fn step(&mut self) -> StepOutput {
        let height = self.grid.len();
        assert!(height > 0);
        let width = self.grid[0].len();
        assert!(width > 0);

        let max_row = height - 1;
        let new_r: usize = match (self.guard_dir.0, self.guard_r) {
            (-1, 0) => {
                self.guard_off_map = true;
                return StepOutput::OffMap;
            }
            (1, r) if r == max_row => {
                self.guard_off_map = true;
                return StepOutput::OffMap;
            }
            (-1, r) if 0 < r && r <= max_row => r - 1,
            (1, r) if r < max_row => r + 1,
            (0, r) if r <= max_row => r,
            _ => panic!("invalid row and/or direction"),
        };

        let max_col = width - 1;
        let new_c: usize = match (self.guard_dir.1, self.guard_c) {
            (-1, 0) => {
                self.guard_off_map = true;
                return StepOutput::OffMap;
            }
            (1, c) if c == max_col => {
                self.guard_off_map = true;
                return StepOutput::OffMap;
            }
            (-1, c) if 0 < c && c <= max_col => c - 1,
            (1, c) if c < max_col => c + 1,
            (0, c) if c <= max_col => c,
            _ => panic!("invalid col and/or direction"),
        };

        let target = self.grid[new_r][new_c];

        if target == '#' {
            self.guard_dir = self.guard_dir.turn_right();

            let directions = self.guard_visited.entry((new_r, new_c));

            if let Entry::Occupied(e) = &directions {
                if self.check_for_loops && e.get().contains(&self.guard_dir) {
                    return StepOutput::Loop;
                }
            }
            directions.or_default().insert(self.guard_dir);

            StepOutput::TurnRight
        } else {
            self.guard_r = new_r;
            self.guard_c = new_c;

            let directions = self.guard_visited.entry((new_r, new_c));

            if let Entry::Occupied(e) = &directions {
                if self.check_for_loops && e.get().contains(&self.guard_dir) {
                    return StepOutput::Loop;
                }
            }
            directions.or_default().insert(self.guard_dir);

            StepOutput::Ok
        }
    }
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let grid: Vec<Vec<char>> = lines_to_grid_of_chars(lines).collect();
    let mut board = Board::new(grid);

    while !board.guard_off_map {
        let result = board.step();
        if matches!(result, StepOutput::Loop) {
            println!("loop!");
            break;
        }
    }

    format!("{}", board.guard_visited.keys().count())
}

pub fn run2(lines: &mut LinesIterator) -> String {
    let grid: Vec<_> = lines_to_grid_of_chars(lines).collect();
    let mut board = Board::new(grid.clone());
    let starting_r = board.guard_r;
    let starting_c = board.guard_c;

    // First run the board to see which positions are hit at all.
    while !board.guard_off_map {
        let result = board.step();
        if matches!(result, StepOutput::Loop) {
            println!("loop!");
            break;
        }
    }

    let visited_positions = board
        .guard_visited
        .keys()
        .filter(|&&x| x != (starting_r, starting_c));

    let mut num_loops = 0;
    let num = visited_positions.clone().count();
    println!("Number of positions to try: {num}");

    // Try every possible position to put a barrier to see if it loops
    for (i, (r, c)) in visited_positions.enumerate() {
        let percentage = 100.0 * (i as f64) / (num as f64);
        print!("\rProgress: {percentage:.2}%");
        std::io::stdout().flush().unwrap();
        let r = *r;
        let c = *c;
        let mut tmp_grid = grid.clone();
        tmp_grid[r][c] = '#';
        let mut loops = false;

        let mut tmp_board = Board::new(tmp_grid);

        while !tmp_board.guard_off_map {
            if matches!(tmp_board.step(), StepOutput::Loop) {
                loops = true;
                break;
            }
        }
        if loops {
            num_loops += 1;
        }
    }
    println!();

    format!("Number of positions which cause a loop: {num_loops}")
}
