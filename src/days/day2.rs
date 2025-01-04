use crate::utils::LinesIterator;

pub fn run1(lines: &mut LinesIterator) -> String {
    let num_safe = prepare_lines(lines)
        .filter(|line| check_safe_1(line))
        .count();
    format!("{num_safe}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    let num_safe = prepare_lines(lines)
        .filter(|line| check_safe_2_naive(line))
        .count();
    format!("{num_safe}")
}

// Lifetime annotation here is not actually needless, so we need to tell clippy to stop
// complaining.
#[allow(clippy::needless_lifetimes)]
fn prepare_lines<'a>(lines: &'a mut LinesIterator) -> impl Iterator<Item = Vec<i32>> + 'a {
    lines.map(Result::unwrap).map(|x| {
        x.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    })
}

fn check_safe_1(line: &[i32]) -> bool {
    let length = line.len();
    let is_decreasing: bool;
    if length >= 2 {
        let mut prev = line[0];
        let cur = line[1];
        is_decreasing = match cur - prev {
            -3..=-1 => true,
            1..=3 => false,
            _ => return false,
        };
        prev = cur;
        for cur in &line[2..] {
            if !matches!(
                (is_decreasing, *cur - prev),
                (true, -3..=-1) | (false, 1..=3)
            ) {
                return false;
            }
            prev = *cur;
        }
    }
    true
}

fn check_safe_2_naive(line: &[i32]) -> bool {
    for i in 0..line.len() {
        let mut removed = Vec::with_capacity(line.len() - 1);
        removed.extend_from_slice(&line[..i]);
        removed.extend_from_slice(&line[i + 1..]);
        if check_safe_1(&removed) {
            return true;
        }
    }
    false
}
