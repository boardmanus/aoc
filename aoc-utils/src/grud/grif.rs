/*
use super::{Grid, GridPos};
use crate::{dir::Dir, grif::Edge};

struct GridEdge {
    a: GridPos,
    b: GridPos,
}

impl Edge for GridEdge {
    type Id = GridPos;
    type Weight = bool;

    fn weight(&self) -> bool {
        true
    }

    fn a(&self) -> GridPos {
        self.a
    }

    fn b(&self) -> GridPos {
        self.b
    }
}

struct GridNode<'a, Item, D>
where
    Item: Copy + Eq,
    D: Dir,
{
    grid: &'a Grid<Item, D>,
    id: GridPos,
    item: Item,
}



impl<'a, Item, D> Node<'a> for GridNode<'a, Item, D>
where
    Item: Eq + Copy,
    D: Dir + Sequence,
{
    type Id = GridPos;
    type Weight = bool;
    type Edge = GridEdge<'a, Item, D>;

    fn id(&self) -> GridPos {
        self.id
    }

    fn edges(&'a self) -> impl Iterator<Item = &Self::Edge> {
        self.grid.neighbours(&self.id).map(|n| Self::Edge {
            grid: self.grid,
            a: self.id,
            b: n,
        })
    }
}

impl<'a, Item, D> Graph<'a> for Grid<Item, D>
where
    Item: Copy + Eq + 'a,
    D: Dir + Sequence + 'a,
{
    type Id = GridPos;
    type Weight = bool;
    type Node = GridNode<'a, Item, D>;

    fn node(&'a self, id: &Self::Id) -> Option<Box<Self::Node>> {
        self.at(id).map(|item| {
            Box::new(Self::Node {
                grid: self,
                id: *id,
                item,
            })
        })
    }

    fn nodes(&self) -> impl Iterator<Item = &Self::Node> {
        self.row_iter().map(|&item| Self::Node {
            grid: self,
            id,
            item,
        })
    }
}

*/
