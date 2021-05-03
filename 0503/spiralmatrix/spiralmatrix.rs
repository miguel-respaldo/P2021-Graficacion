fn main() {
    let m = spiral_matrix(4);
    println!("Spiral Matrix:\n{}", print_matrix(&m));
}

pub fn spiral_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut res = vec![vec![0; n as usize]; n as usize];
    if n < 1 {
        return res;
    }
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (0, n as usize, 0, n as usize);
    let mut i = 1;
    loop {
        for y in y_min..y_max {
            res[x_min][y] = i;
            i += 1;
        }
        x_min += 1;
        if x_min == x_max {
            break;
        }
        for x in x_min..x_max {
            res[x][y_max - 1] = i;
            i += 1;
        }
        y_max -= 1;
        if y_min == y_max {
            break;
        }
        for y in (y_min..y_max).rev() {
            res[x_max - 1][y] = i;
            i += 1;
        }
        x_max -= 1;
        if x_min == x_max {
            break;
        }
        for x in (x_min..x_max).rev() {
            res[x][y_min] = i;
            i += 1;
        }
        y_min += 1;
        if y_min == y_max {
            break;
        }
    }
    res
}

fn print_matrix(m: &Vec<Vec<i32>>) -> String {
    m.iter().fold("".to_string(), |a, r| {
        a + &r
            .iter()
            .fold("".to_string(), |b, e| b + "\t" + &e.to_string())
            + "\n"
    })
}
