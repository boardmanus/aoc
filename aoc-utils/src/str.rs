pub trait AocStr {
  fn first(&self) -> Option<char>;
}

impl AocStr for str {
  fn first(&self) -> Option<char> {
    self.chars().next()
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