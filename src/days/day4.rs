use crate::utils::{LinesIterator, lines_to_grid_of_chars};

pub fn run1(lines: &mut LinesIterator) -> String {
    let grid: Vec<Vec<char>> = lines_to_grid_of_chars(lines).collect();
    let mut output: u32 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let c = grid[i][j];
            if c == 'X' {
                output += u32::from(check_xmas(&grid, i, j));
            }
        }
    }

    format!("{output}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    let grid: Vec<Vec<char>> = lines_to_grid_of_chars(lines).collect();
    let mut output: u32 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let c = grid[i][j];
            if c == 'A' && check_x(&grid, i, j) {
                output += 1;
            }
        }
    }

    format!("{output}")
}

fn check_x(grid: &[Vec<char>], r: usize, c: usize) -> bool {
    assert!(grid[r][c] == 'A');
    let height = grid.len();
    let width = grid[0].len();

    if (r == 0) || (r == height - 1) || (c == 0) || (c == width - 1) {
        return false;
    }

    let mut my_chars: [char; 4] = [' '; 4];

    my_chars[0] = grid[r - 1][c - 1];
    my_chars[1] = grid[r - 1][c + 1];
    my_chars[2] = grid[r + 1][c + 1];
    my_chars[3] = grid[r + 1][c - 1];

    let valid: [[char; 4]; 4] = [
        ['M', 'M', 'S', 'S'],
        ['M', 'S', 'S', 'M'],
        ['S', 'S', 'M', 'M'],
        ['S', 'M', 'M', 'S'],
    ];

    if valid.contains(&my_chars) {
        true
    } else {
        //println!("{my_chars:?}");
        //println!("{:?}", &grid[r - 1][c - 1..=c + 1]);
        //println!("{:?}", &grid[r][c - 1..=c + 1]);
        //println!("{:?}", &grid[r + 1][c - 1..=c + 1]);
        //println!();
        false
    }
}

fn check_xmas(grid: &[Vec<char>], r: usize, c: usize) -> u8 {
    assert!(grid[r][c] == 'X');
    let word: Vec<char> = "XMAS".chars().collect();
    let string_length = word.len();

    let height = grid.len();
    let width = grid[0].len();

    let mut output = 0;

    let directions: [(i8, i8); 8] = [
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 1),
        (-1, -1),
        (1, 1),
        (1, -1),
    ];

    'outer: for (i, j) in directions {
        // check to see if it's even possible for there to be an occurrence in the direction

        if (i == -1 && r + 1 < string_length)
            || (i == 1 && r + string_length >= height + 1)
            || (j == -1 && c + 1 < string_length)
            || (j == 1 && c + string_length >= width + 1)
        {
            continue;
        }

        for k in 1..4 {
            let r_new: usize = (r as isize + k * i as isize) as usize;
            let c_new: usize = (c as isize + k * j as isize) as usize;
            if grid[r_new][c_new] != word[k as usize] {
                continue 'outer;
            }
        }

        output += 1;
    }

    output
}
