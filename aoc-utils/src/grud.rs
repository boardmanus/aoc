pub mod grif;

use enum_iterator::Sequence;

use crate::{
    dir::{Dir, DirVec},
    pos2d::Pos2d,
};
use std::{fmt::Display, marker::PhantomData, slice::Iter};

#[derive(Debug)]
pub enum Error {
    BadSize,
}

pub type GridPos = Pos2d<i64>;
pub type GridVec = DirVec;
pub type Walkable<Item, D> = fn(g: &Grid<Item, D>, a: &GridPos, b: &GridPos) -> bool;

pub struct GridRowIter<'a, Item, D>
where
    Item: Copy + Eq,
{
    iter: std::slice::Iter<'a, Item>,
    phantom: PhantomData<D>,
}

impl<'a, Item, D> GridRowIter<'a, Item, D>
where
    Item: Copy + Eq,
    D: Dir,
{
    fn new(grid: &'a Grid<Item, D>) -> GridRowIter<'a, Item, D> {
        GridRowIter::<Item, D> {
            iter: grid.g.iter(),
            phantom: PhantomData,
        }
    }
}

impl<'a, Item, D> Iterator for GridRowIter<'a, Item, D>
where
    Item: Copy + Eq,
{
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, Item, D> DoubleEndedIterator for GridRowIter<'a, Item, D>
where
    Item: Copy + Eq,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

// An iterator that traverses a grid by row or column
pub struct GridColIter<'a, Item, D>
where
    Item: Copy + Eq,
    D: Dir,
{
    grid: &'a Grid<Item, D>,
    i: Option<usize>,
    phantom: PhantomData<D>,
}

impl<'a, Item, D> GridColIter<'a, Item, D>
where
    Item: Copy + Eq,
    D: Dir,
{
    fn new(grid: &'a Grid<Item, D>) -> GridColIter<'a, Item, D> {
        GridColIter::<Item, D> {
            grid,
            i: if grid.g.is_empty() { None } else { Some(0) },
            phantom: PhantomData,
        }
    }

    fn wrap_col(&self, i: usize) -> Option<usize> {
        if i < self.grid.g.len() {
            Some(i)
        } else {
            let wrap_i = i % self.grid.width + 1;
            if wrap_i < self.grid.width {
                Some(wrap_i)
            } else {
                None
            }
        }
    }

    fn next_i(&mut self) -> Option<usize> {
        let i = self.i?;
        self.i = self.wrap_col(i + self.grid.width);
        Some(i)
    }
}

impl<'a, Item, D> Iterator for GridColIter<'a, Item, D>
where
    Item: Copy + Eq,
    D: Dir,
{
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.next_i()?;
        self.grid.g.get(i)
    }
}

impl<'a, Item, D> DoubleEndedIterator for GridColIter<'a, Item, D>
where
    Item: Copy + Eq,
    D: Dir,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let i = self.next_i()?;
        self.grid.g.get(self.grid.g.len() - i - 1)
    }
}

#[derive(Debug, Clone)]
pub struct Grid<Item, D>
where
    Item: Copy + Eq,
    D: Dir,
{
    width: usize,
    height: usize,
    g: Vec<Item>,
    walkable: Walkable<Item, D>,
    phantom: PhantomData<D>,
}

impl<Item, D> Display for Grid<Item, D>
where
    Item: Copy + Eq + Display,
    D: Dir + Sequence,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for col in 0..self.height() as i64 {
            for row in 0..self.width() as i64 {
                write!(f, "{}", self.g[self.i_from(&GridPos::new(row, col))])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<Item, D> Grid<Item, D>
where
    Item: Copy + Eq,
    D: Dir + Sequence,
{
    pub fn create_walkable(
        width: usize,
        height: usize,
        g: Vec<Item>,
        walkable: Walkable<Item, D>,
    ) -> Result<Grid<Item, D>, Error> {
        if g.len() == width * height {
            Ok(Grid {
                width,
                height,
                g,
                walkable,
                phantom: PhantomData,
            })
        } else {
            Err(Error::BadSize)
        }
    }
    pub fn create(width: usize, height: usize, g: Vec<Item>) -> Result<Grid<Item, D>, Error> {
        Grid::create_walkable(width, height, g, |_, _, _| true)
    }
    pub fn new_walkable(
        item: Item,
        width: usize,
        height: usize,
        walkable: Walkable<Item, D>,
    ) -> Grid<Item, D> {
        let g = vec![item; width * height];
        Grid {
            width,
            height,
            g,
            walkable,
            phantom: PhantomData,
        }
    }

    pub fn new(item: Item, width: usize, height: usize) -> Grid<Item, D> {
        Grid::new_walkable(item, width, height, |_, _, _| true)
    }

    pub fn parse(input: &str) -> Grid<char, D> {
        Grid::parse_items(input, |c| c, |_, _, _| true)
    }

    pub fn parse_walkable(input: &str, walkable: Walkable<char, D>) -> Grid<char, D> {
        Grid::parse_items(input, |c| c, walkable)
    }

    pub fn parse_items(
        input: &str,
        convert: fn(char) -> Item,
        walkable: Walkable<Item, D>,
    ) -> Grid<Item, D> {
        let rows_cols: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
        let width = rows_cols[0].len();
        let height = rows_cols.len();
        let g = rows_cols
            .iter()
            .flatten()
            .map(|x| convert(*x))
            .collect::<Vec<_>>();
        Grid {
            width,
            height,
            g,
            walkable,
            phantom: PhantomData,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn fill(&mut self, item: Item) {
        self.g.fill(item);
    }

    pub fn iter(&self) -> Iter<'_, Item> {
        self.g.iter()
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = GridPos> + '_ {
        (0..self.g.len()).map(|i| self.pos_from(i))
    }

    pub fn iter_pair(&self) -> impl Iterator<Item = (GridPos, Item)> + '_ {
        (0..self.g.len()).map(|i| (self.pos_from(i), self.g[i]))
    }

    pub fn row_iter(&self) -> GridRowIter<'_, Item, D> {
        GridRowIter::<Item, D>::new(self)
    }

    pub fn col_iter(&self) -> GridColIter<'_, Item, D> {
        GridColIter::<Item, D>::new(self)
    }

    pub fn is_valid(&self, pos: &GridPos) -> bool {
        pos.x >= 0 && pos.x < self.width as i64 && pos.y >= 0 && pos.y < self.height as i64
    }

    pub fn value(&self, pos: &GridPos) -> Option<&Item> {
        if self.is_valid(pos) {
            Some(&self.g[self.i_from(pos)])
        } else {
            None
        }
    }

    pub fn at(&self, pos: &GridPos) -> Option<Item> {
        if self.is_valid(pos) {
            Some(self.g[self.i_from(pos)])
        } else {
            None
        }
    }

    pub fn pos_from(&self, i: usize) -> GridPos {
        GridPos::new((i % self.width) as i64, (i / self.width) as i64)
    }

    fn i_from(&self, pos: &GridPos) -> usize {
        (pos.x as usize) + (pos.y as usize) * self.width
    }

    pub fn set(&mut self, pos: &GridPos, val: Item) -> Option<Item> {
        let i = self.i_from(pos);
        if i < self.g.len() {
            let old = self.g[i];
            self.g[i] = val;
            Some(old)
        } else {
            None
        }
    }

    pub fn find(&self, c: Item) -> Option<GridPos> {
        self.g.iter().enumerate().find_map(|(i, &val)| {
            if val == c {
                Some(self.pos_from(i))
            } else {
                None
            }
        })
    }

    pub fn is_walkable(&self, a: &GridPos, b: &GridPos) -> bool {
        (self.walkable)(self, a, b)
    }

    pub fn around(&self, pos: GridPos) -> impl Iterator<Item = GridPos> {
        D::cw().map(move |d| pos + d.to_vec2d())
    }

    pub fn neighbours(&self, pos: GridPos) -> impl Iterator<Item = GridPos> + '_ {
        self.around(pos)
            .filter(move |n| self.is_valid(n) && self.is_walkable(&pos, n))
    }

    pub fn is_neighbour(&self, a: GridPos, b: GridPos) -> bool {
        self.neighbours(a).any(|n| n == b)
    }

    pub fn matches(&self, pos: &GridPos, c: Item) -> bool {
        self.at(pos) == Some(c)
    }

    pub fn filter_items(&self, c: Item) -> impl Iterator<Item = GridPos> + '_ {
        self.row_iter()
            .enumerate()
            .filter(move |(_, &x)| x == c)
            .map(|(i, _)| self.pos_from(i))
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use crate::dir::{Dir4, Dir8};

    use super::*;

    #[test]
    fn test_grid_parse() {
        let g = Grid::<char, Dir4>::parse("1234\n1234\n5678\n");
        assert_eq!(
            g.g,
            vec!['1', '2', '3', '4', '1', '2', '3', '4', '5', '6', '7', '8']
        );
    }

    #[test]
    fn test_grid_at() {
        let g = Grid::<char, Dir4>::parse("1234\n1234\n5678\n");
        assert_eq!(g.at(&GridPos::new(2, 1)), Some('3'));
    }

    #[test]
    fn test_grid_row_iter() {
        let g = Grid::<char, Dir4>::parse("1234\n1234\n5678\n");
        assert_eq!(g.row_iter().collect::<String>(), "123412345678");
    }

    #[test]
    fn test_grid_col_iter() {
        let g = Grid::<char, Dir4>::parse("1234\n1234\n5678\n");
        assert_eq!(g.col_iter().collect::<String>(), "115226337448");
    }

    #[test]
    fn test_grid_row_iter_rev() {
        let g = Grid::<char, Dir4>::parse("1234\n1234\n5678\n");
        assert_eq!(g.row_iter().rev().collect::<String>(), "876543214321");
    }

    #[test]
    fn test_grid_col_iter_rev() {
        let g = Grid::<char, Dir4>::parse("1234\n1234\n5678\n");
        assert_eq!(g.col_iter().rev().collect::<String>(), "844733622511");
    }

    #[test]
    fn test_grid_filter_items() {
        let g = Grid::<char, Dir4>::parse("1234\n1234\n5678\n");
        assert_eq!(
            g.filter_items('2').collect::<Vec<_>>(),
            vec![GridPos::new(1, 0), GridPos::new(1, 1)]
        );
    }

    #[test]
    fn test_grid_neigbours() {
        let g =
            Grid::<char, Dir8>::parse_walkable("1234\n1234\n5373\n", |g, a, b| g.at(a) == g.at(b));

        assert_eq!(
            g.neighbours(GridPos::new(0, 0)).collect::<Vec<_>>(),
            vec![GridPos::new(0, 1)]
        );

        assert_eq!(
            g.neighbours(GridPos::new(2, 1)).collect::<HashSet<_>>(),
            HashSet::from([GridPos::new(2, 0), GridPos::new(1, 2), GridPos::new(3, 2)])
        );

        assert!(g.is_walkable(&GridPos::new(2, 1), &GridPos::new(2, 0)));
        assert!(!g.is_walkable(&GridPos::new(2, 1), &GridPos::new(1, 1)));
        assert!(g.is_neighbour(GridPos::new(2, 1), GridPos::new(2, 0)));
        assert!(!g.is_neighbour(GridPos::new(2, 1), GridPos::new(1, 1)));
    }
}
