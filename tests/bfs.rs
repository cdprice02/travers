use travers::prelude::*;
use travers::algo::traversal::Bfs;

#[test]
fn bfs_traversal() {
    let mut g = DiGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let c = g.add_node();
    g.add_edge(a, b, ());
    g.add_edge(b, c, ());

    let order: Vec<_> = Bfs::new(&g, a).collect();
    assert_eq!(order, vec![a, b, c]);
}
