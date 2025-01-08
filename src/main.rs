mod bimap;
mod days;
mod graph;
mod memoizer;
mod utils;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

pub type LinesIterator = std::io::Lines<std::io::BufReader<std::fs::File>>;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 4 {
        eprintln!(
            "Usage: {} <day_number> <part_1_or_2> [path (optional)]",
            args[0]
        );
        process::exit(1);
    }

    let day_number: u8 = match args[1].parse() {
        Ok(n) if n > 0 => n,
        _ => {
            eprintln!("Please provide a valid positive integer for the day number");
            process::exit(1);
        }
    };

    let part: u8 = match args[2].parse() {
        Ok(n) if (n == 1) || (n == 2) => n,
        _ => {
            eprintln!("Please provide either 1 or 2 to indicate which part");
            process::exit(1);
        }
    };

    let path: String = if args.len() == 4 {
        args[3].clone()
    } else {
        format!("./inputs/input{day_number}.txt")
    };

    println!("Running part {part} of day {day_number} using input {path}.");
    println!();

    let mut lines: LinesIterator = read_lines(path).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    let output = match part {
        1 => match day_number {
            1 => days::day1::run1(&mut lines),
            2 => days::day2::run1(&mut lines),
            3 => days::day3::run1(&mut lines),
            4 => days::day4::run1(&mut lines),
            5 => days::day5::run1(&mut lines),
            6 => days::day6::run1(&mut lines),
            7 => days::day7::run1(&mut lines),
            8 => days::day8::run1(&mut lines),
            9 => days::day9::run1(&mut lines),
            10 => days::day10::run1(&mut lines),
            11 => days::day11::run1(&mut lines),
            12 => days::day12::run1(&mut lines),
            13 => days::day13::run1(&mut lines),
            14 => days::day14::run1(&mut lines),
            15 => days::day15::run1(&mut lines),
            16 => days::day16::run1(&mut lines),
            17 => days::day17::run1(&mut lines),
            18 => days::day18::run1(&mut lines),
            19 => days::day19::run1(&mut lines),
            20 => days::day20::run1(&mut lines),
            21 => days::day21::run1(&mut lines),
            22 => days::day22::run1(&mut lines),
            23 => days::day23::run1(&mut lines),
            24 => days::day24::run1(&mut lines),
            25 => days::day25::run1(&mut lines),

            _ => panic!("huh?"),
        },
        2 => match day_number {
            1 => days::day1::run2(&mut lines),
            2 => days::day2::run2(&mut lines),
            3 => days::day3::run2(&mut lines),
            4 => days::day4::run2(&mut lines),
            5 => days::day5::run2(&mut lines),
            6 => days::day6::run2(&mut lines),
            7 => days::day7::run2(&mut lines),
            8 => days::day8::run2(&mut lines),
            9 => days::day9::run2(&mut lines),
            10 => days::day10::run2(&mut lines),
            11 => days::day11::run2(&mut lines),
            12 => days::day12::run2(&mut lines),
            13 => days::day13::run2(&mut lines),
            14 => days::day14::run2(&mut lines),
            15 => days::day15::run2(&mut lines),
            16 => days::day16::run2(&mut lines),
            17 => days::day17::run2(&mut lines),
            18 => days::day18::run2(&mut lines),
            19 => days::day19::run2(&mut lines),
            20 => days::day20::run2(&mut lines),
            21 => days::day21::run2(&mut lines),
            22 => days::day22::run2(&mut lines),
            23 => days::day23::run2(&mut lines),
            24 => days::day24::run2(&mut lines),
            25 => days::day25::run2(&mut lines),
            _ => panic!("huh?"),
        },
        _ => {
            panic!("huh?");
        }
    };

    println!("{output}");
}
fn read_lines<P>(filename: P) -> io::Result<LinesIterator>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
