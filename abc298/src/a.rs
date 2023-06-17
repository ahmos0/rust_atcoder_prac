use proconio::input;

fn main() {
    input! {
        n: usize,
        mut  s: String,
    }
    let result: Vec<char> = s.chars().collect::<Vec<char>>();
    let mut flag = false;
    for i in 0..s.len(){
        if result[i] == 'o'{
            flag = true;
        }else if result[i]== 'x'{
            flag = false;
            break;
        }
    }
    if flag == true{
        println!("Yes");
    } else if flag == false{
        println!("No");
    }
}
