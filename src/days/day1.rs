use crate::utils::LinesIterator;
use std::collections::HashMap;

pub fn run1(lines: &mut LinesIterator) -> String {
    let mut l = vec![];
    let mut r = vec![];
    for temp_line in lines {
        let line = temp_line.unwrap();
        let mut nums = line.split_whitespace().map(|x| x.parse::<i64>().unwrap());
        l.push(nums.next().unwrap());
        r.push(nums.next().unwrap());
    }

    l.sort_unstable();
    r.sort_unstable();

    let mut total_dist = 0;

    for i in 0..l.len() {
        total_dist += (l[i] - r[i]).abs();
    }

    format!("{total_dist}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    let mut lcount: HashMap<i64, i64> = HashMap::new();
    let mut rcount: HashMap<i64, i64> = HashMap::new();

    for temp_line in lines {
        let line = temp_line.unwrap();
        let mut nums = line.split_whitespace().map(|x| x.parse::<i64>().unwrap());
        let l = nums.next().unwrap();
        let r = nums.next().unwrap();

        match rcount.get(&r) {
            Some(&x) => rcount.insert(r, x + 1),
            None => rcount.insert(r, 1),
        };
        match lcount.get(&l) {
            Some(&x) => lcount.insert(l, &x + 1),
            None => lcount.insert(l, 1),
        };
    }

    let mut output = 0;

    for key in lcount.keys() {
        if rcount.contains_key(key) {
            output += *key * lcount[key] * rcount[key];
        }
    }

    format!("{output}")
}
