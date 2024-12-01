pub fn sorted<T: Clone + Ord>(a: &[T]) -> Vec<T> {
    let mut v = Vec::<T>::from(a);
    v.sort();
    v
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_sorted() {
        let v = sorted(&[1, 5, 2, 7, 3]);
        assert_eq!(v, vec![1, 2, 3, 5, 7]);
    }
}
