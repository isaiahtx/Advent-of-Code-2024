use std::fmt::Debug;

pub fn run<T: Iterator<Item: Debug> + Debug>(lines: T, part: u8) {
    println!("Running part {part} of day 1!");
}
