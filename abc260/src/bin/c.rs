use proconio::input;

fn change_gems(level: i64, x: i64, y: i64, red: bool) -> i64 {
    if level == 1 {
        if red {
            0
        } else {
            1
        }
    } else if red{
        change_gems(level-1, x, y, true) + change_gems(level, x, y, false) * x 
    } else {
        change_gems(level-1, x, y, true) + change_gems(level-1, x, y, false) * y 
    }
}

fn main() {
    input! {
        n: i64,
        x: i64,
        y: i64,
    }

    let result = change_gems(n, x, y, true);
    println!("{}", result);
}
