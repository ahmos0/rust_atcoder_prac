use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, // 頂点数
        m: usize, // 辺の数
        edges: [(Usize1, Usize1); m], // 辺の情報 (u, v)
    }

    // UnionFindの初期化
    let mut uf = UnionFind::new(n);

    // 辺をUnionFindに追加
    for &(u, v) in &edges {
        // 頂点が既に同じ集合に属している場合、ループが存在すると判定
        if !uf.union(u, v) {
            println!("No");
            return;
        }
    }

    // 各頂点の次数を数える
    let mut degrees = vec![0; n];
    for &(u, v) in &edges {
        degrees[u] += 1;
        degrees[v] += 1;
    }

    // 判定
    let mut path_vertex_count = 0;
    for i in 0..n {
        match degrees[i] {
            0 => {}
            1 => path_vertex_count += 1,
            2 => {}
            _ => {
                println!("No");
                return;
            }
        }
    }

    if path_vertex_count == 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
