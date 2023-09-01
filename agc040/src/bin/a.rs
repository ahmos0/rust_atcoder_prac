use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut s_c: Vec<char> = s.chars().collect();
    let mut sum = 0;
    let mut scount = 0;
    let mut dcount = 0;
    let mut sflag = false;
    let mut dflag = false;
    for i in 0..s_c.len(){
        if i == 0 {
            if s_c[i] == '<'{
                sflag = true;
                scount += 1;
                sum+=1;
            } else if s_c[i] == '>'{
                dflag = true;
                dcount += 1;
                sum += 1;
            }
        } else {
            if s_c[i] == '<' && sflag{
                scount += 1;
                sum += scount;
                //println!("b {}",sum);
            } else if s_c[i] == '>' && dflag{
                dcount += 1;
                sum += dcount;
                //println!("c {}",sum);
            } else if s_c[i] == '<' && !sflag{
                scount = dcount+1;
                dcount = 0;
                sflag = true;
                dflag = false;
                //println!("d {}",sum);
            } else if s_c[i] == '>' && !dflag{
                dcount = 1;
                scount = 0;
                sflag = false;
                dflag = true;
                //println!("d {}",sum);
            }
        }
    }
    println!("{}", sum);
}
