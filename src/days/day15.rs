use crate::direction::{Coords, Direction};
use crate::utils::LinesIterator;
use std::collections::{HashSet, VecDeque};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    const fn to_char(self) -> char {
        match self {
            Self::Wall => '#',
            Self::Box => 'O',
            Self::Robot => '@',
            Self::Empty => '.',
        }
    }
}

#[derive(Debug, Clone)]
struct Warehouse {
    grid: Vec<Vec<Tile>>,
    robot: Coords,
}

#[derive(Debug)]
enum StepResult {
    BlockedByWall,
    BlockedByBox,
    Moved,
    Pushed,
}

impl Warehouse {
    fn step(&mut self, dir: Direction) -> StepResult {
        let height = self.grid.len();
        let width = self.grid[0].len();

        if let Some((tgt_r, tgt_c)) = dir.step_coords(self.robot, height, width) {
            match self.grid[tgt_r][tgt_c] {
                // If we are able to step in the desired direction...
                Tile::Empty => {
                    // If the next tile is empty, then just move the robot
                    // there.
                    self.robot = (tgt_r, tgt_c);
                    return StepResult::Moved;
                }

                // If the next tile is a wall, we are blocked, do nothing.
                Tile::Wall => return StepResult::BlockedByWall,

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
                            None => return StepResult::BlockedByBox,

                            Some((r, c)) => match self.grid[r][c] {
                                Tile::Empty => {
                                    // If the next tile over is empty, then:

                                    // shift the boxes over
                                    self.grid[r][c] = Tile::Box;
                                    self.grid[tgt_r][tgt_c] = Tile::Empty;

                                    // move the robot
                                    self.robot = (tgt_r, tgt_c);

                                    // return that we pushed
                                    return StepResult::Pushed;
                                }

                                // If the next tile over is a wall, the boxes
                                // are blocked.
                                Tile::Wall => return StepResult::BlockedByBox,

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
        StepResult::BlockedByWall
    }

    fn sum(&self) -> usize {
        let mut output = 0;
        for (r, row) in self.grid.iter().enumerate() {
            for (c, t) in row.iter().enumerate() {
                if *t == Tile::Box {
                    output += 100 * r + c;
                }
            }
        }
        output
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        // Temporarily mutate the grid to place the robot
        let grid_ptr: *mut Vec<Tile>;
        unsafe {
            grid_ptr = self.grid.as_ptr().cast_mut();
            (*grid_ptr.add(self.robot.0))[self.robot.1] = Tile::Robot;
        }

        self.grid.iter().for_each(|row| {
            row.iter()
                .map(|&x| x.to_char())
                .for_each(|c| output.push(c));
            output.push('\n');
        });

        // Remove robot from grid
        unsafe {
            (*grid_ptr.add(self.robot.0))[self.robot.1] = Tile::Empty;
        }

        output += &format!("\n{:?}\n", self.robot);

        write!(f, "{output}")
    }
}

fn parse_input(lines: &mut LinesIterator) -> (Warehouse, Vec<Direction>) {
    // let mut prev_line = lines.next().unwrap().unwrap();
    let mut grid = Vec::new();
    let mut found = false;
    let mut r = 0;
    let mut c = 0;

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut tmp_line: Vec<char> = line.chars().collect();

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum WideTile {
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
    Empty,
}

impl WideTile {
    fn flip_box(self) -> Result<Self, String> {
        match self {
            Self::BoxLeft => Ok(Self::BoxRight),
            Self::BoxRight => Ok(Self::BoxLeft),
            _ => Err("Can only call on BoxLeft or BoxRight".to_string()),
        }
    }
}

impl WideTile {
    const fn from_tile(t: Tile) -> [Self; 2] {
        match t {
            Tile::Wall => [Self::Wall, Self::Wall],
            Tile::Box => [Self::BoxLeft, Self::BoxRight],
            Tile::Robot => [Self::Robot, Self::Empty],
            Tile::Empty => [Self::Empty, Self::Empty],
        }
    }

    const fn to_char(self) -> char {
        match self {
            Self::Wall => '#',
            Self::BoxLeft => '[',
            Self::BoxRight => ']',
            Self::Robot => '@',
            Self::Empty => '.',
        }
    }
}

struct WideWarehouse {
    grid: Vec<Vec<WideTile>>,
    robot: Coords,
}

impl WideWarehouse {
    /// This function makes basically no checks as to whether or not the grid
    /// is valid, it simply assumes it is.
    fn step(&mut self, dir: Direction) -> StepResult {
        let height = self.grid.len();
        let width = self.grid[0].len();
        let (tgt_r, tgt_c) = dir.step_coords(self.robot, height, width).unwrap();

        match self.grid[tgt_r][tgt_c] {
            // If we are able to step in the desired direction...
            WideTile::Empty => {
                // If the next tile is empty, then just move the robot
                // there.
                self.robot = (tgt_r, tgt_c);
                StepResult::Moved
            }

            WideTile::Wall => StepResult::BlockedByWall,

            _ => match dir {
                Direction::N | Direction::S => {
                    let mut to_check = VecDeque::new();
                    let mut to_move = Vec::new();
                    let mut added_to_to_move = HashSet::new();

                    if self.grid[tgt_r][tgt_c] == WideTile::BoxRight {
                        to_check.push_back((tgt_r, tgt_c));
                        to_move.push((tgt_r, tgt_c));
                        added_to_to_move.insert((tgt_r, tgt_c));
                    } else {
                        to_check.push_back((tgt_r, tgt_c + 1));
                        to_move.push((tgt_r, tgt_c + 1));
                        added_to_to_move.insert((tgt_r, tgt_c + 1));
                    }

                    while !to_check.is_empty() {
                        let (cur_r, cur_c) = to_check.pop_front().unwrap();
                        let (r, c) = dir.step_coords_unchecked((cur_r, cur_c));

                        let adj_right = self.grid[r][c];
                        let adj_left = self.grid[r][c - 1];

                        if adj_right == WideTile::Wall || adj_left == WideTile::Wall {
                            // ?#  or  #?
                            // []      []
                            return StepResult::BlockedByBox;
                        }

                        if adj_right == WideTile::BoxRight {
                            // []
                            // []
                            to_check.push_back((r, c));
                            to_move.push((r, c));
                            continue;
                        }

                        if adj_right == WideTile::BoxLeft && added_to_to_move.insert((r, c + 1)) {
                            // ?[]
                            // []
                            to_check.push_back((r, c + 1));
                            to_move.push((r, c + 1));
                        }

                        if adj_left == WideTile::BoxRight && added_to_to_move.insert((r, c - 1)) {
                            // []?
                            //  []
                            to_check.push_back((r, c - 1));
                            to_move.push((r, c - 1));
                        }
                    }

                    while let Some((before_r, before_c)) = to_move.pop() {
                        let (after_r, after_c) = dir.step_coords_unchecked((before_r, before_c));
                        self.grid[after_r][after_c] = WideTile::BoxRight;
                        self.grid[after_r][after_c - 1] = WideTile::BoxLeft;
                        self.grid[before_r][before_c] = WideTile::Empty;
                        self.grid[before_r][before_c - 1] = WideTile::Empty;
                    }

                    self.robot = (tgt_r, tgt_c);
                    // self.grid[tgt_r][tgt_c] = WideTile::Empty;
                    // self.grid[tgt_r][tgt_c - 1] = WideTile::Empty;
                    StepResult::Pushed
                }
                Direction::E | Direction::W => {
                    let (mut next_r, mut next_c) =
                        dir.step_coords_unchecked(dir.step_coords_unchecked((tgt_r, tgt_c)));

                    loop {
                        match self.grid[next_r][next_c] {
                            WideTile::Wall => {
                                return StepResult::BlockedByBox;
                            }
                            WideTile::BoxRight | WideTile::BoxLeft => {
                                (next_r, next_c) = dir.step_coords_unchecked(
                                    dir.step_coords_unchecked((next_r, next_c)),
                                );
                            }
                            _ => break,
                        }
                    }

                    let mut c = dir.step_coords_unchecked((tgt_r, tgt_c)).1;
                    let mut side = self.grid[tgt_r][c].flip_box().unwrap();
                    while c != next_c {
                        self.grid[tgt_r][c] = side;
                        side = side.flip_box().unwrap();
                        c = dir.step_coords_unchecked((tgt_r, c)).1;
                    }
                    self.grid[tgt_r][c] = side;

                    self.grid[tgt_r][tgt_c] = WideTile::Empty;
                    self.robot = (tgt_r, tgt_c);
                    StepResult::Pushed
                }
                _ => panic!("Direction must be a cardinal"),
            },
        }
    }

    fn sum(&self) -> usize {
        let mut output = 0;
        for (r, row) in self.grid.iter().enumerate() {
            for (c, t) in row.iter().enumerate() {
                if *t == WideTile::BoxLeft {
                    output += 100 * r + c;
                }
            }
        }
        output
    }
}

impl Display for WideWarehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        // Temporarily mutate the grid to place the robot
        let grid_ptr: *mut Vec<WideTile>;
        unsafe {
            grid_ptr = self.grid.as_ptr().cast_mut();
            (*grid_ptr.add(self.robot.0))[self.robot.1] = WideTile::Robot;
        }

        self.grid.iter().for_each(|row| {
            row.iter()
                .map(|&x| x.to_char())
                .for_each(|c| output.push(c));
            output.push('\n');
        });

        // Remove robot from grid
        unsafe {
            (*grid_ptr.add(self.robot.0))[self.robot.1] = WideTile::Empty;
        }

        output += &format!("\n{:?}\n", self.robot);

        write!(f, "{output}")
    }
}

fn warehouse_to_wide_warehouse(wh: &Warehouse) -> WideWarehouse {
    let grid: Vec<Vec<WideTile>> = wh
        .grid
        .iter()
        .map(|v| v.iter().flat_map(|&t| WideTile::from_tile(t)).collect())
        .collect();

    WideWarehouse {
        grid,
        robot: (wh.robot.0, wh.robot.1 * 2),
    }
}

pub fn run2(lines: &mut LinesIterator) -> String {
    let (wh, steps) = parse_input(lines);
    let mut wwh = warehouse_to_wide_warehouse(&wh);

    for dir in steps {
        wwh.step(dir);
    }

    let output = wwh.sum();

    format!("{output}")
}
