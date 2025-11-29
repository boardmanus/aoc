pub struct Pairs<I: Iterator + Clone> {
    first: Option<I::Item>,
    first_iter: I,
    second_iter: I,
}

impl<I> Pairs<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    pub fn new(iter: I) -> Self {
        let mut iter = iter;
        Pairs {
            first: iter.next(),
            first_iter: iter.clone(),
            second_iter: iter,
        }
    }

    fn update_iters(&mut self) -> Option<(I::Item, I::Item)> {
        self.first = self.first_iter.next();
        if let Some(first) = self.first.clone() {
            self.second_iter = self.first_iter.clone();
            self.second_iter.next().map(|second| (first, second))
        } else {
            None
        }
    }
}

impl<I> Iterator for Pairs<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(first) = self.first.clone() {
            if let Some(second) = self.second_iter.next() {
                Some((first, second))
            } else {
                self.update_iters()
            }
        } else {
            None
        }
    }
}
pub trait PairsIterator: Iterator + Clone {
    fn pairs(self) -> Pairs<Self>
    where
        Self::Item: Clone,
        Self: Sized,
    {
        Pairs::new(self)
    }
}

impl<I: Iterator + Clone> PairsIterator for I {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pairs() {
        assert_eq!(0, [1].iter().pairs().collect::<Vec<_>>().len());
        assert_eq!(vec![(&1, &2)], [1, 2].iter().pairs().collect::<Vec<_>>());
        assert_eq!(
            vec![(&1, &2), (&1, &3), (&2, &3)],
            [1, 2, 3].iter().pairs().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(&1, &2), (&1, &3), (&1, &4), (&2, &3), (&2, &4), (&3, &4)],
            [1, 2, 3, 4].iter().pairs().collect::<Vec<_>>()
        );
    }
}
