use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut tmp = 0;
    let mut ans: Vec<i128> = Vec::new();
    for i in 0..n {
        input! {
            a: i128
        }
        if i == 0 {
            ans.push(a);
            tmp = a;
        } else if (a - tmp).abs() == 1 {
            ans.push(a);
            tmp = a;
        } else {
            while (a - tmp).abs() != 1 {
                if a - tmp > 0 {
                    ans.push(tmp + 1);
                    tmp += 1;
                } else {
                    ans.push(tmp - 1);
                    tmp -= 1;
                }
            }
            ans.push(a);
            tmp = a;
        }
    }
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
}
