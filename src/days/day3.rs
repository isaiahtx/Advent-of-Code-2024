use crate::utils::LinesIterator;

use regex::Regex;

/// # Panics
pub fn run1(lines: &mut LinesIterator) -> String {
    let mut result = 0;
    for line in lines.map(Result::unwrap) {
        result += evaluate_1(&line).unwrap();
    }
    format!("{result}")
}

/// # Panics
pub fn run2(lines: &mut LinesIterator) -> String {
    let mut result = 0;
    let mut do_multiply = true;
    for line in lines.map(Result::unwrap) {
        result += evaluate_2(&line, &mut do_multiply).unwrap();
    }
    format!("{result}")
}

fn evaluate_1(s: &str) -> Result<i64, std::num::ParseIntError> {
    let mut output: i64 = 0;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    for (_, [n1, n2]) in re.captures_iter(s).map(|c| c.extract()) {
        output += n1.parse::<i64>()? * n2.parse::<i64>()?;
    }
    Ok(output)
}

fn evaluate_2(s: &str, do_multiply: &mut bool) -> Result<i64, std::num::ParseIntError> {
    let mut output: i64 = 0;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don\'t\(\)").unwrap();
    for line in re.captures_iter(s) {
        match &line[0][..3] {
            "mul" => {
                if *do_multiply {
                    output += line[1].parse::<i64>()? * line[2].parse::<i64>()?;
                }
            }
            "do(" => *do_multiply = true,
            "don" => *do_multiply = false,
            _ => panic!("this should never happen"),
        }
    }
    Ok(output)
}
