pub mod algorithms;
pub mod simple;

pub trait Node<'a, Id: Eq + 'a, Weight: 'a> {
    type Edge: Edge<Id, Weight> + 'a;

    fn id(&self) -> Id;
    fn edges(&'a self) -> impl Iterator<Item = &'a Self::Edge>;

    fn is_adjacent(&'a self, node_id: Id) -> bool {
        self.edges().any(|e| e.b() == node_id)
    }

    fn degree(&'a self) -> usize {
        self.edges().count()
    }

    fn neighbours(&'a self) -> impl Iterator<Item = Id> + 'a {
        self.edges().map(move |e| e.b())
    }
}

pub trait Edge<Id, Weight> {
    fn weight(&self) -> Weight;
    fn a(&self) -> Id;
    fn b(&self) -> Id;
}

pub trait Graph<'a, Id: Eq + 'a, Weight: 'a> {
    type Node: Node<'a, Id, Weight> + 'a;

    fn node(&self, id: &Id) -> Option<&Self::Node>;
    fn nodes(&'a self) -> impl Iterator<Item = &'a Self::Node>;
    fn edges(&'a self) -> impl Iterator<Item = &'a <Self::Node as Node<'a, Id, Weight>>::Edge> {
        self.nodes().flat_map(|n| n.edges())
    }
}

pub trait Builder<'a, Id: Clone + Eq + 'a, Weight: Clone + 'a> {
    type Graph: Graph<'a, Id, Weight>;

    fn add_node(&mut self, id: Id) -> &mut <Self::Graph as Graph<'a, Id, Weight>>::Node;
    fn add_node_edge(&mut self, a: Id, b: Id, weight: Weight);
    fn build(self) -> Self::Graph;

    fn add_directed_edge(&mut self, a: Id, b: Id, weight: Weight) {
        self.add_node_edge(a.clone(), b.clone(), weight.clone());
        self.add_node(b);
    }

    fn add_edge(&mut self, a: Id, b: Id, weight: Weight) {
        self.add_node_edge(a.clone(), b.clone(), weight.clone());
        self.add_node_edge(b, a, weight);
    }
}
