use std::{fmt::Display, rc::Rc};

type First<Data> = Rc<Node<Data>>;
type Rest<Data> = Option<First<Data>>;

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash, Eq)]
pub struct Node<Data> {
    pub data: Data,
    pub rest: Rest<Data>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub struct Lust<Data> {
    rest: Rest<Data>,
}

impl<Data: Clone> Lust<Data> {
    pub fn new(data: Data) -> Self {
        Lust {
            rest: Some(Rc::new(Node { data, rest: None })),
        }
    }

    pub fn data(&self) -> Option<&Data> {
        Some(&self.rest.as_ref()?.data)
    }

    pub fn rest(&self) -> Rest<Data> {
        self.rest.as_ref()?.rest.clone()
    }

    pub fn append(&self, data: Data) -> Self {
        Lust {
            rest: Some(Rc::new(Node {
                data,
                rest: self.rest.clone(),
            })),
        }
    }

    pub fn contains(&self, data: &Data) -> bool
    where
        Data: PartialEq,
    {
        self.iter().any(|d| &d == data)
    }

    pub fn iter(&self) -> LustIt<Data> {
        LustIt {
            rest: self.rest.clone(),
        }
    }
}

impl<Data> Display for Lust<Data>
where
    Data: Display + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        write!(f, "[")?;
        for data in self.iter() {
            if !first {
                write!(f, "->")?;
            }
            write!(f, "{}", data)?;
            first = false;
        }
        write!(f, "]")
    }
}

impl<Data: Clone> IntoIterator for Lust<Data> {
    type Item = Data;
    type IntoIter = LustIt<Data>;

    fn into_iter(self) -> Self::IntoIter {
        LustIt { rest: self.rest }
    }
}

#[derive(Clone)]
pub struct LustIt<Data> {
    rest: Rest<Data>,
}

impl<Data: Clone> Iterator for LustIt<Data> {
    type Item = Data;

    fn next(&mut self) -> Option<Self::Item> {
        let rest = self.rest.clone()?;
        self.rest = rest.rest.clone();
        Some(rest.data.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lust() {
        type TestLust = Lust<isize>;

        let a = TestLust::new(1);
        let b = a.append(2);
        let c = b.append(3);
        let d: Lust<isize> = a.append(0x100);
        let e = d.append(0x200);

        assert_eq!(c.iter().sum::<isize>(), 6);
        assert_eq!(e.iter().sum::<isize>(), 0x301);
        assert_eq!(e.into_iter().sum::<isize>(), 0x301);
    }
}
