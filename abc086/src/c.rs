use proconio::input;

fn main(){
 
    input!{
        n: usize,
        mut txy:[(isize,isize,isize);n],
    } 
    let mut v = vec![(0, 0, 0)];
 
    v.append(&mut txy);
    let mut answer = true;
    for i in 0..n{
        let dt = v[i + 1].0 - v[i].0;
        let dist = (v[i+1].1 - v[i].1).abs() + (v[i+1].2 - v[i].2).abs();
        if dt < dist {
            answer = false;
        }
        if (dt % 2) != (dist % 2){
            answer = false;
        }
    }
    if answer == true{
        println!("Yes");
    } else if answer == false{
        println!("No");
    }
}