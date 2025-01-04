mod days;
mod utils;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

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

    let mut lines: days::LinesIterator = read_lines(path).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    let output = match part {
        1 => match day_number {
            1 => days::day1::run1(&mut lines),
            2 => days::day2::run1(&mut lines),
            3 => days::day3::run1(&mut lines),
            _ => panic!("huh?"),
        },
        2 => match day_number {
            1 => days::day1::run2(&mut lines),
            2 => days::day2::run2(&mut lines),
            3 => days::day3::run2(&mut lines),
            _ => panic!("huh?"),
        },
        _ => {
            panic!("huh?");
        }
    };

    println!("{output}");
}
fn read_lines<P>(filename: P) -> io::Result<days::LinesIterator>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
