use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut s_c:Vec<char> = s.chars().collect();
    let mut tmp = 0;
    let mut max = 0;
    for i in 0..s_c.len()-tmp{
        println!();
        if ((s_c.len()-tmp) % 2) != 0 || i == 0{
            tmp+=1;
            continue;
        } else {
            let mut s_r:Vec<char> = Vec::new();
            let mut s_l:Vec<char> = Vec::new();
            for j in 0..(s_c.len()-tmp)/2{
                s_r.push(s_c[j]);
                
            }
            for j in (s_c.len()-tmp)/2..=s_c.len()-tmp-1{
                s_l.push(s_c[j]);
            }  
            for j in 0..(s_c.len()-tmp)/2{
                if s_r[j] != s_l[j]{
                    break;
                } else if j == ((s_c.len()-tmp)/2) -1{
                    if max < s_c.len()-tmp{
                        max = s_c.len()-tmp;
                    }
                }
            }
            tmp += 1;
        }
    }
    println!("{}",max);
}
