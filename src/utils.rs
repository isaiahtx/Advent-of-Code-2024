pub type LinesIterator = std::io::Lines<std::io::BufReader<std::fs::File>>;

/// Takes an iterator to a bunch of strings and a separator, returns an iterator over vectors of strings, where each vector is obtained by splitting each line in the iterator by the separator.
#[allow(clippy::needless_lifetimes)]
#[allow(dead_code)]
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

#[allow(clippy::needless_lifetimes)]
pub fn lines_to_grid_of_chars<'a>(
    lines: &'a mut LinesIterator,
) -> impl Iterator<Item = Vec<char>> + 'a {
    lines.map(Result::unwrap).map(|x| x.chars().collect())
}
