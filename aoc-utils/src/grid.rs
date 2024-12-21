use crate::{dir::{Dir, Dir4, Dir8, DirVec}, pos2d::Pos2d};
use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, Sub},
    slice::Iter,
};

type GridPos = Pos2d<i64>;
type GridVec = DirVec;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Index(pub i64, pub i64);

impl Index {
    pub fn dir8(dir: Dir8) -> Index {
        match dir {
            Dir8::N => Index(0, -1),
            Dir8::NE => Index(1, -1),
            Dir8::E => Index(1, 0),
            Dir8::SE => Index(1, 1),
            Dir8::S => Index(0, 1),
            Dir8::SW => Index(-1, 1),
            Dir8::W => Index(-1, 0),
            Dir8::NW => Index(-1, -1),
        }
    }

    pub fn dir4(dir: Dir4) -> Index {
        match dir {
            Dir4::N => Index(0, -1),
            Dir4::E => Index(1, 0),
            Dir4::S => Index(0, 1),
            Dir4::W => Index(-1, 0),
        }
    }
}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.1 == other.1 {
            Some(self.0.cmp(&other.0))
        } else {
            Some(self.1.cmp(&other.1))
        }
    }
}

impl Add for Index {
    type Output = Index;

    fn add(self, rhs: Index) -> Self::Output {
        Index(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Dir8> for Index {
    type Output = Index;

    fn add(self, rhs: Dir8) -> Self::Output {
        self + Index::dir8(rhs)
    }
}

impl Add<Dir4> for Index {
    type Output = Index;

    fn add(self, rhs: Dir4) -> Self::Output {
        self + Index::dir4(rhs)
    }
}

impl Add<Dir8> for GridPos {
    type Output = GridPos;

    fn add(self, rhs: Dir8) -> Self::Output {
        self + rhs
    }
}

impl Add<Dir4> for GridPos {
    type Output = Index;

    fn add(self, rhs: Dir4) -> Self::Output {
        self + rhs
    }
}

impl Sub for Index {
    type Output = Index;

    fn sub(self, rhs: Self) -> Self::Output {
        Index(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub<Dir8> for Index {
    type Output = Index;

    fn sub(self, rhs: Dir8) -> Self::Output {
        self - Index::dir8(rhs)
    }
}

impl Sub<Dir4> for Index {
    type Output = Index;

    fn sub(self, rhs: Dir4) -> Self::Output {
        self - Index::dir4(rhs)
    }
}

pub struct GridIndexIter<'a, Item: Copy + Eq> {
    grid_iter: GridIter<'a, Item>,
}

impl<'a, Item: Copy + Eq> GridIndexIter<'a, Item> {
    fn new(grid: &'a Grid<Item>, iter_type: IterType) -> GridIndexIter<'a, Item> {
        let grid_iter = GridIter::<Item> {
            grid,
            iter_type,
            i: if grid.g.is_empty() { None } else { Some(0) },
        };
        GridIndexIter { grid_iter }
    }
}
impl<'a, GridItem: Copy + Eq> Iterator for GridIndexIter<'a, GridItem> {
    type Item = (Index, &'a GridItem);
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.grid_iter.i?;
        let gi = self.grid_iter.next()?;
        Some((self.grid_iter.grid.index_from(i), gi))
    }
}

pub enum IterType {
    Row,
    Col,
}

pub struct GridIter<'a, Item: Copy + Eq> {
    grid: &'a Grid<Item>,
    iter_type: IterType,
    i: Option<usize>,
}

impl<'a, Item: Copy + Eq> GridIter<'a, Item> {
    fn new(grid: &'a Grid<Item>, iter_type: IterType) -> GridIter<'a, Item> {
        GridIter::<Item> {
            grid,
            iter_type,
            i: if grid.g.is_empty() { None } else { Some(0) },
        }
    }

    fn next_i(&self) -> Option<usize> {
        let i = self.i?;
        let len = self.grid.g.len();
        match self.iter_type {
            IterType::Row => {
                if i + 1 < len {
                    Some(i + 1)
                } else {
                    None
                }
            }
            IterType::Col => {
                let new_i = i + self.grid.width;
                if new_i < len {
                    Some(new_i)
                } else {
                    let wrap_i = new_i % self.grid.width + 1;
                    if wrap_i < self.grid.width {
                        Some(wrap_i)
                    } else {
                        None
                    }
                }
            }
        }
    }
}

impl<'a, GridItem: Copy + Eq> Iterator for GridIter<'a, GridItem> {
    type Item = &'a GridItem;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i?;
        self.i = self.next_i();
        self.grid.g.get(i)
    }
}

impl<'a, GridItem: Copy + Eq> DoubleEndedIterator for GridIter<'a, GridItem> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let i = self.i?;
        assert!(i < self.grid.g.len());
        self.i = self.next_i();
        self.grid.g.get(self.grid.g.len() - i - 1)
    }
}

#[derive(Debug, Clone)]
pub struct Grid<Item: Copy + Eq> {
    width: usize,
    height: usize,
    g: Vec<Item>,
}

impl<Item: Copy + Eq + Display> Display for Grid<Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for col in 0..self.height() as i64 {
            for row in 0..self.width() as i64 {
                write!(f, "{}", self.g[self.i_from(Index(row, col))])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<Item: Copy + Eq> Grid<Item> {
    pub fn create(width: usize, height: usize, g: Vec<Item>) -> Option<Grid<Item>> {
        if g.len() == width * height {
            Some(Grid { width, height, g })
        } else {
            None
        }
    }

    pub fn new(item: Item, width: usize, height: usize) -> Grid<Item> {
        let g = vec![item; width * height];
        Grid { width, height, g }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn parse(input: &str) -> Grid<char> {
        Grid::parse_items(input, |c| c)
    }

    pub fn iter(&self) -> Iter<'_, Item> {
        self.g.iter()
    }

    pub fn row_iter(&self) -> GridIter<'_, Item> {
        GridIter::<Item>::new(self, IterType::Row)
    }

    pub fn row_index_iter(&self) -> GridIndexIter<'_, Item> {
        GridIndexIter::<Item>::new(self, IterType::Row)
    }

    pub fn col_iter(&self) -> GridIter<'_, Item> {
        GridIter::<Item>::new(self, IterType::Col)
    }

    pub fn col_index_iter(&self) -> GridIndexIter<'_, Item> {
        GridIndexIter::<Item>::new(self, IterType::Col)
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
    pub fn is_valid_pos(&self, index: &GridPos) -> bool {
        index.x >= 0 && index.x < self.width as i64 && index.y >= 0 && index.y < self.height as i64
    }

    pub fn at(&self, index: Index) -> Option<Item> {
        if self.is_valid(index) {
            Some(self.g[self.i_from(index)])
        } else {
            None
        }
    }

    pub fn at_pos(&self, index: &GridPos) -> Option<Item> {
        if self.is_valid_pos(index) {
            Some(self.g[self.i_from_pos(&index)])
        } else {
            None
        }
    }

    pub fn index_from(&self, i: usize) -> Index {
        Index((i % self.width) as i64, (i / self.width) as i64)
    }
    pub fn pos_from(&self, i: usize) -> GridPos {
        GridPos::new((i % self.width) as i64, (i / self.width) as i64)
    }

    fn i_from(&self, index: Index) -> usize {
        (index.0 as usize) + (index.1 as usize) * self.width
    }

    fn i_from_pos(&self, index: &GridPos) -> usize {
        (index.x as usize) + (index.y as usize) * self.width
    }

    pub fn set(&mut self, index: Index, val: Item) {
        let i = self.i_from(index);
        self.g[i] = val;
    }
    pub fn set_pos(&mut self, index: &GridPos, val: Item) {
        let i = self.i_from_pos(index);
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
    pub fn find_pos(&self, c: Item) -> Option<GridPos> {
        self.g.iter().enumerate().find_map(|(i, &val)| {
            if val == c {
                Some(self.pos_from(i))
            } else {
                None
            }
        })
    }

    pub fn matches(&self, index: Index, c: Item) -> bool {
        self.at(index) == Some(c)
    }

    pub fn matches_pos(&self, index: &GridPos, c: Item) -> bool {
        self.at_pos(index) == Some(c)
    }

    pub fn around(&self, index: Index) -> Vec<Index> {
        Dir8::cw().map(|d| index + d).collect()
    }

    pub fn around_pos(&self, index: &GridPos) -> Vec<GridPos> {
        Dir8::cw().map(|d| *index + d).collect()
    }

    pub fn filter_pos(&self, c: Item) -> Vec<Index> {
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
    pub fn filter_items(&self, c: Item) -> Vec<GridPos> {
        self.g
            .iter()
            .enumerate()
            .filter(|&(_i, &c2)| c == c2)
            .map(|(i, _c)| {
                GridPos::new(
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

    #[test]
    fn test_grid_row_iter() {
        let g = Grid::<char>::parse("1234\n1234\n5678\n");
        assert_eq!(g.row_iter().collect::<String>(), "123412345678");
    }

    #[test]
    fn test_grid_col_iter() {
        let g = Grid::<char>::parse("1234\n1234\n5678\n");
        assert_eq!(g.col_iter().collect::<String>(), "115226337448");
    }

    #[test]
    fn test_grid_row_iter_rev() {
        let g = Grid::<char>::parse("1234\n1234\n5678\n");
        assert_eq!(g.row_iter().rev().collect::<String>(), "876543214321");
    }

    #[test]
    fn test_grid_col_iter_rev() {
        let g = Grid::<char>::parse("1234\n1234\n5678\n");
        assert_eq!(g.col_iter().rev().collect::<String>(), "844733622511");
    }
}
