use crate::{graph::exists_path, utils::LinesIterator};
use std::collections::HashMap;

fn parse_input(lines: &mut LinesIterator) -> (Vec<String>, Vec<String>) {
    let available = lines
        .next()
        .unwrap()
        .unwrap()
        .split(", ")
        .map(std::string::ToString::to_string)
        .collect();

    lines.next();

    let designs = lines.map(Result::unwrap).collect();

    (available, designs)
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let (available, designs) = parse_input(lines);

    let mut possible = 0;

    for design in designs {
        let get_children = |cur: String| {
            let mut output = Vec::new();

            for a in &available {
                let cur = cur.clone();
                let new = cur + a;

                if design.starts_with(&new) {
                    output.push(new);
                }
            }

            output
        };

        let is_tgt = |cur: String| cur == design;

        if exists_path(String::new(), is_tgt, get_children) {
            possible += 1;
        }
    }

    format!("{possible}")
}

fn count_ways(design: &String, available: &[String], map: &mut HashMap<String, usize>) -> usize {
    if map.contains_key(design) {
        map[design]
    } else if design.is_empty() {
        1
    } else {
        let mut combinations = 0;
        for a in available {
            if design.starts_with(a) {
                combinations += count_ways(&design[a.len()..].to_string(), available, map);
            }
        }
        map.insert(design.to_string(), combinations);
        combinations
    }
}

pub fn run2(lines: &mut LinesIterator) -> String {
    let (available, designs) = parse_input(lines);

    let mut result = 0;
    let mut map = HashMap::new();

    for design in designs {
        result += count_ways(&design, &available, &mut map);
    }

    format!("{result}")
}
