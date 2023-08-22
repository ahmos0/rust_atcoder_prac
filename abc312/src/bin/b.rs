use proconio::input;

fn check_pattern(s: &[Vec<char>], i: usize, j: usize) -> bool {
    let b = '#';
    let w = '.';
    
    (s[i][j] == b
        && s[i][j + 1] == b
        && s[i][j + 2] == b
        && s[i + 1][j] == b
        && s[i][j + 1] == b
        && s[i][j + 2] == b
        && s[i + 2][j] == b
        && s[i][j + 1] == b
        && s[i][j + 2] == b)
        && (s[i + 3][j] == w
            && s[i + 3][j + 1] == w
            && s[i + 3][j + 2] == w
            && s[i + 3][j + 3] == w
            && s[i][j + 3] == w
            && s[i + 1][j + 3] == w
            && s[i + 2][j + 3] == w)
        && (s[i + 6][j + 6] == b
            && s[i + 6][j + 7] == b
            && s[i + 6][j + 8] == b
            && s[i + 7][j + 6] == b
            && s[i + 7][j + 7] == b
            && s[i + 7][j + 8] == b
            && s[i + 8][j + 6] == b
            && s[i + 8][j + 7] == b
            && s[i + 8][j + 8] == b)
        && (s[i + 5][j + 5] == w
            && s[i + 5][j + 6] == w
            && s[i + 5][j + 7] == w
            && s[i + 5][j + 8] == w
            && s[i + 6][j + 5] == w
            && s[i + 7][j + 5] == w
            && s[i + 8][j + 5] == w)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s_s: [String; n]
    }
    let mut s: Vec<Vec<char>> = vec![Vec::new(); n];
    for i in 0..n {
        s[i] = s_s[i].chars().collect();
    }

    for i in 0..n {
        for j in 0..m {
            if i + 8 <= n && j + 8 <= m && check_pattern(&s, i, j) {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}
