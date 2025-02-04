pub mod algorithms;
pub mod simple;
use std::hash::Hash;

pub trait Node<'a> {
    type Id: Copy + Eq + 'a;
    type Weight: Copy + 'a;
    type Edge: Edge<Id = Self::Id, Weight = Self::Weight> + 'a;

    fn id(&self) -> Self::Id;
    fn edges(&'a self) -> impl Iterator<Item = &'a Self::Edge>;

    fn is_adjacent(&'a self, node_id: Self::Id) -> bool {
        self.edges().any(|e| e.b() == node_id)
    }

    fn degree(&'a self) -> usize {
        self.edges().count()
    }

    fn neighbours(&'a self) -> impl Iterator<Item = <Self::Edge as Edge>::Id> + 'a {
        self.edges().map(move |e| e.b())
    }
}

pub trait Edge {
    type Id: Copy + Eq;
    type Weight: Copy;

    fn weight(&self) -> Self::Weight;
    fn a(&self) -> Self::Id;
    fn b(&self) -> Self::Id;
}

pub trait Graph<'a> {
    type Id: Hash + Eq + Copy + 'a;
    type Weight: 'a;
    type Node: Node<'a, Id = Self::Id, Weight = Self::Weight> + 'a;

    fn node(&self, id: &Self::Id) -> Option<&Self::Node>;
    fn nodes(&'a self) -> impl Iterator<Item = &Self::Node>;
    fn edges(&'a self) -> impl Iterator<Item = &<Self::Node as Node<'a>>::Edge> {
        self.nodes().flat_map(move |n| n.edges())
    }
}

pub trait Builder<'a> {
    type Id: Hash + Eq + Copy + 'a;
    type Weight: Copy + 'a;
    type Graph: Graph<'a, Id = Self::Id, Weight = Self::Weight>;

    fn add_node(&mut self, id: Self::Id) -> &mut <Self::Graph as Graph<'a>>::Node;
    fn add_node_edge(&mut self, a: Self::Id, b: Self::Id, weight: Self::Weight);
    fn build(self) -> Self::Graph;

    fn add_directed_edge(&mut self, a: Self::Id, b: Self::Id, weight: Self::Weight) {
        self.add_node_edge(a.clone(), b.clone(), weight.clone());
        self.add_node(b);
    }

    fn add_edge(&mut self, a: Self::Id, b: Self::Id, weight: Self::Weight) {
        self.add_node_edge(a.clone(), b.clone(), weight.clone());
        self.add_node_edge(b, a, weight);
    }
}
