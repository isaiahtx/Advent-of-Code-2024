use crate::direction::{Coords, Direction};
use crate::utils::LinesIterator;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    Robot,
    Empty,
}

impl Tile {
    /// # Error
    ///
    const fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            _ => Self::Empty,
        }
    }

    const fn to_char(&self) -> char {
        match self {
            Self::Wall => '#',
            Self::Box => 'O',
            Self::Robot => '@',
            Self::Empty => '.',
        }
    }
}

struct Warehouse {
    grid: Vec<Vec<Tile>>,
    robot: Coords,
}

#[derive(Debug)]
enum WarehouseStepResult {
    BlockedByWall,
    BlockedByBox,
    Moved,
    Pushed,
}

impl Warehouse {
    fn step(&mut self, dir: Direction) -> WarehouseStepResult {
        let height = self.grid.len();
        let width = self.grid[0].len();

        if let Some((tgt_r, tgt_c)) = dir.step_coords(self.robot, height, width) {
            match self.grid[tgt_r][tgt_c] {
                // If we are able to step in the desired direction...
                Tile::Empty => {
                    // If the next tile is empty, then just move the robot
                    // there.
                    self.robot = (tgt_r, tgt_c);
                    return WarehouseStepResult::Moved;
                }

                // If the next tile is a wall, we are blocked, do nothing.
                Tile::Wall => return WarehouseStepResult::BlockedByWall,

                Tile::Box => {
                    // If the next tile over is a box, we need to check how
                    // many boxes there are after. To start, we will loop over
                    // boxes after the first box.
                    let mut next_over: Option<Coords> =
                        dir.step_coords((tgt_r, tgt_c), height, width);
                    loop {
                        match next_over {
                            // If the next tile over is unreachable, we hit a
                            // wall, so the boxes are blocked.
                            None => return WarehouseStepResult::BlockedByBox,

                            Some((r, c)) => match self.grid[r][c] {
                                Tile::Empty => {
                                    // If the next tile over is empty, then:

                                    // shift the boxes over
                                    self.grid[r][c] = Tile::Box;
                                    self.grid[tgt_r][tgt_c] = Tile::Empty;

                                    // move the robot
                                    self.robot = (tgt_r, tgt_c);

                                    // return that we pushed
                                    return WarehouseStepResult::Pushed;
                                }

                                // If the next tile over is a wall, the boxes
                                // are blocked.
                                Tile::Wall => return WarehouseStepResult::BlockedByBox,

                                // If the next tile over is a box, then we
                                // look to the next tile over and continue the
                                // loop.
                                Tile::Box => {
                                    next_over = dir.step_coords((r, c), height, width);
                                }

                                Tile::Robot => panic!("Warehouse grid should NOT contain a robot!"),
                            },
                        }
                    }
                }

                Tile::Robot => panic!("Warehouse grid should NOT contain a robot!"),
            }
        }

        // If we are not able to step in the desired direction, then we hit an edge, i.e., a wall.
        WarehouseStepResult::BlockedByWall
    }

    // fn move_until_blocked(&mut self, dir: Direction) {
    //     let mut n = 1;
    //     loop {
    //         print!("\tLoop {n}: ");
    //         n += 1;
    //         let output = self.step(dir);
    //         println!("{output:?}");
    //         if matches!(
    //             output,
    //             WarehouseStepResult::BlockedByBox | WarehouseStepResult::BlockedByWall
    //         ) {
    //             break;
    //         }
    //     }
    //     println!();
    // }

    fn sum(&self) -> usize {
        let mut output = 0;
        for (r, row) in self.grid.iter().enumerate() {
            for (c, t) in row.iter().enumerate() {
                if *t == Tile::Box {
                    output += 100 * (r + 1) + c + 1;
                }
            }
        }
        output
    }

    // fn peek_window(&self, w: usize, h: usize) -> String {
    //     let r = self.robot.0;
    //     let c = self.robot.1;

    //     let mut output = String::new();

    //     for (i, row) in self.grid.iter().enumerate() {
    //         if r < i + h && i < r + h {
    //             for (j, t) in row.iter().enumerate() {
    //                 if (i, j) == (r, c) {
    //                     output.push('@');
    //                 } else if c < j + w && j < c + w {
    //                     output.push(t.to_char());
    //                 }
    //             }
    //             output.push('\n');
    //         }
    //     }

    //     output
    // }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.grid[0].len();

        let mut output = "#".repeat(width + 2);
        output.push('\n');

        // Temporarily mutate the grid to place the robot
        let grid_ptr: *mut Vec<Tile>;
        unsafe {
            grid_ptr = self.grid.as_ptr().cast_mut();
            (*grid_ptr.add(self.robot.0))[self.robot.1] = Tile::Robot;
        }

        self.grid.iter().for_each(|row| {
            output.push('#');
            row.iter().map(Tile::to_char).for_each(|c| output.push(c));
            output.push_str("#\n");
        });

        // Remove robot from grid
        unsafe {
            (*grid_ptr.add(self.robot.0))[self.robot.1] = Tile::Empty;
        }

        output += &"#".repeat(width + 2);
        output += &format!("\n{:?}\n", self.robot);

        write!(f, "{output}")
    }
}

fn parse_input(lines: &mut LinesIterator) -> (Warehouse, Vec<Direction>) {
    lines.next();
    let mut prev_line = lines.next().unwrap().unwrap();
    let mut grid = Vec::new();
    let mut found = false;
    let mut r = 0;
    let mut c = 0;

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut tmp_line: Vec<char> = prev_line[1..prev_line.len() - 1].chars().collect();

        if !found {
            if let Some(j) = tmp_line.iter().position(|&c| c == '@') {
                found = true;
                tmp_line[j] = '.';
                c = j;
            } else {
                r += 1;
            }
        }

        grid.push(tmp_line.into_iter().map(Tile::from).collect());
        prev_line = line;
    }

    let mut moves = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        moves.extend(
            line.chars()
                .map(Direction::cardinal_from_char)
                .map(Result::unwrap),
        );
    }

    (
        Warehouse {
            grid,
            robot: (r, c),
        },
        moves,
    )
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let (mut wh, steps) = parse_input(lines);

    for dir in steps {
        wh.step(dir);
    }

    let output = wh.sum();

    format!("{output}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}
