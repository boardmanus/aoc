use std::collections::hash_map::Entry;
use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::slice::Iter;

use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures as dots;

#[derive(Debug)]
pub struct Node<Id, Weight> {
    pub id: Id,
    pub edges: Vec<Edge<Id, Weight>>, //pub edges: HashMap<Id, Weight>,
}

impl<Id: PartialEq, Weight> Node<Id, Weight> {
    fn new(id: Id) -> Node<Id, Weight> {
        Node {
            id,
            edges: Vec::<Edge<Id, Weight>>::new(),
        }
    }

    fn is_adjacent(&self, node: &Id) -> bool {
        self.edges.iter().any(|e| e.b == *node)
    }

    fn degree(&self) -> usize {
        self.edges.len()
    }
}

impl<Id: ToString, Weight> Node<Id, Weight> {
    pub fn to_viz(&self) -> dots::Node {
        use graphviz_rust::dot_structures::*;
        let node = node!(self.id.to_string());
        node
    }
}

#[derive(Debug)]
pub struct Edge<Id, Weight> {
    a: Id,
    b: Id,
    weight: Weight,
}

impl<Id, Weight> Edge<Id, Weight> {
    pub fn new(a: Id, b: Id, weight: Weight) -> Edge<Id, Weight> {
        Edge { a, b, weight }
    }
}

impl<Id: ToString, Weight> Edge<Id, Weight> {
    pub fn to_viz(&self) -> dots::Edge {
        use graphviz_rust::dot_structures::*;
        let node = edge!(node_id!(self.a.to_string()) => node_id!(self.b.to_string()));
        node
    }
}

impl<Id: Display, Weight: Display> Display for Edge<Id, Weight> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}-{}-{})", self.a, self.weight, self.b)
    }
}

pub struct Graph<Id, Weight> {
    nodes: HashMap<Id, Node<Id, Weight>>,
    edges: Vec<Edge<Id, Weight>>,
}

impl<Id, Weight> Graph<Id, Weight> {
    pub fn nodes(&self) -> Values<Id, Node<Id, Weight>> {
        self.nodes.values()
    }

    pub fn edges(&self) -> Iter<'_, Edge<Id, Weight>> {
        self.edges.iter()
    }
}

impl<'a> Graph<&str, u8> {
    pub fn parse(input: &'a str, separator: &str) -> Option<Graph<&'a str, u8>> {
        let mut edges: Vec<Edge<&str, u8>> = Vec::new();
        let nodes = input
            .lines()
            .filter_map(|line| {
                let mut it = line.split(separator);
                Some((it.next()?, it.next()?))
            })
            .fold(
                HashMap::<&str, Node<&str, u8>>::new(),
                |mut nodes, (a, b)| {
                    let a_node = nodes.entry(a).or_insert(Node::new(a));
                    a_node.edges.push(Edge::new(a, b, 1));
                    edges.push(Edge::new(a, b, 1));
                    let b_node = nodes.entry(b).or_insert(Node::new(b));
                    edges.push(Edge::new(b, a, 1));
                    b_node.edges.push(Edge::new(b, a, 1));
                    nodes
                },
            );

        Some(Graph { nodes, edges })
    }
}

impl<Id: Display, Weight: Display> Display for Graph<Id, Weight> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, node) in &self.nodes {
            let edges = node
                .edges
                .iter()
                .map(|edge| edge.to_string())
                .collect::<Vec<_>>()
                .join(",");
            write!(f, "<{}>[{}]", id, edges)?;
        }
        Ok(())
    }
}

impl<Id: ToString, Weight> Graph<Id, Weight> {
    pub fn to_viz(&self, name: &str, digraph: bool) -> dots::Graph {
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

impl<Id: Eq + Hash + Copy, Weight> Graph<Id, Weight> {
    fn backtrack_cycle(&self, id: &Id, from_id: &Id, visited: &HashMap<Id, Option<Id>>) -> Vec<Id> {
        let mut cycle = vec![*id];
        let mut from_id = *from_id;
        while let Some(maybe_from) = visited.get(&from_id) {
            cycle.push(from_id);
            if let Some(from) = maybe_from {
                from_id = *from;
            } else {
                break;
            }
        }
        cycle
    }

    pub fn find_cycles_from_r(
        &self,
        id: &Id,
        from_id: Option<Id>,
        start_id: &Id,
        level: usize,
        visited: &mut HashMap<Id, Option<Id>>,
        cycles: &mut Vec<Vec<Id>>,
    ) {
        let node: &Node<Id, Weight> = self.nodes.get(id).unwrap();
        if level == 0 {
            if node.is_adjacent(start_id) {
                cycles.push(self.backtrack_cycle(id, &from_id.unwrap(), visited));
            }
        } else {
            let mut visited_r = visited.clone();
            visited_r.insert(*id, from_id);
            for edge in node.edges.iter() {
                if let Entry::Vacant(e) = visited_r.entry(edge.b) {
                    e.insert(Some(*id));
                    self.find_cycles_from_r(
                        &edge.b,
                        Some(*id),
                        start_id,
                        level - 1,
                        &mut visited_r,
                        cycles,
                    );
                }
            }
        }
    }

    pub fn find_cycles(
        &self,
        cycle_size: usize,
        filter: fn(&&Node<Id, Weight>) -> bool,
    ) -> Vec<Vec<Id>> {
        let mut visited: HashMap<Id, Option<Id>> = HashMap::new();
        let mut all_cycles: Vec<Vec<Id>> = vec![];
        self.nodes().filter(filter).for_each(|node| {
            visited.insert(node.id, None);
            self.find_cycles_from_r(
                &node.id,
                None,
                &node.id,
                cycle_size - 1,
                &mut visited,
                &mut all_cycles,
            );
        });
        all_cycles
    }

    fn find_maximal_clique_r(&self, r: &[Id], clique: &[Id]) -> Vec<Id> {
        let mut new_clique = clique.to_owned();
        let v = self.nodes.get(&r[0]).unwrap();
        if v.degree() >= clique.len() && clique.iter().all(|id| v.is_adjacent(id)) {
            let rest = &r[1..];
            new_clique.push(v.id);
            if !rest.is_empty() {
                let mut max = vec![];
                for i in 0..rest.len() {
                    let new_max = self.find_maximal_clique_r(&rest[i..], &new_clique);
                    if new_max.len() > max.len() {
                        max = new_max;
                    }
                }
                return max;
            }
        }
        new_clique
    }
    // Find the maximal clique at a vertice in the graph.
    // A clique is maximal if and only if it is not a subgraph of another clique in the graph.
    // Note: A clique is a complete subgraph of the graph.
    pub fn find_maximal_clique(&self, node_id: Id) -> Vec<Id> {
        if let Some(node) = self.nodes.get(&node_id) {
            let clique = vec![node_id];
            let r = node.edges.iter().map(|e| e.b).collect::<Vec<_>>();
            self.find_maximal_clique_r(&r, &clique)
        } else {
            vec![]
        }
    }

    // Find the maximum clique in the graph.
    // The maximum clique of a graph is the clique with as many or more vertices than any
    // other clique in the graph.
    // Note: the clique number of the graph is the number of vertices in the maximu cli
    // Note: A clique is a complete subgraph of the graph.
    pub fn find_maximum_clique(&self, filter: fn(&&Node<Id, Weight>) -> bool) -> Vec<Id> {
        let mut max = vec![];
        for node in self.nodes.values().filter(filter) {
            if node.degree() >= max.len() {
                let new_max = self.find_maximal_clique(node.id);
                if new_max.len() > max.len() {
                    max = new_max;
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_maximal_clique() {
        let g = Graph::parse("a-b\na-c\na-d\nb-c\nb-d\na-e\nb-e\nc-e", "-").unwrap();
        let mut max = g.find_maximal_clique("a");
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
        max = g.find_maximal_clique("b");
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
        max = g.find_maximal_clique("c");
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
        max = g.find_maximal_clique("d");
        max.sort();
        assert_eq!(max, vec!["a", "b", "d"]);
        max = g.find_maximal_clique("c");
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
    }

    #[test]
    fn test_find_maximum_clique() {
        let g = Graph::parse("a-b\na-c\na-d\nb-c\nb-d\na-e\nb-e\nc-e", "-").unwrap();
        let mut max = g.find_maximum_clique(|_| true);
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
    }
}
