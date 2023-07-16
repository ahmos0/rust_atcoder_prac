use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mut start = (0, 0);
    let mut reached = vec![vec![false; w]; h];

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 's' {
                start = (i, j);
                break;
            }
        }
    }

    let judge = search(start.0, start.1, h, w, &c, &mut reached);

    if judge {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn search(x: usize, y: usize, h: usize, w: usize, c: &Vec<Vec<char>>, reached: &mut Vec<Vec<bool>>) -> bool {
    if c[x][y] == 'g' {
        return true;
    }

    if x >= h || y >= w || c[x][y] == '#' || reached[x][y] {
        return false;
    }

    reached[x][y] = true;

    let dx = [1, -1, 0, 0];
    let dy = [0, 0, 1, -1];

    for i in 0..4 {
        let nx = x as i32 + dx[i];
        let ny = y as i32 + dy[i];

        if nx >= 0 && nx < h as i32 && ny >= 0 && ny < w as i32 {
            let nx = nx as usize;
            let ny = ny as usize;

            if search(nx, ny, h, w, c, reached) {
                return true;
            }
        }
    }

    false
}



/*use proconio::{input, marker::Chars};
use std::convert::TryInto;
fn main() {
    input! {
        h : usize,
        w : usize,
        c: [Chars; h]
    }
    let mut start: (i64, i64) = (0, 0);
    let mut start_flag = false;
    let mut reached: Vec<Vec<bool>> = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 's' {
                start_flag = true;
                start = (i as i64, j as i64);
                break;
            }
        }
        if start_flag == true {
            break;
        }
    }
    let judge = search(start.0.try_into().unwrap(), start.1.try_into().unwrap(), h, w, &c, & mut reached, false);
    if judge {
        println!("Yes");
    } else {
        println!("No");
    }
}


fn search(x: usize, y: usize, h: usize, w: usize, c: &Vec<Vec<char>>, reached: &mut Vec<Vec<bool>>, yn: bool) -> bool {
    if yn {
        return true;
    }
    if c[x][y] == 'g' {
        return true;
    }
    if x >= h || y >= w || c[x][y] == '#' || reached[x][y] {
        return false;
    }
    reached[x][y] = true;
    let mut result = false;
    if x + 1 < h {
        result = result || search(x + 1, y, h, w, c, reached, yn);
    }
    if x > 0 {
        result = result || search(x - 1, y, h, w, c, reached, yn);
    }
    if y + 1 < w {
        result = result || search(x, y + 1, h, w, c, reached, yn);
    }
    if y > 0 {
        result = result || search(x, y - 1, h, w, c, reached, yn);
    }
    reached[x][y] = false; 
    result
}*/