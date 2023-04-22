//解けてない
use proconio::input;

fn main() {
    input! {
        n : i64,
    }

    let mut tmp_nine = 1;
    let mut tmp_six = 1;
    let mut a = n;
    let mut total_count = 0;
    while a >= 6 {
        loop {
            tmp_six = tmp_six * 6;
            if tmp_six > a {
                tmp_six = tmp_six / 6;
                println!("tmp_six {}", tmp_six);
                break;
            }
        }
        loop {
            tmp_nine = tmp_nine * 9;
            if tmp_nine > a {
                tmp_nine = tmp_nine / 9;
                println!("tmp_nine {}", tmp_nine);
                break;
            }
        }
        if tmp_nine > tmp_six {
            a = a - tmp_nine;
            println!("nine a = {}",a);
            total_count += 1;
        } else {
            a = a - tmp_six;
            println!("six a = {}",a);
            total_count += 1;
        }
        println!("a {}", a);
        tmp_nine = 1;
        tmp_six = 1;
    }
    total_count = total_count + a;
    println!("{}", total_count);
}
