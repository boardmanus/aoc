use std::collections::BTreeMap;
use std::fmt::Display;
use std::hash::Hash;

use graphviz_rust::dot_structures as dots;

use super::{Builder, Graph};

type FnEdgeWeight<NodeId, Weight> = fn(from: &NodeId, to: &NodeId) -> Weight;

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum EdgeWeight<NodeId, Weight> {
    Static(Weight),
    Dynamic(FnEdgeWeight<NodeId, Weight>),
}

impl<NodeId, Weight> EdgeWeight<NodeId, Weight>
where
    NodeId: Copy,
    Weight: Copy,
{
    fn weight(&self, from: &NodeId, to: &NodeId) -> Weight {
        match self {
            EdgeWeight::Static(w) => *w,
            EdgeWeight::Dynamic(f) => f(from, to),
        }
    }
}

struct Node<NodeId, NodeValue, Weight> {
    id: NodeId,
    value: NodeValue,
    edges: BTreeMap<NodeId, EdgeWeight<NodeId, Weight>>,
}

pub struct SimpleGraph<NodeId, NodeValue, Weight>
where
    NodeId: Copy + Eq + Ord,
    Weight: Copy + Eq + Hash,
{
    name: String,
    nodes: BTreeMap<NodeId, Node<NodeId, NodeValue, Weight>>,
}

impl<NodeId, NodeValue, Weight> SimpleGraph<NodeId, NodeValue, Weight>
where
    NodeId: Copy + Eq + Ord + Display,
    Weight: Copy + Eq + Hash,
{
    pub fn to_viz(&self, digraph: bool) -> dots::Graph {
        super::to_viz::<Self>(self, digraph)
    }
}

impl<NodeId, NodeValue, Weight> Display for SimpleGraph<NodeId, NodeValue, Weight>
where
    NodeId: Copy + Eq + Ord + Display,
    NodeValue: Display,
    Weight: Copy + Eq + Hash + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        super::fmt_graph(self, f)
    }
}

impl<NodeId, NodeValue, Weight> Graph for SimpleGraph<NodeId, NodeValue, Weight>
where
    NodeId: Copy + Eq + Ord,
    Weight: Copy + Eq + Hash,
{
    type NodeId = NodeId;
    type NodeValue = NodeValue;
    type Weight = Weight;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn node(&self, id: &Self::NodeId) -> Option<&Self::NodeValue> {
        Some(&self.nodes.get(id)?.value)
    }

    fn nodes(&self) -> impl Iterator<Item = Self::NodeId> {
        self.nodes.values().map(|n| n.id)
    }

    fn node_edges(&self, node: Self::NodeId) -> impl Iterator<Item = (Self::NodeId, Self::Weight)> {
        self.nodes.get(&node).into_iter().flat_map(|node| {
            node.edges
                .iter()
                .map(|(&to, &edge_weight)| (to, edge_weight.weight(&node.id, &to)))
        })
    }
}

pub struct SimpleGraphBuilder<NodeId, NodeValue, Weight>
where
    NodeId: Copy + Eq + Ord,
    Weight: Copy + Eq + Hash,
{
    graph: SimpleGraph<NodeId, NodeValue, Weight>,
}

impl<NodeId, NodeValue, Weight> SimpleGraphBuilder<NodeId, NodeValue, Weight>
where
    NodeId: Copy + Eq + Ord + Display,
    Weight: Copy + Eq + Hash,
{
    fn new(name: &str) -> SimpleGraphBuilder<NodeId, NodeValue, Weight> {
        SimpleGraphBuilder {
            graph: SimpleGraph {
                name: name.to_string(),
                nodes: BTreeMap::new(),
            },
        }
    }
}

impl<'a> SimpleGraphBuilder<&'a str, char, u8> {
    pub fn parse(
        name: &str,
        input: &'a str,
        separator: &str,
    ) -> Option<SimpleGraph<&'a str, char, u8>> {
        let mut builder = SimpleGraphBuilder::new(name);
        input
            .lines()
            .filter_map(|line| {
                let mut it = line.split(separator);
                Some((it.next()?, it.next()?))
            })
            .for_each(|(a, b)| {
                builder.add_edge(a, b, 1);
            });

        Some(builder.build())
    }
}

impl<NodeId, NodeValue, Weight> Builder for SimpleGraphBuilder<NodeId, NodeValue, Weight>
where
    NodeId: Copy + Eq + Ord + Copy,
    NodeValue: Default,
    Weight: Copy + Eq + Hash,
{
    type NodeId = NodeId;
    type NodeValue = NodeValue;
    type Weight = Weight;
    type Graph = SimpleGraph<NodeId, NodeValue, Weight>;

    fn add_node(&mut self, id: NodeId) -> &mut Self {
        self.graph.nodes.entry(id).or_insert(Node {
            id,
            value: NodeValue::default(),
            edges: BTreeMap::new(),
        });
        self
    }

    fn add_node_edge(&mut self, a: NodeId, b: NodeId, weight: Weight) -> &mut Self {
        self.add_node(a);
        self.graph.nodes.entry(a).and_modify(|n| {
            n.edges.insert(b, EdgeWeight::Static(weight));
        });
        self
    }

    fn add_directed_edge(&mut self, a: NodeId, b: NodeId, weight: Weight) -> &mut Self {
        self.add_node(a);
        self.add_node(b);
        self.graph.nodes.entry(a).and_modify(|n| {
            n.edges.insert(b, EdgeWeight::Static(weight));
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
        let mut builder = SimpleGraphBuilder::<u8, u8, u8>::new("test");
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
        let mut builder = SimpleGraphBuilder::<u8, u8, u8>::new("test");
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
