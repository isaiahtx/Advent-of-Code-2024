use std::cmp::Ordering;

use crate::utils::LinesIterator;

/// # Panics
pub fn run1(lines: &mut LinesIterator) -> String {
    let mut edges: Vec<(usize, usize)> = vec![];
    let mut updates: Vec<Vec<usize>> = vec![];

    while let Some(Ok(line)) = lines.next() {
        if line.contains('|') {
            let v: Vec<&str> = line.split('|').collect();
            edges.push((v[0].parse().unwrap(), v[1].parse().unwrap()));
        } else if line.contains(',') {
            let v: Vec<usize> = line
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            updates.push(v);
        }
    }

    let compare = |x: &usize, y: &usize| !edges.contains(&(*y, *x));

    let mut output = 0;
    for update in updates {
        if update.is_sorted_by(compare) {
            output += update[update.len() / 2];
        }
    }
    format!("{output}")
}

// pub fn run1_old(lines: &mut LinesIterator) -> String {
//     let mut edges: Vec<(usize, usize)> = vec![];
//     let mut updates: Vec<Vec<usize>> = vec![];
//
//     while let Some(Ok(line)) = lines.next() {
//         if line.contains('|') {
//             let v: Vec<&str> = line.split('|').collect();
//             edges.push((v[0].parse().unwrap(), v[1].parse().unwrap()));
//         } else if line.contains(',') {
//             let v: Vec<usize> = line
//                 .split(',')
//                 .map(|x| x.parse::<usize>().unwrap())
//                 .collect();
//             updates.push(v);
//         }
//     }
//
//     let graph: DirectedCsrGraph<usize> = GraphBuilder::new().edges(edges).build();
//     let mut output = 0;
//     for update in updates {
//         let mut passed = true;
//
//         'break_me: for i in 0..update.len() {
//             for x in graph.in_neighbors(update[i]).filter(|y| update.contains(y)) {
//                 if !update[..i].contains(x) {
//                     passed = false;
//                     break 'break_me;
//                 }
//             }
//         }
//
//         if passed {
//             output += update[(update.len() - 1) / 2];
//         }
//     }
//
//     format!("{output}")
// }

/// # Panics
pub fn run2(lines: &mut LinesIterator) -> String {
    let mut edges: Vec<(usize, usize)> = vec![];
    let mut updates: Vec<Vec<usize>> = vec![];

    while let Some(Ok(line)) = lines.next() {
        if line.contains('|') {
            let v: Vec<&str> = line.split('|').collect();
            edges.push((v[0].parse().unwrap(), v[1].parse().unwrap()));
        } else if line.contains(',') {
            let v: Vec<usize> = line
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            updates.push(v);
        }
    }

    let compare = |x: &usize, y: &usize| {
        if edges.contains(&(*x, *y)) {
            Ordering::Less
        } else if edges.contains(&(*y, *x)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    let mut output = 0;
    for update in &mut updates {
        if !update.is_sorted_by(|a, b| !matches!(compare(a, b), Ordering::Greater)) {
            update.sort_by(compare);
            output += update[update.len() / 2];
        }
    }
    format!("{output}")
}
