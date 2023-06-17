use proconio::input;

fn main(){
    input! {
        n : usize,
        s : String,
    }
    let mut judge_bar = false;
    let mut judge_as = false;
    let mut count = 0;
    let result: Vec<char> = s.chars().collect::<Vec<char>>();
    for i in 0..s.len(){
        if result[i] == '|'{
            count = count + 1;
            judge_bar = true;
            if count == 2 && judge_as == true{
                println!("in");
                return;
            }  else if count == 2 && judge_as == true{
                count = 1;
            }
        } else if result[i] == '*' && judge_bar == true{
            judge_as = true;
        } 
    }
    println!("out");
}