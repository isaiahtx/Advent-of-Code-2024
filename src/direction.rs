#![allow(dead_code)]

pub type Coords = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
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
    /// # Errors
    ///
    /// Only accepts inputs 0 through 7 (inclusive)
    pub fn from(n: usize) -> Result<Self, String> {
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

    /// # Errors
    ///
    pub fn cardinal_from_char(c: char) -> Result<Self, String> {
        match c {
            '^' => Ok(Self::N),
            '>' => Ok(Self::E),
            'v' => Ok(Self::S),
            '<' => Ok(Self::W),
            _ => Err("invalid entry".to_string()),
        }
    }

    #[must_use]
    pub const fn turn_right(self) -> Self {
        match self {
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

    #[must_use]
    pub const fn to_num(self) -> usize {
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

    #[must_use]
    pub const fn reflect(self) -> Self {
        match self {
            Self::N => Self::S,
            Self::NE => Self::SW,
            Self::E => Self::W,
            Self::SE => Self::NW,
            Self::S => Self::N,
            Self::SW => Self::NE,
            Self::W => Self::E,
            Self::NW => Self::SE,
        }
    }

    #[must_use]
    pub const fn combine_cardinal(self, dir: Self) -> Option<Self> {
        let mut left = self;
        let mut right = dir;

        if left.to_num() > right.to_num() {
            left = dir;
            right = self;
        }
        match (left, right) {
            (Self::N, Self::E) => Some(Self::NE),
            (Self::E, Self::S) => Some(Self::SE),
            (Self::S, Self::W) => Some(Self::SW),
            (Self::N, Self::W) => Some(Self::NW),
            _ => None,
        }
    }

    #[must_use]
    pub const fn to_coords(self) -> (i8, i8) {
        match self {
            Self::N => (-1, 0),
            Self::NE => (-1, 1),
            Self::E => (0, 1),
            Self::SE => (1, 1),
            Self::S => (1, 0),
            Self::SW => (1, -1),
            Self::W => (0, -1),
            Self::NW => (-1, -1),
        }
    }

    #[must_use]
    pub const fn step_coords_cardinal_unchecked(self, coords: Coords) -> Coords {
        match self {
            Self::N => (coords.0 - 1, coords.1),
            Self::E => (coords.0, coords.1 + 1),
            Self::S => (coords.0 + 1, coords.1),
            Self::W => (coords.0, coords.1 - 1),
            _ => panic!("only takes cardinal values!"),
        }
    }

    #[must_use]
    pub const fn step_coords_unchecked(self, coords: Coords) -> Coords {
        let (dx, dy) = self.to_coords();
        let mut x = coords.0;
        let mut y = coords.1;

        if dx < 0 {
            x -= 1;
        } else if dx > 0 {
            x += 1;
        }

        if dy < 0 {
            y -= 1;
        } else if dy > 0 {
            y += 1;
        }

        (x, y)
    }

    #[must_use]
    pub const fn step_coords(self, coords: Coords, height: usize, width: usize) -> Option<Coords> {
        let mut x = coords.0;
        let mut y = coords.1;
        let (dx, dy) = self.to_coords();
        if dx < 0 {
            if x == 0 {
                return None;
            }
            x -= 1;
        } else if dx > 0 {
            if x + 1 >= height {
                return None;
            }
            x += 1;
        }

        if dy < 0 {
            if y == 0 {
                return None;
            }
            y -= 1;
        } else if dy > 0 {
            if y + 1 >= width {
                return None;
            }
            y += 1;
        }

        Some((x, y))
    }
}
