use super::{Builder, Graph};
use graphviz_rust::dot_structures as dots;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Display;
use std::hash::Hash;
use std::marker::PhantomData;

type FnEdgeWeight<NodeId, Weight> = fn(from: &NodeId, to: &NodeId) -> Weight;

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum EdgeWeight<NodeId, Weight> {
    Static(Weight),
    Dynamic(FnEdgeWeight<NodeId, Weight>),
}

struct Node<NodeId> {
    id: NodeId,
    edges: BTreeSet<NodeId>,
}

pub struct SimpleGraph<NodeId>
where
    NodeId: Copy + Eq + Ord,
{
    name: String,
    nodes: BTreeMap<NodeId, Node<NodeId>>,
}

impl<NodeId> SimpleGraph<NodeId>
where
    NodeId: Copy + Eq + Ord + Display + Hash,
{
    pub fn to_viz(&self, digraph: bool) -> dots::Graph {
        super::to_viz::<Self>(self, digraph)
    }
}

impl<NodeId> Display for SimpleGraph<NodeId>
where
    NodeId: Copy + Eq + Ord + Display + Hash,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        super::fmt_graph(self, f)
    }
}

impl<NodeId> Graph for SimpleGraph<NodeId>
where
    NodeId: Copy + Eq + Ord + Hash,
{
    type NodeId = NodeId;
    type NodeValue = PhantomData<u8>;
    type Weight = u8;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn node(&self, id: &Self::NodeId) -> Option<&Self::NodeValue> {
        if self.nodes.contains_key(id) {
            Some(&PhantomData)
        } else {
            None
        }
    }

    fn nodes(&self) -> impl Iterator<Item = Self::NodeId> {
        self.nodes.values().map(|n| n.id)
    }

    fn node_edges(&self, node: Self::NodeId) -> impl Iterator<Item = (Self::NodeId, Self::Weight)> {
        self.nodes
            .get(&node)
            .into_iter()
            .flat_map(|node| node.edges.iter().map(|&to| (to, 1)))
    }
}

pub struct SimpleGraphBuilder<NodeId>
where
    NodeId: Copy + Eq + Ord,
{
    graph: SimpleGraph<NodeId>,
}

pub trait NodeIdFromStr<'a>: Sized {
    fn node_id_from_str(s: &'a str) -> Option<Self>;
}

impl<'a> NodeIdFromStr<'a> for u64 {
    fn node_id_from_str(s: &'a str) -> Option<Self> {
        s.parse().ok()
    }
}

impl<'a> NodeIdFromStr<'a> for i64 {
    fn node_id_from_str(s: &'a str) -> Option<Self> {
        s.parse().ok()
    }
}

impl<'a> NodeIdFromStr<'a> for &'a str {
    fn node_id_from_str(s: &'a str) -> Option<Self> {
        Some(s)
    }
}

impl<'a, NodeId: Copy + Ord> SimpleGraphBuilder<NodeId> {
    pub fn new(name: &str) -> SimpleGraphBuilder<NodeId> {
        SimpleGraphBuilder {
            graph: SimpleGraph {
                name: name.to_string(),
                nodes: BTreeMap::new(),
            },
        }
    }
}

impl<'a, NodeId> SimpleGraphBuilder<NodeId>
where
    NodeId: Copy + Ord + Display + Hash + NodeIdFromStr<'a>,
{
    fn parse_nodes(
        input: &'a str,
        separator: &'a str,
    ) -> impl Iterator<Item = (NodeId, NodeId)> + 'a {
        input.lines().filter_map(move |line| {
            let mut it = line.split(separator);
            Some((
                NodeId::node_id_from_str(it.next()?)?,
                NodeId::node_id_from_str(it.next()?)?,
            ))
        })
    }

    pub fn parse(name: &str, input: &'a str, separator: &'a str) -> Option<SimpleGraph<NodeId>> {
        let mut builder = SimpleGraphBuilder::new(name);
        Self::parse_nodes(input, separator).for_each(|(a, b)| {
            builder.add_edge(a, b, 1);
        });
        Some(builder.build())
    }

    pub fn parse_directed(
        name: &str,
        input: &'a str,
        separator: &'a str,
    ) -> Option<SimpleGraph<NodeId>> {
        let mut builder = SimpleGraphBuilder::new(name);
        Self::parse_nodes(input, separator).for_each(|(a, b)| {
            builder.add_directed_edge(a, b, 1);
        });
        Some(builder.build())
    }
}

impl<NodeId> Builder for SimpleGraphBuilder<NodeId>
where
    NodeId: Copy + Eq + Ord + Hash,
{
    type Graph = SimpleGraph<NodeId>;
    type NodeId = NodeId;
    type NodeValue = PhantomData<u8>;
    type Weight = u8;

    fn add_node(&mut self, id: Self::NodeId) -> &mut Self {
        self.graph.nodes.entry(id).or_insert(Node {
            id,
            edges: BTreeSet::new(),
        });
        self
    }

    fn add_node_edge(
        &mut self,
        a: Self::NodeId,
        b: Self::NodeId,
        _weight: Self::Weight,
    ) -> &mut Self {
        self.add_node(a);
        self.graph.nodes.entry(a).and_modify(|n| {
            n.edges.insert(b);
        });
        self
    }

    fn add_directed_edge(
        &mut self,
        a: Self::NodeId,
        b: Self::NodeId,
        _weight: Self::Weight,
    ) -> &mut Self {
        self.add_node(a);
        self.add_node(b);
        self.graph.nodes.entry(a).and_modify(|n| {
            n.edges.insert(b);
        });
        self
    }

    fn build(self) -> Self::Graph {
        self.graph
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_simple_graph_builder() {
        let mut builder = SimpleGraphBuilder::<u64>::new("test");
        builder.add_edge(1, 2, 1);
        assert_eq!(builder.graph.nodes.len(), 2);
        assert_eq!(builder.graph.nodes.get(&1).unwrap().edges.len(), 1);
        assert_eq!(builder.graph.nodes.get(&2).unwrap().edges.len(), 1);

        builder.add_edge(1, 3, 1);
        assert_eq!(builder.graph.nodes.len(), 3);
        assert_eq!(builder.graph.nodes.get(&1).unwrap().edges.len(), 2);
        assert_eq!(builder.graph.nodes.get(&2).unwrap().edges.len(), 1);
        assert_eq!(builder.graph.nodes.get(&3).unwrap().edges.len(), 1);

        builder.add_edge(2, 3, 1);
        assert_eq!(builder.graph.nodes.len(), 3);
        assert_eq!(builder.graph.nodes.len(), 3);
        assert_eq!(builder.graph.nodes.get(&1).unwrap().edges.len(), 2);
        assert_eq!(builder.graph.nodes.get(&2).unwrap().edges.len(), 2);
        assert_eq!(builder.graph.nodes.get(&3).unwrap().edges.len(), 2);

        let graph = builder.build();
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.nodes.get(&1).unwrap().edges.len(), 2);
        assert_eq!(graph.nodes.get(&2).unwrap().edges.len(), 2);
        assert_eq!(graph.nodes.get(&3).unwrap().edges.len(), 2);
    }

    #[test]
    fn test_simple_graph() {
        let mut builder = SimpleGraphBuilder::<u64>::new("test");
        builder.add_edge(1, 2, 10);
        builder.add_edge(1, 3, 11);
        builder.add_edge(2, 3, 12);
        builder.add_directed_edge(3, 4, 14);
        let graph = builder.build();
        assert_eq!(graph.nodes().count(), 4);
        assert_eq!(graph.edges().count(), 7);

        assert_eq!(graph.node_edges(1).count(), 2);
        assert_eq!(graph.nodes_are_adjacent(1, 2), true);
        assert_eq!(graph.nodes_are_adjacent(1, 3), true);
        assert_eq!(graph.nodes_are_adjacent(1, 4), false);
        assert_eq!(graph.degree(1), 2);

        assert_eq!(graph.node_edges(2).count(), 2);
        assert_eq!(graph.nodes_are_adjacent(2, 1), true);
        assert_eq!(graph.nodes_are_adjacent(2, 3), true);
        assert_eq!(graph.nodes_are_adjacent(2, 4), false);
        assert_eq!(graph.degree(2), 2);

        assert_eq!(graph.node_edges(3).count(), 3);
        assert_eq!(graph.nodes_are_adjacent(3, 1), true);
        assert_eq!(graph.nodes_are_adjacent(3, 2), true);
        assert_eq!(graph.nodes_are_adjacent(3, 4), true);
        assert_eq!(graph.degree(3), 3);

        println!("{graph}");
    }
}
