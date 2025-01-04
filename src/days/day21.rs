use crate::utils::LinesIterator;

pub fn run1(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}

pub fn run2(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}
