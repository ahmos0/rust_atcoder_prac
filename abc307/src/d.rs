//TLEする
use proconio::input;

fn main(){
    input! {
        n:usize,
        mut s:String
    }
    let mut l_num = 0;
    let mut r_num = 0;
    let mut l_flag = false;
    let mut r_flag = false;
    let mut modified_s = s.clone();
    loop {
        let mut i = 0;
        l_flag = false;
        r_flag = false;
        for c in modified_s.chars() {
            if c == '(' {
                l_num = i;
                l_flag = true;
            } else if c == ')' && l_flag  && r_flag != true {
                r_num = i;
                r_flag = true;
            }
            if l_flag  && r_flag  {
                modified_s.replace_range(l_num..=r_num, "");
                break;
            }
            i += 1;
        }
        if !(l_flag && r_flag) {
            break;
        }
    }
    println!("{}", modified_s);
}

//boolの配列を持っておいて、falseで初期化して、l_num..=r_numをtrueに置き換える。
//falseのものだけ順に追加していく。->これが答え。
