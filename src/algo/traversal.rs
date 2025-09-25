use crate::core::traits::Graph;
use std::collections::VecDeque;

pub struct Bfs<'a, G: Graph> {
    graph: &'a G,
    queue: VecDeque<G::NodeIndex>,
    visited: std::collections::HashSet<G::NodeIndex>,
}

impl<'a, G: Graph> Bfs<'a, G> {
    pub fn new(graph: &'a G, start: G::NodeIndex) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        Self {
            graph,
            queue,
            visited: std::collections::HashSet::new(),
        }
    }
}

impl<'a, G: Graph> Iterator for Bfs<'a, G> {
    type Item = G::NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.queue.pop_front() {
            if self.visited.insert(node) {
                for n in self.graph.neighbors(node) {
                    self.queue.push_back(n);
                }
                return Some(node);
            }
        }
        None
    }
}
