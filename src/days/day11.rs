use crate::utils::{LinesIterator, Memoizer};
use std::collections::HashMap;

fn num_digits(n: u128) -> usize {
    (n.checked_ilog10().unwrap_or(0) + 1) as usize
}

fn blink_once(stone: u128) -> (u128, Option<u128>) {
    if stone == 0 {
        (1, None)
    } else {
        let n = num_digits(stone);
        if n % 2 == 0 {
            let half = n / 2;
            let divisor = 10u128.pow(half as u32);
            (stone / divisor, Some(stone % divisor))
        } else {
            (stone * 2024, None)
        }
    }
}

fn blink(stones: &[u128]) -> Vec<u128> {
    stones
        .iter()
        .flat_map(|n| match blink_once(*n) {
            (a, None) => vec![a],
            (a, Some(b)) => vec![a, b],
        })
        .collect()
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let mut nums: Vec<u128> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(str::parse::<u128>)
        .map(Result::unwrap)
        .collect();

    for _ in 0..25 {
        let x = blink(&nums);
        // println!("{} {}", i + 1, x.len());
        nums = x;
    }

    format!("{}", nums.len())
}

type Blinked = (u128, Option<u128>);

struct StoneCollapser {
    memoized_blink_once: Memoizer<fn(u128) -> Blinked, u128, Blinked>,
    map: HashMap<(u128, usize), usize>,
}

impl StoneCollapser {
    fn new() -> Self {
        let blink_once = Memoizer::new(blink_once as fn(u128) -> Blinked);

        Self {
            memoized_blink_once: blink_once,
            map: HashMap::new(),
        }
    }

    fn collapse_stone(&mut self, stone: u128, gens: usize) -> usize {
        if let Some(n) = self.map.get(&(stone, gens)) {
            return *n;
        }

        let digits = num_digits(stone);

        let output = match gens {
            0 => 1,
            1 => {
                if stone == 0 {
                    1
                } else if digits % 2 == 0 {
                    2
                } else {
                    1
                }
            }
            _ => match self.memoized_blink_once.call(stone) {
                (a, None) => self.collapse_stone(a, gens - 1),
                (a, Some(b)) => self.collapse_stone(a, gens - 1) + self.collapse_stone(b, gens - 1),
            },
        };

        self.map.insert((stone, gens), output);

        output
    }
}

pub fn run2(lines: &mut LinesIterator) -> String {
    let nums: Vec<u128> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(str::parse::<u128>)
        .map(Result::unwrap)
        .collect();

    let mut sc = StoneCollapser::new();
    let mut result = 0;

    let gens = 75;

    for stone in nums {
        result += sc.collapse_stone(stone, gens);
    }

    format!("{result}")
}
