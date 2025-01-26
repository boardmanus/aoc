use std::collections::HashMap;
use std::hash::Hash;

use super::{Edge, Graph, Node};

fn backtrack_cycle<'a, Id: Copy + Eq + Hash>(
    id: Id,
    from_id: Id,
    visited: &HashMap<Id, Option<Id>>,
) -> Vec<Id> {
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

pub fn find_cycles_from_r<'a, Id: Copy + Eq + Hash, Weight>(
    graph: &'a impl Graph<'a, Id, Weight>,
    id: Id,
    from_id: Option<Id>,
    start_id: Id,
    level: usize,
    visited: &mut HashMap<Id, Option<Id>>,
    cycles: &mut Vec<Vec<Id>>,
) {
    let node = graph.node(&id).unwrap();
    if level == 0 {
        if node.is_adjacent(start_id) {
            cycles.push(backtrack_cycle(id, from_id.unwrap(), visited));
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

pub fn find_cycles<'a, Id: Copy + Eq + Hash, Weight>(
    graph: &'a impl Graph<'a, Id, Weight>,
    cycle_size: usize,
) -> Vec<Vec<Id>> {
    let mut visited: HashMap<Id, Option<Id>> = HashMap::new();
    let mut all_cycles: Vec<Vec<Id>> = vec![];
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

fn find_maximal_clique_r<'a, Id: Copy + Eq + Hash, Weight>(
    graph: &'a impl Graph<'a, Id, Weight>,
    r: &[Id],
    clique: &Vec<Id>,
) -> Vec<Id> {
    let mut new_clique = clique.clone();
    let v = graph.node(&r[0]).unwrap();
    if v.degree() >= clique.len() {
        if clique.iter().all(|&id| v.is_adjacent(id)) {
            let rest = &r[1..];
            new_clique.push(v.id());
            if rest.len() > 0 {
                let mut max = vec![];
                for i in 0..rest.len() {
                    let new_max = find_maximal_clique_r(graph, &rest[i..], &new_clique);
                    if new_max.len() > max.len() {
                        max = new_max;
                    }
                }
                return max;
            }
        }
    }
    new_clique
}
// Find the maximal clique at a vertice in the graph.
// A clique is maximal if and only if it is not a subgraph of another clique in the graph.
// Note: A clique is a complete subgraph of the graph.
pub fn find_maximal_clique<'a, Id: Copy + Eq + Hash, Weight>(
    graph: &'a impl Graph<'a, Id, Weight>,
    node_id: Id,
) -> Vec<Id> {
    if let Some(node) = graph.node(&node_id) {
        let mut clique = vec![node_id];
        let r = node.edges().map(|e| e.b()).collect::<Vec<_>>();
        find_maximal_clique_r(graph, &r, &mut clique)
    } else {
        vec![]
    }
}

// Find the maximum clique in the graph.
// The maximum clique of a graph is the clique with as many or more vertices than any
// other clique in the graph.
// Note: the clique number of the graph is the number of vertices in the maximu cli
// Note: A clique is a complete subgraph of the graph.
pub fn find_maximum_clique<'a, Id: Copy + Eq + Hash, Weight>(
    graph: &'a impl Graph<'a, Id, Weight>,
) -> Vec<Id> {
    let mut max = vec![];
    for node in graph.nodes() {
        if node.degree() >= max.len() {
            let new_max = find_maximal_clique(graph, node.id());
            if new_max.len() > max.len() {
                max = new_max;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {

    use crate::grof::simple::SimpleGraphBuilder;

    use super::*;

    #[test]
    fn test_find_maximal_clique() {
        let g = SimpleGraphBuilder::parse("a-b\na-c\na-d\nb-c\nb-d\na-e\nb-e\nc-e", "-").unwrap();
        println!("{}", g);
        let mut max = find_maximal_clique(&g, "a");
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
        max = find_maximal_clique(&g, "b");
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
        max = find_maximal_clique(&g, "c");
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
        max = find_maximal_clique(&g, "d");
        max.sort();
        assert_eq!(max, vec!["a", "b", "d"]);
        max = find_maximal_clique(&g, "c");
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
    }

    #[test]
    fn test_find_maximum_clique() {
        let g = SimpleGraphBuilder::parse("a-b\na-c\na-d\nb-c\nb-d\na-e\nb-e\nc-e", "-").unwrap();
        let mut max = find_maximum_clique(&g);
        max.sort();
        assert_eq!(max, vec!["a", "b", "c", "e"]);
    }
}
