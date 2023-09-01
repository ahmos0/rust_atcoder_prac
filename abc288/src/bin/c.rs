use proconio::{input, marker::Usize1};
use petgraph::unionfind::UnionFind;
fn main(){
    input! {
        n: usize,
        m: usize,
        vp: [(Usize1, Usize1); m],
    }
    let mut uf=  UnionFind::new(n);
    let mut ans = 0;
    for (a, b) in vp {
        if uf.equiv(a, b) {
            ans += 1;
        } else {
            uf.union(a, b);
        }
    }
    println!("{}", ans);
}