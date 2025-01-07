use crate::utils::LinesIterator;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::VecDeque;

// I was crossed and sleep deprived when I wrote this function no idea how it works tbh
fn parse(line: &[u32]) -> usize {
    // print_vec_num(line);
    let length = line.len();
    let mut to_borrow: VecDeque<(usize, u32)> = VecDeque::new();
    let mut j = if length % 2 == 1 { length + 1 } else { length };
    let mut k = 0;
    let mut result = 0;
    assert!(j % 2 == 0);

    // let mut compressed_row = Vec::new();
    'outer: for (i, num) in line.iter().enumerate() {
        // println!("----");
        let num = *num;
        if i % 2 == 0 {
            if i >= j {
                assert!(to_borrow.len() == 1);
                for _ in 0..to_borrow[0].1 {
                    // compressed_row.push(to_borrow[0].0);
                    result += to_borrow[0].0 * k;
                    k += 1;
                }
                break;
            }
            for _ in 0..num {
                // compressed_row.push(i / 2);
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
                // compressed_row.push(id);
                to_borrow[0].1 -= 1;
                result += id * k;
                k += 1;
            }
        }
        // print_vec_num(&compressed_row);
        // print_vecdq_num(&to_borrow);
        // println!("i:{i}");
        // println!("j:{j}");
    }

    // print_vec_num(&compressed_row);

    result
}

fn move_back(row: &mut Vec<(Option<usize>, u32)>, ent: (usize, u32)) -> &Vec<(Option<usize>, u32)> {
    let (tgt_id, tgt_num) = ent;
    let mut last = 0;
    for i in 0..row.len() {
        let (id_opt, num) = row[i];
        if let Some(id) = id_opt {
            if id == tgt_id {
                // println!("did not move");
                return row;
            }
        } else {
            assert!(id_opt.is_none());
            match num.cmp(&tgt_num) {
                Ordering::Less => continue,
                Ordering::Greater => {
                    // println!("inserted");
                    row[i].1 -= tgt_num;
                    row.insert(i, (Some(tgt_id), tgt_num));
                    last = i + 2;
                    break;
                }
                Ordering::Equal => {
                    // println!("replaced");
                    row[i] = (Some(tgt_id), tgt_num);
                    last = i + 1;
                    break;
                }
            }
        }
    }

    for (j, (id, num)) in row.iter().skip(last).enumerate() {
        if let Some(id) = *id {
            if id == tgt_id {
                assert!(*num == tgt_num);
                row[last + j].0 = None;
                break;
            }
        }
    }

    row
}

#[allow(dead_code)]
fn print_row(row: &[(Option<usize>, u32)]) {
    for (x, y) in row {
        let x = *x;
        let y = *y;
        match x {
            None => print!("{}", ".".repeat(y as usize)),
            Some(id) => print!("{}", id.to_string().repeat(y as usize)),
        }
    }
    println!();
}

fn parse_2(line: &[u32]) -> u64 {
    let mut row: Vec<(Option<usize>, u32)> = Vec::new();
    let mut to_move: Vec<(usize, u32)> = Vec::new();
    for (i, num) in line.iter().enumerate() {
        if i % 2 == 0 {
            row.push((Some(i / 2), *num));
            to_move.push((i / 2, *num));
        } else {
            row.push((None, *num));
        }
    }

    // print_row(&row);
    for ent in to_move.iter().rev() {
        move_back(&mut row, *ent);
        // print_row(&row);
    }

    let mut i = 0;
    let mut result = 0;

    for (id_opt, num) in row {
        if let Some(id) = id_opt {
            result += id as u64 * u64::from((num * i) + (((num - 1) * num) / 2));
        }
        i += num;
    }

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
fn print_vecdq_num(v: &VecDeque<(usize, u32)>) {
    for n in v {
        print!("{n:?}");
    }
    println!();
}

#[allow(dead_code)]
fn print_vec_num<T: std::fmt::Debug>(v: &[T]) {
    for n in v {
        print!("{n:?}");
    }
    println!();
}

pub fn run2(lines: &mut LinesIterator) -> String {
    let result = parse_2(
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
fn generate_random_vector() -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(1..=100); // Random length between 1 and 100
    (0..length)
        .map(|_| rng.gen_range(0..10) as u32) // Random single-digit integers (0 to 9)
        .collect()
}
