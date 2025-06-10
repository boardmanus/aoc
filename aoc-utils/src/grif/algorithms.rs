#![allow(dead_code)]

use num_traits::{AsPrimitive, Unsigned};

use super::Graph;
use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

fn backtrack_cycle<G>(
    id: G::NodeId,
    from_id: G::NodeId,
    visited: &HashMap<G::NodeId, Option<G::NodeId>>,
) -> Vec<G::NodeId>
where
    G: Graph,
    G::NodeId: Eq + Hash,
{
    let mut cycle = vec![id];
    let mut from_id = from_id;
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

pub fn find_cycles_from_r<G>(
    graph: &G,
    id: G::NodeId,
    from_id: Option<G::NodeId>,
    start_id: G::NodeId,
    level: usize,
    visited: &mut HashMap<G::NodeId, Option<G::NodeId>>,
    cycles: &mut Vec<Vec<G::NodeId>>,
) where
    G: Graph,
    G::NodeId: Eq + Hash,
{
    if level == 0 {
        if graph.nodes_are_adjacent(id, start_id) {
            cycles.push(backtrack_cycle::<G>(id, from_id.unwrap(), visited));
        }
    } else {
        let mut visited_r = visited.clone();
        visited_r.insert(id, from_id);
        for edge in graph.node_edges(id) {
            if let Entry::Vacant(e) = visited_r.entry(edge.0) {
                e.insert(Some(id));
                visited_r.insert(edge.0, Some(id));
                find_cycles_from_r(
                    graph,
                    edge.0,
                    Some(id),
                    start_id,
                    level - 1,
                    &mut visited_r,
                    cycles,
                );
            }
        }
    }
}

type FilterPred<NodeId> = fn(node: &NodeId) -> bool;
pub fn find_cycles<G>(
    graph: &G,
    cycle_size: usize,
    filter: FilterPred<G::NodeId>,
) -> Vec<Vec<G::NodeId>>
where
    G: Graph,
    G::NodeId: Eq + Hash,
{
    let mut visited: HashMap<G::NodeId, Option<G::NodeId>> = HashMap::new();
    let mut all_cycles: Vec<Vec<G::NodeId>> = vec![];
    graph.nodes().filter(filter).for_each(|node| {
        visited.insert(node, None);
        find_cycles_from_r(
            graph,
            node,
            None,
            node,
            cycle_size - 1,
            &mut visited,
            &mut all_cycles,
        );
    });
    all_cycles
}

fn union<NodeId>(a: &[NodeId], other: &[NodeId]) -> Vec<NodeId>
where
    NodeId: Copy + Eq + Hash,
{
    let mut union = Vec::from(a);
    for &id in other {
        if !union.contains(&id) {
            union.push(id);
        }
    }
    union
}

fn difference<NodeId>(a: &[NodeId], other: &[NodeId]) -> Vec<NodeId>
where
    NodeId: Copy + Eq + Hash,
{
    let mut diff = vec![];
    for &id in a {
        if !other.contains(&id) {
            diff.push(id);
        }
    }
    diff
}

fn intersection<NodeId>(a: &[NodeId], other: &[NodeId]) -> Vec<NodeId>
where
    NodeId: Copy + Eq + Hash,
{
    let mut intersection = vec![];
    for &id in a {
        if other.contains(&id) {
            intersection.push(id);
        }
    }
    intersection
}

// Bron-Kerbosch algorithm
// Find the maximal clique at a vertice in the graph.
// A clique is maximal if and only if it is not a subgraph of another clique in the graph.
// Note: A clique is a complete subgraph of the graph.
//
// algorithm BronKerbosch1(R, P, X) is
// if P and X are both empty then
//     report R as a maximal clique
// for each vertex v in P do
//     BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
//     P := P \ {v}
//     X := X ⋃ {v}
fn find_maximal_clique_r<G: Graph>(
    graph: &G,
    r: Vec<G::NodeId>,
    p: Vec<G::NodeId>,
    x: Vec<G::NodeId>,
) -> Option<Vec<G::NodeId>>
where
    G::NodeId: Copy + Eq + Hash,
{
    if p.is_empty() && x.is_empty() {
        return Some(r);
    }

    let mut max_clique: Option<Vec<G::NodeId>> = None;
    let mut p2 = p.clone();
    let mut x2 = x;
    for v in p {
        let n = graph.node_neighbours(v).collect::<Vec<_>>();
        if let Some(clique) = find_maximal_clique_r(
            graph,
            union(&r, &[v]),
            intersection(&p2, &n),
            intersection(&x2, &n),
        ) {
            if max_clique.is_none() || clique.len() > max_clique.as_ref().unwrap().len() {
                max_clique = Some(clique);
            }
        }
        p2.retain(|x| *x != v);
        x2.push(v);
    }

    max_clique
}

// Find the maximal clique at a vertice in the graph.
// A clique is maximal if and only if it is not a subgraph of another clique in the graph.
// Note: A clique is a complete subgraph of the graph.
pub fn find_maximal_clique<G: Graph>(graph: &G, node_id: G::NodeId) -> Option<Vec<G::NodeId>>
where
    G::NodeId: Copy + Eq + Hash,
{
    let r = vec![node_id];
    let p = graph.node_neighbours(node_id).collect::<Vec<_>>();
    let x = vec![];
    find_maximal_clique_r(graph, r, p, x)
}

// Find the maximum clique in the graph.
// The maximum clique of a graph is the clique with as many or more vertices than any
// other clique in the graph.
// Note: the clique number of the graph is the number of vertices in the maximu cli
// Note: A clique is a complete subgraph of the graph.
pub fn find_maximum_clique<G>(graph: &G) -> Option<Vec<G::NodeId>>
where
    G: Graph,
    G::NodeId: Eq + Hash,
{
    let mut max = vec![];
    let mut checked: HashSet<G::NodeId> = HashSet::new();
    for node in graph.nodes() {
        if graph.degree(node) >= max.len() && !checked.contains(&node) {
            let clique = find_maximal_clique(graph, node)?;
            checked.extend(clique.iter());
            if clique.len() > max.len() {
                max = clique;
            }
        }
    }
    Some(max)
}

struct PQ<NodeId>
where
    NodeId: Eq + Hash,
{
    q: Vec<NodeId>,
}

impl<NodeId> PQ<NodeId>
where
    NodeId: Eq + Hash,
{
    fn new(start: NodeId) -> PQ<NodeId> {
        PQ { q: vec![start] }
    }

    fn push(&mut self, node: NodeId) {
        self.q.push(node);
    }

    fn sort(&mut self, distances: &HashMap<NodeId, (usize, Option<NodeId>)>) {
        self.q.sort_by(|a, b| distances[b].0.cmp(&distances[a].0));
    }

    fn pop(&mut self) -> Option<NodeId> {
        self.q.pop()
    }
}

pub fn shortest_path_djikstra<G>(
    graph: &G,
    start: G::NodeId,
    end: G::NodeId,
) -> Option<Vec<G::NodeId>>
where
    G: Graph,
    G::NodeId: Copy + Eq + Hash,
    G::Weight: Unsigned + AsPrimitive<usize>,
{
    let mut distances: HashMap<G::NodeId, (usize, Option<G::NodeId>)> = HashMap::new();
    distances.insert(start, (0, None));
    let mut visited: HashSet<G::NodeId> = HashSet::new();
    let mut pq = PQ::new(start);

    while let Some(node) = pq.pop() {
        if node == end {
            let mut path = vec![node];
            let mut from = distances[&node].1;
            while let Some(prev) = from {
                path.push(prev);
                from = distances[&prev].1;
            }
            path.reverse();
            return Some(path);
        }
        graph
            .node_edges(node)
            .filter(|(neighbour, _weight)| !visited.contains(neighbour))
            .for_each(|(neighbour, weight)| {
                let new_dist = distances[&node].0 + weight.as_();
                let distance = distances.entry(neighbour).or_insert((new_dist, Some(node)));
                if new_dist < distance.0 {
                    *distance = (new_dist, Some(node));
                }
                pq.push(neighbour);
            });
        visited.insert(node);
        pq.sort(&distances);
    }

    None
}

fn is_acyclic_r<G>(
    graph: &G,
    node: G::NodeId,
    undirected: bool,
    last_node: Option<G::NodeId>,
    visited: &mut BTreeSet<G::NodeId>,
    queue: &mut Vec<G::NodeId>,
) -> bool
where
    G: Graph + ?Sized,
    G::NodeId: Copy + Eq + Hash + Display,
{
    if queue.contains(&node) {
        // Cycle detected (back-edge)
        return false;
    }

    if visited.contains(&node) {
        // Already visited, no cycle (not a back-edge)
        return true;
    }

    visited.insert(node);
    queue.push(node);

    for edge in graph.node_edges(node) {
        if undirected && last_node == Some(edge.0) {
            // If the edge points to the last node, skip it
            continue;
        }
        if !is_acyclic_r(graph, edge.0, undirected, Some(node), visited, queue) {
            // Cycle detected in recursion
            return false;
        }
    }

    let n = queue.pop();
    assert!(n == Some(node));

    true
}

pub fn is_acyclic<G>(graph: &G, start: G::NodeId, undirected: bool) -> bool
where
    G: Graph + ?Sized,
    G::NodeId: Copy + Eq + Hash + Display,
{
    let mut visited = BTreeSet::<G::NodeId>::new();
    let mut queue = Vec::<G::NodeId>::new();
    is_acyclic_r(graph, start, undirected, None, &mut visited, &mut queue)
}

#[cfg(test)]
mod tests {

    use crate::grif::simple as sh;

    use super::*;

    #[test]
    fn test_is_acyclic() {
        let g = sh::SimpleGraphBuilder::<&str>::parse("acyclic", "a-b\nb-c\nc-d", "-").unwrap();
        assert!(is_acyclic(&g, "a", true));
        let g = sh::SimpleGraphBuilder::<&str>::parse("cyclic", "a-b\nb-c\nc-a", "-").unwrap();
        assert!(!is_acyclic(&g, "a", true));
        let g = sh::SimpleGraphBuilder::<&str>::parse("cyclic", "a-b\nb-c\na-c", "-").unwrap();
        assert!(!is_acyclic(&g, "a", true));
        let g = sh::SimpleGraphBuilder::<&str>::parse_directed("acyclic", "a-b\nb-c\na-c", "-")
            .unwrap();
        assert!(is_acyclic(&g, "a", false));
        let g = sh::SimpleGraphBuilder::<&str>::parse_directed(
            "acyclic",
            "a-b\nb-c\nc-d\na-e\ne-f\nf-d\nd-g",
            "-",
        )
        .unwrap();
        assert!(is_acyclic(&g, "a", false));
        let g = sh::SimpleGraphBuilder::<&str>::parse_directed(
            "cyclic",
            "a-b\nb-c\nc-d\ne-a\nf-e\nd-f\nd-g",
            "-",
        )
        .unwrap();
        assert!(!is_acyclic(&g, "a", false));
    }

    #[test]
    fn test_find_maximal_clique() {
        let g =
            sh::SimpleGraphBuilder::parse("clique", "a-b\na-c\na-d\nb-c\nb-d\na-e\nb-e\nc-e", "-")
                .unwrap();
        println!("{}", g);
        let mut max = find_maximal_clique(&g, "a").unwrap();
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
        max = find_maximal_clique(&g, "b").unwrap();
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
        max = find_maximal_clique(&g, "c").unwrap();
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
        max = find_maximal_clique(&g, "d").unwrap();
        max.sort();
        assert_eq!(max, vec!["a", "b", "d"]);
        max = find_maximal_clique(&g, "c").unwrap();
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
    }

    #[test]
    fn test_find_maximum_clique() {
        let g = sh::SimpleGraphBuilder::<&str>::parse(
            "clique",
            "a-b\na-c\na-d\nb-c\nb-d\na-e\nb-e\nc-e",
            "-",
        )
        .unwrap();
        let mut max = find_maximum_clique(&g).unwrap();
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
    }

    #[test]
    fn test_difference() {
        let a = [1, 2, 3, 4, 5];
        let b = [2, 4, 6];
        let diff = difference(&a, &b);
        assert_eq!(diff, vec![1, 3, 5]);
    }

    #[test]
    fn test_djikstra() {
        let g = sh::SimpleGraphBuilder::<&str>::parse("djikstra", "a-b\na-c\nb-d\nb-e\nc-e", "-")
            .unwrap();
        let path = shortest_path_djikstra(&g, "a", "e").unwrap();
        assert_eq!(path, vec!["a", "c", "e"]);
    }
}
