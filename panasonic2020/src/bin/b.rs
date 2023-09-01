use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize
    }
    if h == 1 || w == 1{
        println!("1");
    }
    else if (h * w) % 2 == 0{
        println!("{}", h*w/2);
    } else {
        println!("{}", h*w/2 + 1);
    }

}
