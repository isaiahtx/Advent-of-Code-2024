use crate::{graph::shortest_path_length, utils::LinesIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NumKey {
    A,
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
}

impl NumKey {
    const fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            '0' => Self::N0,
            '1' => Self::N1,
            '2' => Self::N2,
            '3' => Self::N3,
            '4' => Self::N4,
            '5' => Self::N5,
            '6' => Self::N6,
            '7' => Self::N7,
            '8' => Self::N8,
            '9' => Self::N9,
            _ => panic!("invalid input!"),
        }
    }

    const fn to_char(self) -> char {
        match self {
            Self::A => 'A',
            Self::N0 => '0',
            Self::N1 => '1',
            Self::N2 => '2',
            Self::N3 => '3',
            Self::N4 => '4',
            Self::N5 => '5',
            Self::N6 => '6',
            Self::N7 => '7',
            Self::N8 => '8',
            Self::N9 => '9',
        }
    }

    fn get_nbrs(self) -> Vec<(DirKey, Self)> {
        match self {
            Self::A => vec![(DirKey::W, Self::N0), (DirKey::N, Self::N3)],
            Self::N0 => vec![(DirKey::E, Self::A), (DirKey::N, Self::N2)],
            Self::N1 => vec![(DirKey::E, Self::N2), (DirKey::N, Self::N4)],
            Self::N2 => vec![
                (DirKey::E, Self::N3),
                (DirKey::S, Self::N0),
                (DirKey::W, Self::N1),
                (DirKey::N, Self::N5),
            ],
            Self::N3 => vec![
                (DirKey::S, Self::A),
                (DirKey::W, Self::N2),
                (DirKey::N, Self::N6),
            ],
            Self::N4 => vec![
                (DirKey::E, Self::N5),
                (DirKey::N, Self::N7),
                (DirKey::S, Self::N1),
            ],
            Self::N5 => vec![
                (DirKey::E, Self::N6),
                (DirKey::S, Self::N2),
                (DirKey::W, Self::N4),
                (DirKey::N, Self::N8),
            ],
            Self::N6 => vec![
                (DirKey::S, Self::N3),
                (DirKey::W, Self::N5),
                (DirKey::N, Self::N9),
            ],
            Self::N7 => vec![(DirKey::E, Self::N8), (DirKey::S, Self::N4)],
            Self::N8 => vec![
                (DirKey::E, Self::N9),
                (DirKey::S, Self::N5),
                (DirKey::W, Self::N7),
            ],
            Self::N9 => vec![(DirKey::S, Self::N6), (DirKey::W, Self::N8)],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirKey {
    A,
    N,
    E,
    S,
    W,
}

impl DirKey {
    // const fn from(c: char) -> Self {
    //     match c {
    //         'A' => Self::A,
    //         '^' => Self::N,
    //         '>' => Self::E,
    //         'v' => Self::S,
    //         '<' => Self::W,
    //         _ => panic!(),
    //     }
    // }

    // const fn step(self, dir: Self) -> Self {
    //     match (self, dir) {
    //         (_, Self::A) => self,
    //         (Self::A, Self::S) | (Self::S, Self::E) => Self::E,
    //         (Self::A, Self::W) | (Self::S, Self::N) => Self::N,
    //         (Self::N, Self::E) | (Self::E, Self::N) => Self::A,
    //         (Self::N, Self::S) | (Self::W, Self::E) | (Self::E, Self::W) => Self::S,
    //         (Self::S, Self::W) => Self::W,
    //         _ => panic!(),
    //     }
    // }

    fn get_nbrs(self) -> Vec<(Self, Self)> {
        match self {
            Self::A => vec![(Self::S, Self::E), (Self::W, Self::N)],
            Self::N => vec![(Self::E, Self::A), (Self::S, Self::S)],
            Self::E => vec![(Self::W, Self::S), (Self::N, Self::A)],
            Self::S => vec![(Self::E, Self::E), (Self::N, Self::N), (Self::W, Self::W)],
            Self::W => vec![(Self::E, Self::S)],
        }
    }

    //const fn to_char(self) -> char {
    //    match self {
    //        Self::A => 'A',
    //        Self::N => '^',
    //        Self::E => '>',
    //        Self::S => 'v',
    //        Self::W => '<',
    //    }
    //}
}

// fn unravel(dirs: &[DirKey]) -> Vec<DirKey> {
//     let mut output = Vec::new();
//     let mut cur = DirKey::A;
//     for &k in dirs {
//         if k == DirKey::A {
//             output.push(cur);
//         } else {
//             cur = cur.step(k);
//         }
//     }
//     output
// }

// Returns all shortest paths from src to tgt
fn numkey_shortest_paths(src: NumKey, tgt: NumKey) -> Vec<Vec<DirKey>> {
    let threshold = shortest_path_length(
        (None, src),
        |(_, x)| x == tgt,
        |(_, x)| x.get_nbrs().iter().map(|(d, k)| (Some(*d), *k)).collect(),
    )
    .unwrap();

    let mut output = Vec::new();
    // If we start the search and the source is at the target, then there
    // is one path of length 0.
    if src == tgt {
        output.push(Vec::from([DirKey::A]));
    } else {
        numkey_paths_threshold_recursive(src, tgt, threshold, 0, &mut output);
    }
    output
}

// Returns all paths from src to tgt with length less than equal to a certain
// threshold. Stops searches when it reaches the target.
fn numkey_paths_threshold_recursive(
    src: NumKey,
    tgt: NumKey,
    threshold: usize,
    depth: usize,
    output: &mut Vec<Vec<DirKey>>,
) -> Vec<usize> {
    if depth < threshold {
        // This vector will store the indices in the output list of paths that
        // have been found which stem from the current source.
        let mut to_add = vec![];

        for (dir, nbr) in src.get_nbrs() {
            if nbr == tgt {
                let i = output.len();
                to_add.push(i);
                output.push(Vec::with_capacity(depth + 2));
                output[i].resize(depth + 2, DirKey::A);
                output[i][depth] = dir;
            } else {
                let new_path_indices =
                    numkey_paths_threshold_recursive(nbr, tgt, threshold, depth + 1, output);

                for &i in &new_path_indices {
                    output[i][depth] = dir;
                }

                to_add.extend(new_path_indices);
            }
        }
        return to_add;
    }

    vec![]
}

// Returns all shortest paths from src to tgt
fn dirkey_shortest_paths(src: DirKey, tgt: DirKey) -> Vec<Vec<DirKey>> {
    let threshold = shortest_path_length(
        (None, src),
        |(_, x)| x == tgt,
        |(_, x)| x.get_nbrs().iter().map(|(d, k)| (Some(*d), *k)).collect(),
    )
    .unwrap();

    let mut output = Vec::new();
    // If we start the search and the source is at the target, then there
    // is one path of length 0.
    if src == tgt {
        output.push(Vec::from([DirKey::A]));
    } else {
        dirkey_paths_threshold_recursive(src, tgt, threshold, 0, &mut output);
    }
    output
}

// Returns all paths from src to tgt with length less than equal to a certain
// threshold. Stops searches when it reaches the target.
fn dirkey_paths_threshold_recursive(
    src: DirKey,
    tgt: DirKey,
    threshold: usize,
    depth: usize,
    output: &mut Vec<Vec<DirKey>>,
) -> Vec<usize> {
    if depth < threshold {
        // This vector will store the indices in the output list of paths that
        // have been found which stem from the current source.
        let mut to_add = vec![];

        for (dir, nbr) in src.get_nbrs() {
            if nbr == tgt {
                let i = output.len();
                to_add.push(i);
                output.push(Vec::with_capacity(depth + 2));
                output[i].resize(depth + 2, DirKey::A);
                output[i][depth] = dir;
            } else {
                let new_path_indices =
                    dirkey_paths_threshold_recursive(nbr, tgt, threshold, depth + 1, output);

                for &i in &new_path_indices {
                    output[i][depth] = dir;
                }

                to_add.extend(new_path_indices);
            }
        }
        return to_add;
    }

    vec![]
}

fn expand_dirkey_path(path: &[DirKey]) -> Vec<Vec<DirKey>> {
    let mut output: Vec<Vec<DirKey>> = vec![vec![]];

    let mut cur = DirKey::A;
    for &next in path {
        let next_parts = dirkey_shortest_paths(cur, next);

        if next_parts.len() == 1 {
            for output_path in &mut output {
                output_path.extend(next_parts[0].clone());
            }
        } else {
            let n = output.len();
            let cloned = output.clone();
            output.extend(cloned);
            for j in 0..n {
                output[j].extend(next_parts[0].clone());
                output[n + j].extend(next_parts[1].clone());
            }
        }
        cur = next;
    }

    output
}

// Given a source NumKey, a tgt NumKey, and a number of keypads, returns a
// shortest input sequence that, when typed on the first keypad, results in
// moving from the source to the target on the final numpad and pressing the
// target.
fn shortest_input_sequence(src: NumKey, tgt: NumKey, keypads: usize) -> Vec<DirKey> {
    let mut options = numkey_shortest_paths(src, tgt);

    for _ in 1..keypads {
        let mut new_options = vec![];
        for path in options {
            new_options.extend(expand_dirkey_path(&path));
        }

        let min = new_options.iter().map(Vec::len).min().unwrap();
        options = new_options.into_iter().filter(|p| p.len() == min).collect();
    }

    options.pop().unwrap()
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let codes: Vec<Vec<_>> = lines
        .map(Result::unwrap)
        .map(|s| s.chars().map(NumKey::from).collect())
        .collect();

    let mut output = 0;

    for code in codes {
        let code_num = code[..code.len() - 1]
            .iter()
            .map(|&k| k.to_char())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let mut to_add = 0;

        let mut cur = NumKey::A;
        for key in code {
            to_add += shortest_input_sequence(cur, key, 3).len();
            cur = key;
        }

        output += to_add * code_num;
    }

    format!("{output}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}
