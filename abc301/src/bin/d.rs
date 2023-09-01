use proconio::input;

fn main() {
    input! {
        s: String,
        n: usize,
    }

    // Nを2進数の文字列に変換
    let n_binary = format!("{:b}", n);

    // ? の個数を計算
    let question_count = s.chars().filter(|&c| c == '?').count();

    // すべての可能な組み合わせを試す
    let mut max_value = -1;

    // ? の個数分のビットを用いて組み合わせを生成
    for i in 0..(1 << question_count) {
        let mut s_chars = s.chars().collect::<Vec<char>>();
        let mut n_idx = 0;

        // ? を 1 または 0 に置き換える
        for j in 0..s_chars.len() {
            if s_chars[j] == '?' {
                let digit = (i >> n_idx) & 1;
                s_chars[j] = std::char::from_digit(digit, 10).unwrap();
                n_idx += 1;
            }
        }

        // 置き換えた値を10進数の整数に変換し、N以下で最大の値を更新
        let value = s_chars.iter().collect::<String>().parse::<i32>().unwrap();
        if value <= n as i32&& value > max_value {
            max_value = value;
        }
    }

    println!("{}", max_value);
}
