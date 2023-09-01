use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        a: [Usize1; n], 
    }
    let mut hs = HashSet::new(); //Hashsetは重複しない
    let mut ans:Vec<usize> = Vec::new();
    let mut element = 0;
    hs.insert(0);
    let mut first = a[0];
    // hashsetに含まれていれば、サイクルができたということ
    while !hs.contains(&first){
        hs.insert(first);
        first = a[first]; 
    }
    element = first;
    ans.push(first+1);
    while a[element] != first{
        element = a[element];
        ans.push(element+1);
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
