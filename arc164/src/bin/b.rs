use petgraph::graph::{NodeIndex, UnGraph};
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        v: [(usize, usize); m],
        c: [usize; n]
    }

    let edges: Vec<(usize, usize)> = v.iter().map(|&(a, b)| (a - 1, b - 1)).collect();

    let mut g = UnGraph::<(), ()>::new_undirected();
    let nodes: Vec<NodeIndex> = (0..n).map(|_| g.add_node(())).collect();
    for &(a, b) in &edges {
        g.add_edge(nodes[a], nodes[b], ());
    }

    for &(a, b) in &v {
        let node_a = nodes[a - 1];
        let node_b = nodes[b - 1];
        if g.find_edge(node_a, node_b).is_some() {
            println!("Edge ({}, {}) exists", a, b);
        } else {
            println!("Edge ({}, {}) does not exist", a, b);
        }
    }
}
