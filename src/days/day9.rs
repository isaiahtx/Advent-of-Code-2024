use crate::utils::LinesIterator;
use rand::Rng;
use std::collections::VecDeque;

fn parse(line: &[u32]) -> usize {
    // print_vec_num(line);
    let length = line.len();
    let mut to_borrow: VecDeque<(usize, u32)> = VecDeque::new();
    let mut j = if length % 2 == 1 { length + 1 } else { length };
    let mut k = 0;
    let mut result = 0;
    assert!(j % 2 == 0);

    // let mut output = Vec::new();
    'outer: for (i, num) in line.iter().enumerate() {
        // println!("----");
        let num = *num;
        if i % 2 == 0 {
            if i >= j {
                assert!(to_borrow.len() == 1);
                for _ in 0..to_borrow[0].1 {
                    // output.push(to_borrow[0].0);
                    result += to_borrow[0].0 * k;
                    k += 1;
                }
                break;
            }
            for _ in 0..num {
                // output.push(i / 2);
                result += (i / 2) * k;
                k += 1;
            }
        } else {
            for _ in 0..num {
                while to_borrow.is_empty() || to_borrow[0].1 == 0 {
                    j -= 2;
                    if j <= i {
                        break 'outer;
                    }
                    to_borrow.push_back((j / 2, line[j]));
                    if to_borrow[0].1 == 0 {
                        to_borrow.pop_front();
                    }
                }
                let id = to_borrow[0].0;
                // output.push(id);
                to_borrow[0].1 -= 1;
                result += id * k;
                k += 1;
            }
        }
        // print_vec_num(&output);
        // print_vecdq_num(&to_borrow);
        // println!("i:{i}");
        // println!("j:{j}");
    }

    // print_vec_num(&output);

    result
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let result = parse(
        &lines
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>(),
    );

    format!("{result}")
}

#[allow(dead_code)]
pub fn print_vecdq_num(v: &VecDeque<(usize, u32)>) {
    for n in v {
        print!("{n:?}");
    }
    println!();
}

#[allow(dead_code)]
pub fn print_vec_num<T: std::fmt::Debug>(v: &[T]) {
    for n in v {
        print!("{n:?}");
    }
    println!();
}

pub fn run2(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}

#[allow(dead_code)]
fn generate_random_vector() -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(1..=100); // Random length between 1 and 100
    (0..length)
        .map(|_| rng.gen_range(0..10) as u32) // Random single-digit integers (0 to 9)
        .collect()
}
