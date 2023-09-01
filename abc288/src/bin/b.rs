use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n]
    }
    let mut ans:Vec<String> = Vec::new();
    for i in 0..k{
        ans.push(s[i].clone());
    }
    ans.sort();
    for i in 0..ans.len(){
        println!("{}", ans[i]);
    }
}
