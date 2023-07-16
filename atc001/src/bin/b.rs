use proconio::input;
use petgraph::unionfind::{UnionFind, self};
fn main() {
    input!{
        n:usize,
        q:usize,
    }
    let mut uf = UnionFind::<usize>::new(n);
    let mut pa :Vec<usize> = Vec::new();
    let mut aa :Vec<usize> = Vec::new();
    let mut ba :Vec<usize> = Vec::new();
    for i in 0..q{
        input! {
            p:usize,
            a:usize,
            b:usize,
        }
        pa.push(p);
        aa.push(a);
        ba.push(b);
        if p == 0{
            uf.union(a, b);
        } else if p == 1{
            let flag: bool = uf.equiv(a, b);
            if flag == true{
                println!("Yes");
            } else if flag == false{
                println!("No");
            }
        }

    }
    
}
