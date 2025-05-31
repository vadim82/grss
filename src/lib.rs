use std::io::{BufRead, BufReader, Read};
pub fn find_matches<R: Read>(reader: R, pattern: &str) -> impl Iterator<Item = String> {
    let buf_reader = BufReader::new(reader);
    buf_reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(move |l| l.contains(pattern))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn find_a_match() {
        let input = "lorem ipsum\ndolor sit amet";
        let result: Vec<_> = find_matches(Cursor::new(input), "lorem").collect();
        assert_eq!(result, vec!["lorem ipsum"]);
    }
}