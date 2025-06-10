use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::Graph;

pub struct DfsIter<'a, G, Pred>
where
    G: Graph + ?Sized,
    Pred: Fn(&G::NodeId) -> bool,
{
    graph: &'a G,
    visited: BTreeSet<G::NodeId>,
    stack: Vec<G::NodeId>,
    filter: Pred,
}

impl<'a, G, Pred> DfsIter<'a, G, Pred>
where
    G: Graph + ?Sized,
    G::NodeId: Copy + Eq + Ord,
    Pred: Fn(&G::NodeId) -> bool,
{
    pub fn new(graph: &'a G, start: G::NodeId, filter: Pred) -> DfsIter<'a, G, Pred>
    where
        Pred: Fn(&G::NodeId) -> bool,
    {
        DfsIter {
            graph,
            visited: BTreeSet::from([start]),
            stack: vec![start],
            filter,
        }
    }
}

impl<'a, G, Pred> Iterator for DfsIter<'a, G, Pred>
where
    G: Graph + ?Sized,
    G::NodeId: Copy + Eq + Ord,
    Pred: Fn(&G::NodeId) -> bool,
{
    type Item = G::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            for edge in self.graph.node_edges(node).filter(|e| (self.filter)(&e.0)) {
                // Add all new nodes that haven't been marked for visitation
                if !self.visited.contains(&edge.0) {
                    self.stack.push(edge.0);
                    self.visited.insert(edge.0);
                }
            }
            Some(node)
        } else {
            None
        }
    }
}

pub struct DfsPostIter<'a, G>
where
    G: Graph + ?Sized,
{
    graph: &'a G,
    visited: BTreeSet<G::NodeId>,
    stack: Vec<G::NodeId>,
}

impl<'a, G> DfsPostIter<'a, G>
where
    G: Graph + ?Sized,
    G::NodeId: Copy + Eq + Ord,
{
    pub fn new(graph: &'a G, start: G::NodeId) -> DfsPostIter<'a, G> {
        DfsPostIter {
            graph,
            visited: BTreeSet::from([]),
            stack: vec![start],
        }
    }
}

impl<'a, G> Iterator for DfsPostIter<'a, G>
where
    G: Graph + ?Sized,
    G::NodeId: Copy + Eq + Ord,
{
    type Item = G::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            if !self.visited.contains(&node) {
                self.stack.push(node);
                self.visited.insert(node);
                for edge in self.graph.node_edges(node) {
                    // Add all new nodes that haven't been marked for visitation
                    if !self.visited.contains(&edge.0) && !self.stack.contains(&edge.0) {
                        self.stack.push(edge.0);
                    }
                }
            } else {
                return Some(node);
            }
        }
        None
    }
}

pub struct BfsIter<'a, G>
where
    G: Graph + ?Sized,
{
    graph: &'a G,
    visited: BTreeMap<G::NodeId, usize>,
    stack: VecDeque<(G::NodeId, usize)>,
}

impl<'a, G> BfsIter<'a, G>
where
    G: Graph + ?Sized,
    G::NodeId: Copy + Eq + Ord,
{
    pub fn new(graph: &'a G, start: G::NodeId) -> BfsIter<'a, G> {
        BfsIter {
            graph,
            visited: BTreeMap::from([(start, 0)]),
            stack: VecDeque::from([(start, 0)]),
        }
    }
}

impl<'a, G> Iterator for BfsIter<'a, G>
where
    G: Graph + ?Sized,
    G::NodeId: Copy + Eq + Ord,
{
    type Item = (G::NodeId, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((node, level)) = self.stack.pop_front() {
            for edge in self.graph.node_edges(node) {
                // Add all new nodes that haven't been marked for visitation
                if !self.visited.contains_key(&edge.0) {
                    self.stack.push_back((edge.0, level + 1));
                    self.visited.insert(edge.0, level + 1);
                }
            }
            Some((node, level))
        } else {
            None
        }
    }
}

pub struct BfsPostIter<G>
where
    G: Graph + ?Sized,
{
    stack: Vec<G::NodeId>,
}

impl<G> BfsPostIter<G>
where
    G: Graph + ?Sized,
    G::NodeId: Copy + Eq + Ord,
{
    pub fn new(graph: &G, start: G::NodeId) -> BfsPostIter<G> {
        let mut visited: BTreeSet<G::NodeId> = BTreeSet::from([]);
        let mut stack = vec![start];
        let mut i: usize = 0;
        while let Some(node) = stack.get(i) {
            if !visited.contains(node) {
                visited.insert(*node);
                for edge in graph.node_edges(*node) {
                    // Add all new nodes that haven't been marked for visitation
                    if !visited.contains(&edge.0) && !stack.contains(&edge.0) {
                        stack.push(edge.0);
                    }
                }
                i += 1;
            }
        }
        BfsPostIter { stack }
    }
}

impl<G> Iterator for BfsPostIter<G>
where
    G: Graph + ?Sized,
    G::NodeId: Copy + Eq + Ord,
{
    type Item = G::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

#[cfg(test)]
mod tests {
    use crate::grif::simple as sh;

    use super::*;

    #[test]
    fn test_dfs_iterator() {
        let g = sh::SimpleGraphBuilder::<&str>::parse(
            "iter",
            "a-b\na-c\nb-d\nc-e\nd-f\ne-g\nf-h\ng-h",
            "-",
        )
        .unwrap();

        let dfs_iter = DfsIter::new(&g, "a", |_| true);
        assert_eq!(
            dfs_iter.collect::<Vec<_>>(),
            vec!["a", "c", "e", "g", "h", "f", "d", "b"]
        );
    }

    #[test]
    fn test_dfs_post_iterator() {
        let g = sh::SimpleGraphBuilder::parse("iter", "a-b\na-c\nb-d\nc-e\nd-f\ne-g", "-").unwrap();

        let dfs_iter = DfsPostIter::new(&g, "a");
        assert_eq!(
            dfs_iter.collect::<Vec<_>>(),
            vec!["g", "e", "c", "f", "d", "b", "a"]
        );
    }

    #[test]
    fn test_bfs_post_iterator() {
        let g = sh::SimpleGraphBuilder::parse("iter", "a-b\na-c\nb-d\nc-e\nd-f\ne-g", "-").unwrap();

        let dfs_iter = BfsPostIter::new(&g, "a");
        assert_eq!(
            dfs_iter.collect::<Vec<_>>(),
            vec!["g", "f", "e", "d", "c", "b", "a"]
        );
    }

    #[test]
    fn test_bfs_iterator() {
        let g =
            sh::SimpleGraphBuilder::parse("iter", "a-b\na-c\nb-d\nc-e\nd-f\ne-g\nf-h\ng-h", "-")
                .unwrap();

        let bfs_iter = BfsIter::new(&g, "a");
        assert_eq!(
            bfs_iter.collect::<Vec<_>>(),
            vec![
                ("a", 0),
                ("b", 1),
                ("c", 1),
                ("d", 2),
                ("e", 2),
                ("f", 3),
                ("g", 3),
                ("h", 4)
            ]
        );
    }
}
