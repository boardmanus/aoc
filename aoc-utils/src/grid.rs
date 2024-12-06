use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Index(pub i64, pub i64);

const DIRS: [Index; 8] = [
    Index(-1, -1),
    Index(0, -1),
    Index(1, -1),
    Index(-1, 0),
    Index(1, 0),
    Index(-1, 1),
    Index(0, 1),
    Index(1, 1),
];

impl Add for Index {
    type Output = Index;

    fn add(self, rhs: Index) -> Self::Output {
        Index(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Index {
    type Output = Index;

    fn sub(self, rhs: Self) -> Self::Output {
        Index(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Clone)]
pub struct Grid<Item: Copy + Eq> {
    width: usize,
    height: usize,
    g: Vec<Item>,
}

impl<Item: Copy + Eq> Grid<Item> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn parse(input: &str) -> Grid<char> {
        Grid::parse_items(input, |c| c)
    }

    pub fn parse_items(input: &str, convert: fn(char) -> Item) -> Grid<Item> {
        let rows_cols: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
        let width = rows_cols[0].len();
        let height = rows_cols.len();
        let g = rows_cols
            .iter()
            .flatten()
            .map(|x| convert(*x))
            .collect::<Vec<_>>();
        Grid { width, height, g }
    }

    pub fn is_valid(&self, index: Index) -> bool {
        index.0 >= 0 && index.0 < self.width as i64 && index.1 >= 0 && index.1 < self.height as i64
    }

    pub fn at(&self, index: Index) -> Option<Item> {
        if self.is_valid(index) {
            Some(self.g[(index.0 as usize) + (index.1 as usize) * self.width])
        } else {
            None
        }
    }

    fn index_from(&self, i: usize) -> Index {
        Index((i % self.width) as i64, (i / self.width) as i64)
    }

    fn i_from(&self, index: Index) -> usize {
        (index.0 as usize) + (index.1 as usize) * self.width
    }

    pub fn set(&mut self, index: Index, val: Item) {
        let i = self.i_from(index);
        self.g[i] = val;
    }

    pub fn find(&self, c: Item) -> Option<Index> {
        self.g.iter().enumerate().find_map(|(i, &val)| {
            if val == c {
                Some(self.index_from(i))
            } else {
                None
            }
        })
    }

    pub fn at_match(&self, index: Index, c: Item) -> bool {
        self.at(index) == Some(c)
    }

    pub fn around(&self, index: Index) -> Vec<Index> {
        DIRS.iter().map(|&d| index + d).collect()
    }

    pub fn pos_with_item(&self, c: Item) -> Vec<Index> {
        self.g
            .iter()
            .enumerate()
            .filter(|&(_i, &c2)| c == c2)
            .map(|(i, _c)| {
                Index(
                    i.rem_euclid(self.width) as i64,
                    i.div_euclid(self.width) as i64,
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_grid_parse() {
        let g = Grid::<char>::parse("1234\n1234\n5678\n");
        assert_eq!(
            g.g,
            vec!['1', '2', '3', '4', '1', '2', '3', '4', '5', '6', '7', '8']
        );
    }

    #[test]
    fn test_grid_at() {
        let g = Grid::<char>::parse("1234\n1234\n5678\n");
        assert_eq!(g.at(Index(2, 1)), Some('3'));
    }
}
