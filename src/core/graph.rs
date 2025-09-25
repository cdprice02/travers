use super::traits::Graph;

pub struct DiGraph<W = ()> {
    adj: Vec<Vec<(usize, W)>>,
}

impl DiGraph {
    pub fn new() -> Self {
        Self { adj: Vec::new() }
    }
}

impl<W: Copy> Graph for DiGraph<W> {
    type NodeIndex = usize;
    type EdgeWeight = W;

    fn neighbors(&self, node: Self::NodeIndex) -> Box<dyn Iterator<Item = Self::NodeIndex> + '_> {
        Box::new(self.adj[node].iter().map(|(n, _)| *n))
    }

    fn add_edge(&mut self, from: Self::NodeIndex, to: Self::NodeIndex, weight: Self::EdgeWeight) {
        self.adj[from].push((to, weight))
    }
}

pub struct UnGraph<W = ()> {
    adj: Vec<Vec<(usize, W)>>,
}

impl UnGraph {
    pub fn new() -> Self {
        Self { adj: Vec::new() }
    }
}

impl<W: Copy> Graph for UnGraph<W> {
    type NodeIndex = usize;
    type EdgeWeight = W;

    fn neighbors(&self, node: Self::NodeIndex) -> Box<dyn Iterator<Item = Self::NodeIndex> + '_> {
        Box::new(self.adj[node].iter().map(|(n, _)| *n))
    }

    fn add_edge(&mut self, a: Self::NodeIndex, b: Self::NodeIndex, weight: Self::EdgeWeight) {
        self.adj[a].push((b, weight));
        self.adj[b].push((a, weight));
    }
}
