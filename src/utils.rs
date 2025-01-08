pub type LinesIterator = std::io::Lines<std::io::BufReader<std::fs::File>>;

/// Takes an iterator to a bunch of strings and a separator, returns an iterator over vectors of strings, where each vector is obtained by splitting each line in the iterator by the separator.
pub fn lines_to_grid<'a>(
    lines: &'a mut LinesIterator,
    separator: &'a str,
) -> impl Iterator<Item = Vec<String>> + 'a {
    lines.map(Result::unwrap).map(move |x| {
        x.split(separator)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect::<Vec<_>>()
    })
}

pub fn lines_to_grid_of_chars(lines: &mut LinesIterator) -> impl Iterator<Item = Vec<char>> + '_ {
    lines.map(Result::unwrap).map(|x| x.chars().collect())
}

/// # Panics
///
/// Panics if any char in the input is not '0', '1', ..., '9'.
pub fn lines_to_grid_of_usize(lines: &mut LinesIterator) -> impl Iterator<Item = Vec<usize>> + '_ {
    lines.map(Result::unwrap).map(|x| {
        x.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    })
}
