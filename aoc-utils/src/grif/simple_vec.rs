use std::fmt::Display;

use super::{Builder, Edge, Graph, Node};

pub struct SimpleNode<Id, Weight>
where
    Id: Copy + Eq,
    Weight: Eq,
{
    id: Id,
    edges: Vec<SimpleEdge<Id, Weight>>,
}

impl<'a, Id, Weight> Node<'a> for SimpleNode<Id, Weight>
where
    Id: Display + Eq + Copy + 'a,
    Weight: Display + Eq + Copy + 'a,
{
    type Id = Id;
    type Weight = Weight;
    type Edge = SimpleEdge<Id, Weight>;

    fn id(&self) -> Id {
        self.id
    }

    fn edges(&'a self) -> impl Iterator<Item = &'a Self::Edge> {
        self.edges.iter()
    }
}

impl<Id, Weight> Display for SimpleNode<Id, Weight>
where
    Id: Display + Eq + Copy,
    Weight: Display + Eq + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        super::display_fmt_node(self, f)
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct SimpleEdge<Id: Eq, Weight: Eq> {
    a: Id,
    b: Id,
    weight: Weight,
}

impl<Id, Weight> Edge for SimpleEdge<Id, Weight>
where
    Id: Display + Eq + Copy,
    Weight: Display + Eq + Copy,
{
    type Id = Id;
    type Weight = Weight;

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

impl<Id, Weight> Display for SimpleEdge<Id, Weight>
where
    Id: Display + Eq + Copy,
    Weight: Display + Eq + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        super::display_fmt_edge(self, f)
    }
}

pub struct SimpleGraph<Id, Weight>
where
    Id: Copy + Eq,
    Weight: Eq,
{
    name: String,
    nodes: Vec<SimpleNode<Id, Weight>>,
}

impl<Id, Weight> Display for SimpleGraph<Id, Weight>
where
    Id: Display + Eq + Copy,
    Weight: Display + Eq + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        super::display_fmt_graph(self, f)
    }
}

impl<'a, Id, Weight> Graph<'a> for SimpleGraph<Id, Weight>
where
    Id: Display + Copy + Eq + 'a,
    Weight: Display + Eq + Copy + 'a,
{
    type Id = Id;
    type Weight = Weight;
    type Node = SimpleNode<Id, Weight>;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn node(&self, id: &Id) -> Option<&Self::Node> {
        self.nodes.iter().find(|n| n.id() == *id)
    }

    fn nodes(&'a self) -> impl Iterator<Item = &'a Self::Node> {
        self.nodes.iter()
    }
}

pub struct SimpleGraphBuilder<Id, Weight>
where
    Id: Copy + Eq,
    Weight: Copy + Eq,
{
    graph: SimpleGraph<Id, Weight>,
}

impl<Id, Weight> SimpleGraphBuilder<Id, Weight>
where
    Id: Copy + Eq,
    Weight: Copy + Eq,
{
    fn new(name: &str) -> SimpleGraphBuilder<Id, Weight> {
        SimpleGraphBuilder {
            graph: SimpleGraph {
                name: name.to_string(),
                nodes: Vec::new(),
            },
        }
    }
}

impl<'a> SimpleGraphBuilder<&'a str, u8> {
    pub fn parse(name: &str, input: &'a str, separator: &str) -> Option<SimpleGraph<&'a str, u8>> {
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

impl<'a, Id, Weight> Builder<'a> for SimpleGraphBuilder<Id, Weight>
where
    Id: Display + Eq + Copy + 'a,
    Weight: Display + Eq + Copy + 'a,
{
    type Id = Id;
    type Weight = Weight;
    type Graph = SimpleGraph<Id, Weight>;

    fn add_node(&mut self, id: Id) -> &mut <Self::Graph as Graph<'a>>::Node {
        if let Some(i) = self.graph.nodes.iter().position(|n| n.id() == id) {
            &mut self.graph.nodes[i]
        } else {
            self.graph.nodes.push(SimpleNode {
                id,
                edges: Vec::new(),
            });
            self.graph.nodes.last_mut().unwrap()
        }
    }

    fn add_node_edge(&mut self, a: Id, b: Id, weight: Weight) {
        let node = self.add_node(a);
        if let Some(i) = node.edges.iter().position(|e| e.b() == b) {
            node.edges[i].weight = weight;
        } else {
            node.edges.push(SimpleEdge { a, b, weight });
        }
    }

    fn add_directed_edge(&mut self, a: Id, b: Id, weight: Weight) {
        let node = self.add_node(a);
        if let Some(i) = node.edges.iter().position(|e| e.b() == b) {
            node.edges[i].weight = weight;
        } else {
            node.edges.push(SimpleEdge { a, b, weight });
        }
        self.add_node(b);
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
        let mut builder = SimpleGraphBuilder::new("test");
        builder.add_edge(1, 2, 1);
        assert_eq!(builder.graph.nodes.len(), 2);
        assert_eq!(builder.graph.node(&1).unwrap().edges.len(), 1);
        assert_eq!(builder.graph.node(&2).unwrap().edges.len(), 1);

        builder.add_edge(1, 3, 1);
        assert_eq!(builder.graph.nodes.len(), 3);
        assert_eq!(builder.graph.node(&1).unwrap().edges.len(), 2);
        assert_eq!(builder.graph.node(&2).unwrap().edges.len(), 1);
        assert_eq!(builder.graph.node(&3).unwrap().edges.len(), 1);

        builder.add_edge(2, 3, 1);
        assert_eq!(builder.graph.nodes.len(), 3);
        assert_eq!(builder.graph.nodes.len(), 3);
        assert_eq!(builder.graph.node(&1).unwrap().edges.len(), 2);
        assert_eq!(builder.graph.node(&2).unwrap().edges.len(), 2);
        assert_eq!(builder.graph.node(&3).unwrap().edges.len(), 2);

        let graph = builder.build();
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.node(&1).unwrap().edges.len(), 2);
        assert_eq!(graph.node(&2).unwrap().edges.len(), 2);
        assert_eq!(graph.node(&3).unwrap().edges.len(), 2);
    }

    #[test]
    fn test_simple_graph() {
        let mut builder = SimpleGraphBuilder::new("test");
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

        println!("{graph}");
    }
}
