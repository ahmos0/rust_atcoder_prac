/*use proconio::input;

fn main(){
    let mut a = vec!["a".to_string(); 100];
    let mut tmp = 1000000000;
    let mut li = 0;
    input! {
        n :usize,
    }
    for i in 0..n{
        input!{
            b : String,
            num : i32
        }
        a[i] = b;
        if num < tmp {
            tmp = num;
            li = i;
        }
    }
    for i in li..n{
        println!("{}",a[i])
    }
    if li != 0{
        for i in 0..li{
            println!("{}",a[i]);
        }
    }

}*//* */

/*use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        matrix: [Chars; n],
    }

    let mut new_matrix = vec![vec!['\0'; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == 0 &&(j == 0 || j == n - 1){

            }
        }
    }

    for row in new_matrix {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}*/


/*use petgraph::graph::{NodeIndex, UnGraph};
use proconio::input;

fn dfs(
    g: &UnGraph<(), ()>,
    node: NodeIndex,
    colors: &mut [usize],
    start: NodeIndex,
    visited: &mut [bool],
    prev_node: Option<NodeIndex>,
) -> bool {
    visited[node.index()] = true;

    if colors[node.index()] == 0 {
        colors[node.index()] = 1;
    } else {
        colors[node.index()] = 0;
    }

    if node == start && colors[node.index()] != 0 {
        return true;
    }

    let mut reachable_nodes: Vec<usize> = Vec::new();

    for neighbor in g.neighbors(node) {
        let neighbor_color = colors[neighbor.index()];
        if neighbor_color != colors[node.index()] {
            reachable_nodes.push(neighbor.index());
            if let Some(prev) = prev_node {
                if prev == neighbor {
                    continue;
                }
            }
            if !visited[neighbor.index()] && dfs(g, neighbor, colors, start, visited, Some(node)) {
                return true;
            }
        }
    }

    if colors[node.index()] == 0 {
        colors[node.index()] = 1;
    } else {
        colors[node.index()] = 0;
    }

    println!("From node {}: Reachable nodes: {:?}", node.index() + 1, reachable_nodes);

    false
}

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
        g.add_edge(nodes[b], nodes[a], ());
    }

    let mut colors = c.clone();
    let mut visited = vec![false; n];

    for i in 0..n {
        let start = nodes[i];
        dfs(&g, start, &mut colors, start, &mut visited, None);

        // Reset colors and visited for the next iteration
        colors.clone_from_slice(&c);
        visited.iter_mut().for_each(|v| *v = false);
    }
}*/
