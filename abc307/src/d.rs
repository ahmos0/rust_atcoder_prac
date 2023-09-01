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
    let mut tbool: Vec<bool> = vec![false; n];
    let mut modified_s:Vec<char> = Vec::new();
    let mut s_c:Vec<char> = s.chars().collect();
    let mut i = 0;
    while i != s_c.len() -1{
        i = 0;
        l_flag = false;
        r_flag = false;
        for c in s_c.clone() {
            if c == '(' {
                l_num = i;
                l_flag = true;
            } else if c == ')' && l_flag  && !r_flag  {
                r_num = i;
                r_flag = true;
            }
            if l_flag  && r_flag  {
                break;
            }
            i += 1;
        }
        for k in l_num..r_num {
            tbool[k] = true;
        }
        if !(l_flag && r_flag) {
            break;
        }
    }
    for i in 0..s_c.len(){
        if tbool[i] {
            modified_s.push(s_c[i]);
        }
    }
    for i in 0..modified_s.len(){
        print!("{}",modified_s[i]);
    }
    println!("");
}

//boolの配列を持っておいて、falseで初期化して、l_num..=r_numをtrueに置き換える。
//falseのものだけ順に追加していく。->これが答え。
