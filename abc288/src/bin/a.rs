use proconio::input;

fn main() {
    input! {
        N:i64,
    };
    for i in 0..N {
        for j in 0..1 {
            input! {
                a :i128,
                b :i128,
            };
            println!("{}", a + b);
        }
    }
}
