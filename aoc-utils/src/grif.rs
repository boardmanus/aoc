pub mod algorithms;
pub mod simple;
pub mod simple_vec;
use graphviz_rust::dot_generator as dotg;
use graphviz_rust::dot_structures as dots;
use std::fmt::Display;

pub trait Node<'a> {
    type Id: Display + Copy + Eq + 'a;
    type Weight: Copy + 'a;
    type Edge: Edge<Id = Self::Id, Weight = Self::Weight> + 'a;

    fn id(&self) -> Self::Id;
    fn edges(&'a self) -> impl Iterator<Item = &'a Self::Edge> + 'a;

    fn is_adjacent(&'a self, node_id: Self::Id) -> bool {
        self.edges().any(|e| e.b() == node_id)
    }

    fn degree(&'a self) -> usize {
        self.edges().count()
    }

    fn neighbours(&'a self) -> impl Iterator<Item = <Self::Edge as Edge>::Id> {
        self.edges().map(move |e| e.b())
    }

    fn to_viz(&self) -> dots::Node {
        use graphviz_rust::dot_generator::*;
        use graphviz_rust::dot_structures::*;
        let node = node!("bob");
        node
    }
}

pub fn display_fmt_node<'a, N: Node<'a>>(
    node: &'a N,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    write!(f, "[{}: ", node.id())?;
    let mut edges = node.edges();
    if let Some(e) = edges.next() {
        display_fmt_edge(e, f)?;
        for e in edges {
            write!(f, ", ")?;
            display_fmt_edge(e, f)?;
        }
    }
    write!(f, "]")
}

pub trait Edge {
    type Id: Copy + Eq + Display;
    type Weight: Copy + Display;

    fn weight(&self) -> Self::Weight;
    fn a(&self) -> Self::Id;
    fn b(&self) -> Self::Id;

    fn to_viz(&self) -> dots::Edge {
        use graphviz_rust::dot_generator::*;
        use graphviz_rust::dot_structures::*;
        let node = dotg::edge!(node_id!(self.a().to_string()) => node_id!(self.b().to_string()));
        node
    }
}

pub fn display_fmt_edge<E: Edge>(edge: &E, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}-{}-{})", edge.a(), edge.weight(), edge.b())
}

pub trait Graph<'a> {
    type Id: Eq + Copy + 'a;
    type Weight: 'a;
    type Node: Node<'a, Id = Self::Id, Weight = Self::Weight> + 'a;

    fn name(&self) -> String;
    fn node(&self, id: &Self::Id) -> Option<&Self::Node>;
    fn nodes(&'a self) -> impl Iterator<Item = &'a Self::Node> + 'a;

    fn edges(&'a self) -> impl Iterator<Item = &'a <Self::Node as Node<'a>>::Edge> {
        self.nodes().flat_map(|n| n.edges())
    }

    fn to_viz(&'a self, name: &str, digraph: bool) -> dots::Graph {
        use graphviz_rust::dot_generator::*;
        use graphviz_rust::dot_structures::*;
        let stmts = self
            .edges()
            .map(|e| dots::Stmt::Edge(e.to_viz()))
            .collect::<Vec<_>>();
        if digraph {
            dots::Graph::DiGraph {
                id: id!(name),
                strict: true,
                stmts,
            }
        } else {
            dots::Graph::Graph {
                id: id!(name),
                strict: true,
                stmts,
            }
        }
    }
}

pub fn display_fmt_graph<'a, G: Graph<'a>>(
    graph: &'a G,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "<{}:", graph.name())?;
    for n in graph.nodes() {
        display_fmt_node(n, f)?;
        writeln!(f)?;
    }
    writeln!(f, ">")
}

pub trait Builder<'a> {
    type Id: Eq + Copy + 'a;
    type Weight: Copy + 'a;
    type Graph: Graph<'a, Id = Self::Id, Weight = Self::Weight>;

    fn add_node(&mut self, id: Self::Id) -> &mut <Self::Graph as Graph<'a>>::Node;
    fn add_node_edge(&mut self, a: Self::Id, b: Self::Id, weight: Self::Weight);
    fn build(self) -> Self::Graph;

    fn add_directed_edge(&mut self, a: Self::Id, b: Self::Id, weight: Self::Weight) {
        self.add_node_edge(a, b, weight);
        self.add_node(b);
    }

    fn add_edge(&mut self, a: Self::Id, b: Self::Id, weight: Self::Weight) {
        self.add_node_edge(a, b, weight);
        self.add_node_edge(b, a, weight);
    }
}
