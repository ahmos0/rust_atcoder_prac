use proconio::input;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn dfs(
    field: &mut Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
    h: usize,
    w: usize,
) -> i64 {
    if field[x][y] == 'G' {
        return 0;
    }
    
    visited[x][y] = true;
    let mut min_moves = i64::MAX;

    for &(dx, dy) in &DIRECTIONS {
        let new_x = (x as i32 + dx) as usize;
        let new_y = (y as i32 + dy) as usize;
        
        if new_x < h && new_y < w && !visited[new_x][new_y] && field[new_x][new_y] != '#' {
            let moves = if field[new_x][new_y] == '.' {
                1
            } else {
                // Handle a person's line of sight
                let mut line_of_sight = false;
                let mut cur_x = x as i32;
                let mut cur_y = y as i32;

                while cur_x >= 0 && cur_x < h as i32 && cur_y >= 0 && cur_y < w as i32 {
                    if cur_x as usize == new_x && cur_y as usize == new_y {
                        break;
                    }
                    if field[cur_x as usize][cur_y as usize] == '#' || field[cur_x as usize][cur_y as usize] == '>' ||
                        field[cur_x as usize][cur_y as usize] == 'v' || field[cur_x as usize][cur_y as usize] == '<' ||
                        field[cur_x as usize][cur_y as usize] == '^' {
                        line_of_sight = true;
                        break;
                    }
                    cur_x += dx;
                    cur_y += dy;
                }

                if !line_of_sight {
                    2 // Need an extra step to move out of the line of sight
                } else {
                    0
                }
            };
            let next_moves = dfs(field, visited, new_x, new_y, h, w);
            if moves != i64::MAX && next_moves != i64::MAX {
                min_moves = min_moves.min(moves + next_moves);
            }
        }
    }
    
    visited[x][y] = false;
    min_moves
}

fn main() {
    input! {
        h: usize,
        w: usize,
        field: [String; h],
    }

    let mut start_x = 0;
    let mut start_y = 0;
    let mut goal_x = 0;
    let mut goal_y = 0;
    let mut grid = Vec::new();

    for (i, row) in field.iter().enumerate() {
        let mut temp_row = Vec::new();
        for (j, c) in row.chars().enumerate() {
            temp_row.push(c);
            if c == 'S' {
                start_x = i;
                start_y = j;
            } else if c == 'G' {
                goal_x = i;
                goal_y = j;
            }
        }
        grid.push(temp_row);
    }

    let mut visited = vec![vec![false; w]; h];
    let result = dfs(&mut grid, &mut visited, start_x, start_y, h, w);
    if result == i64::MAX {
        println!("-1"); // Goal is unreachable
    } else {
        println!("{}", result);
    }
}
