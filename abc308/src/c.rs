use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    }
    let mut map: HashMap<usize, f64> = HashMap::new();
    let mut arr: Vec<f64> = vec![0.0; n];
    for i in 0..n {
        input! {
            a: f64,
            b: f64
        }
        arr[i] = a / (a + b);
        map.insert(i + 1, arr[i]);
    }
    let mut vec: Vec<(usize, f64)> = map.into_iter().collect();
    vec.sort_by(|&(k1, v1), &(k2, v2)| {
        if v1 == v2 {
            k1.cmp(&k2)
        } else {
            v2.partial_cmp(&v1).unwrap()
        }
    });
    for (key, _) in vec {
        println!("{}", key);
    }
}

