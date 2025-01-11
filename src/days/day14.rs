use crate::utils::LinesIterator;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;
use std::io::Write;
use std::marker::PhantomData;

type Coords = (i32, i32);
type Velocity = (i32, i32);

const HEIGHT: i32 = 103;
const WIDTH: i32 = 101;

fn parse_input(lines: &mut LinesIterator) -> Vec<(Coords, Velocity)> {
    let mut output = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let mut line = line.split(' ');

        let mut pos = line.next().unwrap()[2..].split(',');
        let p2 = pos.next().unwrap().parse::<i32>().unwrap();
        let p1 = pos.next().unwrap().parse::<i32>().unwrap();

        let mut vel = line.next().unwrap()[2..].split(',');
        let v2 = vel.next().unwrap().parse::<i32>().unwrap();
        let v1 = vel.next().unwrap().parse::<i32>().unwrap();

        output.push(((p1, p2), (v1, v2)));
    }

    output
}

const fn update(robot: (Coords, Velocity), seconds: i32) -> (Coords, Velocity) {
    let p1 = robot.0 .0;
    let p2 = robot.0 .1;

    let v1 = robot.1 .0;
    let v2 = robot.1 .1;

    let new_p1 = (p1 + v1 * seconds).rem_euclid(HEIGHT);
    let new_p2 = (p2 + v2 * seconds).rem_euclid(WIDTH);

    ((new_p1, new_p2), (v1, v2))
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let robots = parse_input(lines);

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for robot in robots {
        let height_middle = (HEIGHT - 1) / 2;
        let width_middle = (WIDTH - 1) / 2;

        let (p1, p2) = update(robot, 100).0;

        match (p1.cmp(&height_middle), p2.cmp(&width_middle)) {
            (Less, Less) => top_left += 1,
            (Less, Greater) => top_right += 1,
            (Greater, Less) => bottom_left += 1,
            (Greater, Greater) => bottom_right += 1,
            _ => {}
        }
    }

    let output = top_left * top_right * bottom_left * bottom_right;

    format!("{output}")
}

fn chars_grid_to_string(chars: &Vec<Vec<char>>) -> String {
    chars
        .into_iter()
        .map(|l| {
            let mut s = l.into_iter().collect::<String>();
            s.push('\n');
            s
        })
        .collect()
}

fn robots_to_grid(robots: &Vec<(Coords, Velocity)>) -> Vec<Vec<char>> {
    let mut chars = Vec::with_capacity(HEIGHT as usize);

    for _ in 0..HEIGHT {
        chars.push(Vec::from([' '; WIDTH as usize]));
    }

    for ((r, c), _) in robots {
        chars[*r as usize][*c as usize] = '█';
    }

    chars
}

pub fn run2(lines: &mut LinesIterator) -> String {
    let mut robots = parse_input(lines);
    let mut _b = String::new();
    let mut n = 0;

    loop {
        let chars = robots_to_grid(&robots);

        for (i, row) in chars.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if i + 4 < HEIGHT as usize
                    && *c == '█'
                    && chars[i + 1][j] == '█'
                    && chars[i + 2][j] == '█'
                    && chars[i + 3][j] == '█'
                    && chars[i + 4][j] == '█'
                {
                    println!("\r{}\n\n", chars_grid_to_string(&chars));
                    println!("{n}");
                    io::stdout().flush().unwrap();

                    let _ = io::stdin().read_line(&mut _b);
                }
            }
        }

        robots = robots.into_iter().map(|r| update(r, 1)).collect();
        n += 1;
    }
}
