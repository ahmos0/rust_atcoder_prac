use proconio::input;

fn main(){
    input!{
        mut s : String,
    }
    let words:Vec<String> = vec!["dream", "dreamer", "erase", "eraser"]
        .iter()
        .map(|&s| s.to_string())
        .map(|s| s.chars().rev().collect::<String>())
        .collect();
    //reverse
    let mut equal = true;
    s = s.chars().rev().collect::<String>();
    let mut flag = 1;
    while !s.is_empty() {
        if let Some(word) = words.iter().find(|&p| s.starts_with(p)) {
            s = s.replacen(word, "", 1);
        } else {
            equal = false;
            break;
        }
    }
    if equal == true{
        println!("YES");
    } else if equal == false {
        println!("NO");
    }
}