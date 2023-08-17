use proconio::input;
fn main() {
    input! {
        n:usize,
        m:usize,
        mut uv:[(usize, usize);m]
    }
    let mut visited: [bool; 100] = [false; 100];
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    let mut u = 0;
    let mut v = 0;
    let mut ans = 0;
    for i in 0..m {
        u = uv[i].0 - 1;
        v = uv[i].1 - 1;
        g[u].push(v);
        g[v].push(u);
    }
    for i in 0..n {
        if !visited[i] {
            ans += 1;
            visited = dfs(i, visited, &g);
        }
    }
    println!("{}",ans);
}

fn dfs(c: usize, mut visit: [bool; 100], g: &Vec<Vec<usize>>) -> [bool; 100] {
    visit[c] = true;
    for &d in &g[c] {
        if !visit[d] {
            visit = dfs(d, visit, g); 
        }
    }
    visit
}