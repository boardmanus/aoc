use enum_iterator::Sequence;

use super::{Grid, GridPos};
use crate::{dir::Dir, grif::Graph};

impl<Item, D> Graph for Grid<Item, D>
where
    Item: Copy + Eq,
    D: Dir + Sequence,
{
    type NodeId = GridPos;
    type Weight = usize;
    type NodeValue = Item;

    fn node(&self, id: &Self::NodeId) -> Option<&Self::NodeValue> {
        self.value(id)
    }

    fn nodes(&self) -> impl Iterator<Item = Self::NodeId> {
        self.iter_pos()
    }

    fn name(&self) -> String {
        "grud".to_string()
    }

    fn node_edges(&self, node: Self::NodeId) -> impl Iterator<Item = (Self::NodeId, Self::Weight)> {
        self.neighbours(node).map(|n| (n, 1))
    }
}
