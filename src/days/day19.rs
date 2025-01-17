use crate::{graph::exists_path, utils::LinesIterator};

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

pub fn run2(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}
