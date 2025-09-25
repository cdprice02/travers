
use travers::prelude::*;

#[test]
fn digraph_add_nodes_edges() {
    let mut g = DiGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b, ());

    let neighbors: Vec<_> = g.neighbors(a).collect();
    assert_eq!(neighbors, vec![b]);
}

#[test]
fn ungraph_symmetric_edges() {
    let mut g = UnGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b, ());

    assert_eq!(g.neighbors(a).collect::<Vec<_>>(), vec![b]);
    assert_eq!(g.neighbors(b).collect::<Vec<_>>(), vec![a]);
}
