use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

use super::{Builder, Edge, Graph, Node};

//trait SimpleId: Eq + Hash + Clone + Copy {}
//trait SimpleWeight: Eq + Hash + Clone + Copy {}
//#![feature(trait_alias)]
//pub trait SimpleId = Eq + Hash + Copy;

pub struct SimpleNode<Id: Eq + Hash, Weight: Eq + Hash> {
    id: Id,
    edges: HashSet<SimpleEdge<Id, Weight>>,
}

impl<Id: Display + Eq + Hash + Copy, Weight: Display + Eq + Hash + Copy> Display
    for SimpleNode<Id, Weight>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]:", self.id())?;
        for edge in &self.edges {
            write!(f, "{edge}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct SimpleEdge<Id: Eq + Hash, Weight: Eq + Hash> {
    a: Id,
    b: Id,
    weight: Weight,
}

impl<Id: Display + Eq + Hash + Copy, Weight: Display + Eq + Hash + Copy> Display
    for SimpleEdge<Id, Weight>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}-{}-{})", self.a(), self.weight(), self.b())
    }
}

pub struct SimpleGraph<Id: Eq + Hash, Weight: Eq + Hash> {
    nodes: HashMap<Id, SimpleNode<Id, Weight>>,
}

impl<Id: Display + Eq + Hash + Copy, Weight: Display + Eq + Hash + Copy> Display
    for SimpleGraph<Id, Weight>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.nodes()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

pub struct SimpleGraphBuilder<Id: Eq + Hash, Weight: Eq + Hash> {
    graph: SimpleGraph<Id, Weight>,
}

impl<Id: Eq + Hash, Weight: Eq + Hash> SimpleGraphBuilder<Id, Weight> {
    fn new() -> SimpleGraphBuilder<Id, Weight> {
        SimpleGraphBuilder {
            graph: SimpleGraph {
                nodes: HashMap::new(),
            },
        }
    }
}

impl<'a> SimpleGraphBuilder<&'a str, u8> {
    pub fn parse(input: &'a str, separator: &str) -> Option<SimpleGraph<&'a str, u8>> {
        let mut builder = SimpleGraphBuilder::new();
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

impl<'a, Id: Eq + Hash + Copy + 'a, Weight: Eq + Hash + Copy + 'a> Builder<'a, Id, Weight>
    for SimpleGraphBuilder<Id, Weight>
{
    type Graph = SimpleGraph<Id, Weight>;

    fn add_node(&mut self, id: Id) -> &mut <Self::Graph as Graph<'a, Id, Weight>>::Node {
        self.graph.nodes.entry(id).or_insert(SimpleNode {
            id,
            edges: HashSet::new(),
        })
    }

    fn add_node_edge(&mut self, a: Id, b: Id, weight: Weight) {
        self.add_node(a).edges.insert(SimpleEdge { a, b, weight });
    }

    fn add_directed_edge(&mut self, a: Id, b: Id, weight: Weight) {
        self.add_node(a).edges.insert(SimpleEdge { a, b, weight });
        self.add_node(b);
    }

    fn build(self) -> Self::Graph {
        self.graph
    }
}

impl<Id: Eq + Hash + Copy, Weight: Eq + Hash + Copy> Edge<Id, Weight> for SimpleEdge<Id, Weight> {
    fn weight(&self) -> Weight {
        self.weight
    }

    fn a(&self) -> Id {
        self.a
    }

    fn b(&self) -> Id {
        self.b
    }
}

impl<'a, Id: Eq + Hash + Copy + 'a, Weight: Eq + Hash + Copy + 'a> Node<'a, Id, Weight>
    for SimpleNode<Id, Weight>
{
    type Edge = SimpleEdge<Id, Weight>;

    fn id(&self) -> Id {
        self.id
    }

    fn edges(&'a self) -> impl Iterator<Item = &'a Self::Edge> {
        self.edges.iter()
    }
}

impl<'a, Id: Copy + Eq + Hash + 'a, Weight: Eq + Hash + Copy + 'a> Graph<'a, Id, Weight>
    for SimpleGraph<Id, Weight>
{
    type Node = SimpleNode<Id, Weight>;

    fn node(&self, id: &Id) -> Option<&Self::Node> {
        self.nodes.get(id)
    }

    fn nodes(&'a self) -> impl Iterator<Item = &'a Self::Node> {
        self.nodes.iter().map(|(_, n)| n)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_simple_graph_builder() {
        let mut builder = SimpleGraphBuilder::new();
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
        let mut builder = SimpleGraphBuilder::new();
        builder.add_edge(1, 2, 10);
        builder.add_edge(1, 3, 11);
        builder.add_edge(2, 3, 12);
        builder.add_directed_edge(3, 4, 14);
        let graph = builder.build();
        assert_eq!(graph.nodes().count(), 4);
        assert_eq!(graph.edges().count(), 7);

        assert_eq!(graph.node(&1).unwrap().edges().count(), 2);
        assert_eq!(graph.node(&1).unwrap().is_adjacent(2), true);
        assert_eq!(graph.node(&1).unwrap().is_adjacent(3), true);
        assert_eq!(graph.node(&1).unwrap().is_adjacent(4), false);
        assert_eq!(graph.node(&1).unwrap().degree(), 2);

        assert_eq!(graph.node(&2).unwrap().edges().count(), 2);
        assert_eq!(graph.node(&1).unwrap().is_adjacent(2), true);
        assert_eq!(graph.node(&1).unwrap().is_adjacent(3), true);
        assert_eq!(graph.node(&1).unwrap().is_adjacent(4), false);
        assert_eq!(graph.node(&2).unwrap().degree(), 2);

        assert_eq!(graph.node(&3).unwrap().edges().count(), 3);
        assert_eq!(graph.node(&3).unwrap().is_adjacent(1), true);
        assert_eq!(graph.node(&3).unwrap().is_adjacent(2), true);
        assert_eq!(graph.node(&3).unwrap().is_adjacent(4), true);
        assert_eq!(graph.node(&3).unwrap().degree(), 3);

        for node in graph.nodes() {
            println!("Node: {}", node.id());
            for edge in node.edges() {
                println!("Edge: {}-{}-{}", edge.a(), edge.weight(), edge.b());
            }
        }
    }
}
