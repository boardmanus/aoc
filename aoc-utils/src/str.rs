pub trait AocStr {
    fn first(&self) -> Option<char>;
    fn nth(&self, n: usize) -> char;
}

impl AocStr for str {
    fn first(&self) -> Option<char> {
        self.chars().next()
    }

    fn nth(&self, n: usize) -> char {
        self.chars().nth(n).unwrap()
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
