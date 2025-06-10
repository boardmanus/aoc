use std::str::FromStr;

pub trait AocStr {
    fn first(&self) -> Option<char>;
    fn nth(&self, n: usize) -> char;
    fn parse_lines<V>(&self, f: impl FnMut(&str) -> V) -> Vec<V>;
    fn parse_nums<N: FromStr>(&self) -> Vec<N>;
    fn parse_sep_nums<N: FromStr>(&self, sep: &str) -> Vec<N>;
}

impl AocStr for str {
    fn first(&self) -> Option<char> {
        self.chars().next()
    }

    fn nth(&self, n: usize) -> char {
        self.chars().nth(n).unwrap()
    }

    fn parse_lines<V>(&self, f: impl FnMut(&str) -> V) -> Vec<V> {
        self.lines().map(f).collect::<Vec<_>>()
    }

    fn parse_nums<N: FromStr>(&self) -> Vec<N> {
        self.split_whitespace()
            .filter_map(|s| s.parse::<N>().ok())
            .collect()
    }

    fn parse_sep_nums<N: FromStr>(&self, sep: &str) -> Vec<N> {
        self.split(sep)
            .filter_map(|s| s.parse::<N>().ok())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!("abc".first(), Some('a'));
        assert_eq!("".first(), None);
    }
}
