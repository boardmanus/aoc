use priority_queue::PriorityQueue;
use rustworkx_core::{
    connectivity::stoer_wagner_min_cut,
    petgraph::{graph::NodeIndex, graph::UnGraph},
    Result,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{self, Display, Formatter},
};

type Node = usize;
type Edge = usize;
type Edges = HashMap<Node, Edge>;
type Nodes = HashMap<Node, Edges>;

#[derive(Debug, Clone, Copy)]
struct MinCut {
    s: Node,
    t: Node,
    w: usize,
}

impl MinCut {
    fn new(s: Node, t: Node, w: usize) -> Self {
        MinCut { s, t, w }
    }

    fn default() -> Self {
        MinCut {
            s: Node::default(),
            t: Node::default(),
            w: usize::MAX,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Graph<'a> {
    node_map: HashMap<&'a str, Node>,
    nodes: Nodes,
}

impl<'a> Display for Graph<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let nodes = self.nodes.keys().collect::<Vec<_>>();
        let num_edges = self.nodes.values().map(|edges| edges.len()).sum::<usize>() / 2;
        write!(
            f,
            "Graph[num_nodes={}, num_edges={num_edges}]: {nodes:?}",
            nodes.len()
        )?;
        Ok(())
    }
}

impl<'a> Graph<'a> {
    fn from_str(input: &'a str) -> Result<Self, ()> {
        let mut graph = Graph::default();
        for line in input.lines() {
            let mut parts = line.split(": ");
            let a_name: &str = parts.next().ok_or(())?;
            let rest = parts.next().ok_or(())?.split_whitespace();

            let a = graph.add_node(a_name);
            rest.for_each(|b_name| {
                let b = graph.add_node(b_name);
                graph.add_edge(a, b, 1);
            });
        }
        Ok(graph)
    }

    fn node(&self, node_name: &'a str) -> Option<Node> {
        self.node_map.get(node_name).copied()
    }

    fn add_node(&mut self, node_name: &'a str) -> Node {
        *self.node_map.entry(node_name).or_insert_with(|| {
            let node = self.nodes.len();
            self.nodes.insert(node, HashMap::new());
            node
        })
    }

    fn add_edge(&mut self, a: Node, b: Node, weight: usize) {
        self.nodes.entry(a).or_default().insert(b, weight);
        self.nodes.entry(b).or_default().insert(a, weight);
    }

    fn update_edge(&mut self, a: Node, b: Node, weight: usize) {
        *self.nodes.entry(a).or_default().entry(b).or_default() += weight;
        *self.nodes.entry(b).or_default().entry(a).or_default() += weight;
    }

    fn nodes(&self) -> impl Iterator<Item = (&Node, &Edges)> + '_ {
        self.nodes.iter()
    }

    fn edges(&self, node: Node) -> impl Iterator<Item = (&Node, &Edge)> + '_ {
        self.nodes[&node].iter()
    }

    fn remove(&mut self, t: Node) {
        self.nodes.remove(&t);
        self.nodes
            .iter_mut()
            .for_each(|(_, edges)| _ = edges.remove(&t));
    }

    fn collapse(&mut self, s: Node, t: Node) {
        // Move all edges from t to s.
        let edges = self.edges(t).map(|(&a, &b)| (a, b)).collect::<Vec<_>>();
        for (r, weight) in edges {
            // Add/update edge from r to s
            self.update_edge(r, s, weight);
        }

        // Remove t from the graph
        self.remove(t);
    }

    fn reachable_nodes(&self, start: Node) -> Vec<Node> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([start]);
        while let Some(node) = queue.pop_front() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            queue.extend(self.nodes[&node].keys());
        }
        visited.into_iter().collect()
    }

    //
    // function: MinCutPhase(Graph G, Weights W, Vertex a):
    //     A <- {a}
    //     while A != V:
    //         add tightly connected vertex to A
    //     store cut_of_the_phase and shrink G by merging the two vertices added last

    // minimum = INF
    // function: MinCut(Graph G, Weights W, Vertex a):
    //     while |V| > 1:
    //         MinCutPhase(G,W,a)
    //         if cut_of_the_phase < minimum:
    //             minimum = cut_of_the_phase
    //     return minimum
    fn min_cut_phase(&self) -> MinCut {
        // Add all nodes to the priority queue, with a default weight.
        let mut pq: PriorityQueue<Node, usize> = PriorityQueue::with_capacity(self.nodes.len());
        for (&node, _) in self.nodes() {
            pq.push(node, 0);
        }
        //self.nodes.keys().for_each(|&node| _ = pq.push(node, 0));

        // The nodes that are currently being tracked.
        let mut cut_of_phase = MinCut::default();

        while let Some((node, weight)) = pq.pop() {
            // Update current nodes being tracked.
            cut_of_phase = MinCut::new(cut_of_phase.t, node, weight);

            // Update the priority queue such that all nodes connected to the current
            // have the weight of the edge added to the current priority. This will result
            // in the most connected node having the highest priority.
            for (node_b, &weight_b) in self.edges(node) {
                pq.change_priority_by(node_b, |cur| *cur += weight_b);
            }
        }

        cut_of_phase
    }

    fn min_cut(&'a self) -> Vec<Node> {
        // Clone the graph for mutation
        let mut graph = self.clone();
        let mut min_cut = MinCut::default();
        let mut min_cut_phase = 0;
        let mut cuts = Vec::new();

        for phase in 0..graph.nodes.len() - 1 {
            let cut = graph.min_cut_phase();

            graph.collapse(cut.s, cut.t);

            cuts.push(cut);
            if min_cut.w > cut.w {
                min_cut = cut;
                min_cut_phase = phase;
            }
        }

        // Contracted graph
        let mut cut_graph = Graph::default();
        cuts.iter()
            .take(min_cut_phase)
            .for_each(|cut| cut_graph.add_edge(cut.s, cut.t, 1));

        let reachable = cut_graph.reachable_nodes(min_cut.t);

        reachable
    }
}

fn petgraph_from_str<'a>(input: &'a str) -> UnGraph<&'a str, ()> {
    let mut graph = UnGraph::new_undirected();
    let mut nodes = HashMap::<&str, NodeIndex>::new();
    input.lines().for_each(|line| {
        let mut parts = line.split(": ");
        let a_name: &str = parts.next().unwrap();
        let rest = parts.next().unwrap().split_whitespace().collect::<Vec<_>>();

        let a = *nodes
            .entry(a_name)
            .or_insert_with(|| graph.add_node(a_name));
        for b_name in rest {
            let b = *nodes
                .entry(b_name)
                .or_insert_with(|| graph.add_node(b_name));
            graph.add_edge(a, b, ());
        }
    });
    graph
}

fn solve_part_petgraph(input: &str) -> usize {
    let graph = petgraph_from_str(input);
    let res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    if let Ok(Some(min_cut)) = res {
        let num_min_cut_nodes = min_cut.1.len();
        num_min_cut_nodes * (graph.node_count() - num_min_cut_nodes)
    } else {
        panic!("No min cut found!");
    }
}

fn solve_part1(input: &str) -> usize {
    let graph = Graph::from_str(input).unwrap();
    let partition = graph.min_cut();
    let num_min_cut_nodes = partition.len();
    num_min_cut_nodes * (graph.nodes.len() - num_min_cut_nodes)
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let now = std::time::Instant::now();
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1} ({:?})", now.elapsed());
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");
    const TEST_INPUT_2: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 54);
    }

    #[test]
    fn test_graph_from_str() {
        let graph = Graph::from_str(TEST_INPUT).unwrap();
        let n = |name| graph.node(name).unwrap();
        assert_eq!(
            graph.nodes[&n("jqt")],
            HashMap::from([(n("rhn"), 1), (n("xhk"), 1), (n("nvd"), 1), (n("ntq"), 1)])
        );
        assert_eq!(
            graph.nodes[&n("nvd")],
            HashMap::from([
                (n("jqt"), 1),
                (n("cmg"), 1),
                (n("pzl"), 1),
                (n("qnr"), 1),
                (n("lhk"), 1)
            ])
        );
        println!("{:?}", graph);
    }

    #[test]
    fn test_petgraph_from_str() {
        let graph = petgraph_from_str(TEST_INPUT);
        //assert_eq!(graph["jqt"], vec!["rhn", "xhk", "nvd", "ntq"]);
        //assert_eq!(graph["nvd"], vec!["jqt", "cmg", "pzl", "qnr", "lhk"]);
        println!("{:?}", graph);
    }
}
