use std::vec;

use proconio::input;


type Graph = Vec<Vec<usize>>;
fn main(){
    input! {
        v: usize,
        e: usize,
        s: usize,
        t: usize,
        edges: [(usize, usize); e]
    }
    let mut graph: Graph = vec![Vec::new(); v];
    for (a, b) in edges {
        graph[a].push(b);
    }
    let mut seen: Vec<bool> = vec![false; v];
    dfs(&graph, &mut seen, s);
    if seen[t] {
        println!("yes");
    } else {
        println!("no");
    }
}

fn dfs(graph: &Graph, seen: &mut Vec<bool>, v: usize) {
    seen[v] = true;
    for &next in &graph[v] {
        if !seen[next] {
            dfs(graph, seen, next);
        }
    }
}