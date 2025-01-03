mod days;
mod utils;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <day_number> <part_1_or_2>", args[0]);
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

    let path = format!("./inputs/input{day_number}.txt");
    let lines = read_lines(path).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    match day_number {
        1 => days::day1::run(lines, part),
        2 => days::day2::run(lines, part),
        3 => days::day3::run(lines, part),
        4 => days::day4::run(lines, part),
        5 => days::day5::run(lines, part),
        6 => days::day6::run(lines, part),
        7 => days::day7::run(lines, part),
        8 => days::day8::run(lines, part),
        9 => days::day9::run(lines, part),
        10 => days::day10::run(lines, part),
        11 => days::day11::run(lines, part),
        12 => days::day12::run(lines, part),
        13 => days::day13::run(lines, part),
        14 => days::day14::run(lines, part),
        15 => days::day15::run(lines, part),
        16 => days::day16::run(lines, part),
        17 => days::day17::run(lines, part),
        18 => days::day18::run(lines, part),
        19 => days::day19::run(lines, part),
        20 => days::day20::run(lines, part),
        21 => days::day21::run(lines, part),
        22 => days::day22::run(lines, part),
        23 => days::day23::run(lines, part),
        24 => days::day24::run(lines, part),
        25 => days::day25::run(lines, part),
        _ => println!("no!"),
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
