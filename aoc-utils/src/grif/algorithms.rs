use super::{Edge, Graph, Node};
use std::collections::{HashMap, HashSet};

fn backtrack_cycle<'a, G: Graph<'a>>(
    id: G::Id,
    from_id: G::Id,
    visited: &HashMap<G::Id, Option<G::Id>>,
) -> Vec<G::Id> {
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

pub fn find_cycles_from_r<'a, G: Graph<'a>>(
    graph: &'a G,
    id: G::Id,
    from_id: Option<G::Id>,
    start_id: G::Id,
    level: usize,
    visited: &mut HashMap<G::Id, Option<G::Id>>,
    cycles: &mut Vec<Vec<G::Id>>,
) {
    let node = graph.node(&id).unwrap();
    if level == 0 {
        if node.is_adjacent(start_id) {
            cycles.push(backtrack_cycle::<G>(id, from_id.unwrap(), visited));
        }
    } else {
        let mut visited_r = visited.clone();
        visited_r.insert(id, from_id);
        for edge in node.edges() {
            if !visited_r.contains_key(&edge.b()) {
                visited_r.insert(edge.b(), Some(id));
                find_cycles_from_r(
                    graph,
                    edge.b(),
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

pub fn find_cycles<'a, G: Graph<'a>>(graph: &'a G, cycle_size: usize) -> Vec<Vec<G::Id>> {
    let mut visited: HashMap<G::Id, Option<G::Id>> = HashMap::new();
    let mut all_cycles: Vec<Vec<G::Id>> = vec![];
    graph.nodes().for_each(|node| {
        visited.insert(node.id(), None);
        find_cycles_from_r(
            graph,
            node.id(),
            None,
            node.id(),
            cycle_size - 1,
            &mut visited,
            &mut all_cycles,
        );
    });
    all_cycles
}

trait SetOps<Id: Eq + Clone + Copy> {
    fn union(&self, other: &[Id]) -> Vec<Id>;
    fn difference(&self, other: &[Id]) -> Vec<Id>;
    fn intersection(&self, other: &[Id]) -> Vec<Id>;
}

impl<Id: Eq + Clone + Copy> SetOps<Id> for Vec<Id> {
    fn union(&self, other: &[Id]) -> Vec<Id> {
        let mut union = self.clone();
        for &id in other {
            if !union.contains(&id) {
                union.push(id);
            }
        }
        union
    }

    fn difference(&self, other: &[Id]) -> Vec<Id> {
        let mut diff = vec![];
        for id in self {
            if !other.contains(id) {
                diff.push(*id);
            }
        }
        diff
    }

    fn intersection(&self, other: &[Id]) -> Vec<Id> {
        let mut intersection = vec![];
        for id in self {
            if other.contains(id) {
                intersection.push(*id);
            }
        }
        intersection
    }
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
fn find_maximal_clique_r<'a, G: Graph<'a>>(
    graph: &'a G,
    r: Vec<G::Id>,
    p: Vec<G::Id>,
    x: Vec<G::Id>,
) -> Option<Vec<G::Id>> {
    if p.is_empty() && x.is_empty() {
        return Some(r);
    }

    let mut max_clique: Option<Vec<G::Id>> = None;
    let mut p2 = p.clone();
    let mut x2 = x;
    for v in p {
        let n = graph.node(&v).unwrap().neighbours().collect::<Vec<_>>();
        if let Some(clique) = find_maximal_clique_r(
            graph,
            r.union(&[v]),
            p2.intersection(&n),
            x2.intersection(&n),
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
pub fn find_maximal_clique<'a, G: Graph<'a>>(graph: &'a G, node_id: G::Id) -> Option<Vec<G::Id>> {
    let r = vec![node_id];
    let p = graph.node(&node_id)?.neighbours().collect::<Vec<_>>();
    let x = vec![];
    find_maximal_clique_r(graph, r, p, x)
}

// Find the maximum clique in the graph.
// The maximum clique of a graph is the clique with as many or more vertices than any
// other clique in the graph.
// Note: the clique number of the graph is the number of vertices in the maximu cli
// Note: A clique is a complete subgraph of the graph.
pub fn find_maximum_clique<'a, G: Graph<'a>>(graph: &'a G) -> Option<Vec<G::Id>> {
    let mut max = vec![];
    let mut checked: HashSet<G::Id> = HashSet::new();
    for node in graph.nodes() {
        if node.degree() >= max.len() && !checked.contains(&node.id()) {
            let clique = find_maximal_clique(graph, node.id())?;
            checked.extend(clique.iter());
            if clique.len() > max.len() {
                max = clique;
            }
        }
    }
    Some(max)
}

fn dfs_r<'a, G: Graph<'a>>(
    graph: &'a G,
    id: G::Id,
    visited: &mut HashSet<G::Id>,
    f: &mut dyn FnMut(&G::Node),
) {
    if visited.contains(&id) {
        return;
    }
    visited.insert(id);
    let node = graph.node(&id).unwrap();
    f(node);
    for edge in node.edges() {
        dfs_r(graph, edge.b(), visited, f);
    }
}

pub fn dfs<'a, G: Graph<'a>>(graph: &'a G, id: G::Id, f: &mut dyn FnMut(&G::Node)) {
    let mut visited: HashSet<G::Id> = HashSet::new();
    dfs_r(graph, id, &mut visited, f);
}

#[cfg(test)]
mod tests {

    use crate::grif::simple::SimpleGraphBuilder;

    use super::*;

    #[test]
    fn test_find_maximal_clique() {
        let g = SimpleGraphBuilder::parse("a-b\na-c\na-d\nb-c\nb-d\na-e\nb-e\nc-e", "-").unwrap();
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
        let g = SimpleGraphBuilder::parse("a-b\na-c\na-d\nb-c\nb-d\na-e\nb-e\nc-e", "-").unwrap();
        let mut max = find_maximum_clique(&g).unwrap();
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
    }

    #[test]
    fn test_difference() {
        let a = vec![1, 2, 3, 4, 5];
        let b = [2, 4, 6];
        let diff = a.difference(&b);
        assert_eq!(diff, vec![1, 3, 5]);
    }
}
