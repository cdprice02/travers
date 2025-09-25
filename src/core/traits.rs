pub trait Graph {
    type NodeIndex: Copy + Eq + std::hash::Hash;
    type EdgeWeight;

    fn neighbors(&self, node: Self::NodeIndex) -> Box<dyn Iterator<Item = Self::NodeIndex> + '_>;
    fn add_edge(&mut self, from: Self::NodeIndex, to: Self::NodeIndex, weight: Self::EdgeWeight);
}
