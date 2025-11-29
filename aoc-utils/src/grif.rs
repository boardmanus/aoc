pub mod algorithms;
pub mod iterators;
pub mod simple;
use graphviz_rust::dot_generator as dotg;
use graphviz_rust::dot_structures as dots;
use std::fmt::Display;

pub trait Graph {
    type NodeId: Copy + Clone + PartialEq + Ord;
    type NodeValue;
    type Weight;

    fn name(&self) -> String;
    fn nodes(&self) -> impl Iterator<Item = Self::NodeId>;
    fn node(&self, id: &Self::NodeId) -> Option<&Self::NodeValue>;
    fn node_edges(&self, node: Self::NodeId) -> impl Iterator<Item = (Self::NodeId, Self::Weight)>;

    fn nodes_are_adjacent(&self, a: Self::NodeId, b: Self::NodeId) -> bool {
        self.node_edges(a).any(|e| e.0 == b)
    }

    fn degree(&self, node: Self::NodeId) -> usize {
        self.node_edges(node).count()
    }

    fn node_neighbours(&self, node: Self::NodeId) -> impl Iterator<Item = Self::NodeId> {
        self.node_edges(node).map(|e| e.0)
    }

    fn edges(&self) -> impl Iterator<Item = (Self::NodeId, Self::NodeId, Self::Weight)> {
        self.nodes()
            .flat_map(|n| self.node_edges(n).map(move |e| (n, e.0, e.1)))
    }

    fn dfs<Pred>(&self, start: Self::NodeId, filter: Pred) -> impl Iterator<Item = Self::NodeId>
    where
        Pred: Fn(&Self::NodeId) -> bool,
    {
        iterators::DfsIter::new(self, start, filter)
    }

    fn bfs(&self, start: Self::NodeId) -> impl Iterator<Item = (Self::NodeId, usize)> {
        iterators::BfsIter::new(self, start)
    }

    fn bfs_path(
        &self,
        start: Self::NodeId,
        all_paths: bool,
    ) -> impl Iterator<Item = iterators::Path<Self>> {
        iterators::BfsPathIter::new(self, start, all_paths)
    }
}

fn fmt_node<G: Graph>(g: &G, n: G::NodeId, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
where
    G::NodeId: Display,
    G::Weight: Display,
{
    write!(f, "[{}: ", n)?;
    let mut edges = g.node_edges(n);
    if let Some(e) = edges.next() {
        write!(f, "|-{}-{})", e.1, e.0)?;
        for e in edges {
            write!(f, ", ")?;
            write!(f, "|-{}-{})", e.1, e.0)?;
        }
    }
    write!(f, "]")
}

pub fn fmt_graph<G: Graph>(g: &G, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
where
    G::NodeId: Display,
    G::Weight: Display,
{
    writeln!(f, "<{}:", g.name())?;
    for n in g.nodes() {
        fmt_node(g, n, f)?;
        writeln!(f)?;
    }
    writeln!(f, ">")
}

pub fn to_viz<G: Graph>(g: &G, digraph: bool) -> dots::Graph
where
    G::NodeId: Display,
{
    use graphviz_rust::dot_generator::*;
    use graphviz_rust::dot_structures::*;
    let mut stmts = g
        .nodes()
        .map(|n| Stmt::Node(dotg::node!(n)))
        .collect::<Vec<_>>();
    stmts.extend(g.edges().map(|e| {
        dots::Stmt::Edge(
            // Add weight attribute: EdgeAttribute::weight(e.2.to_string())
            dotg::edge!(node_id!(e.0.to_string()) => node_id!(e.1.to_string())),
        )
    }));
    if digraph {
        dots::Graph::DiGraph {
            id: id!(g.name()),
            strict: false,
            stmts,
        }
    } else {
        dots::Graph::Graph {
            id: id!(g.name()),
            strict: false,
            stmts,
        }
    }
}

pub trait Builder {
    type NodeId: PartialEq + Copy;
    type NodeValue: Default;
    type Weight: Copy;
    type Graph: Graph<NodeId = Self::NodeId, NodeValue = Self::NodeValue, Weight = Self::Weight>;

    fn add_node(&mut self, id: Self::NodeId) -> &mut Self;
    fn add_node_edge(
        &mut self,
        a: Self::NodeId,
        b: Self::NodeId,
        weight: Self::Weight,
    ) -> &mut Self;
    fn build(self) -> Self::Graph;

    fn add_directed_edge(
        &mut self,
        a: Self::NodeId,
        b: Self::NodeId,
        weight: Self::Weight,
    ) -> &mut Self {
        self.add_node_edge(a, b, weight).add_node(b)
    }

    fn add_edge(&mut self, a: Self::NodeId, b: Self::NodeId, weight: Self::Weight) -> &mut Self {
        self.add_node_edge(a, b, weight).add_node_edge(b, a, weight)
    }
}
