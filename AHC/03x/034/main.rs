use std::io::stdin;
use std::str::FromStr;
use std::cmp::min;

fn main() {
    let n: usize = get_one();
    let mut hs = get_hs(n);
    let mut dir = Direction::Right;
    let mut x = 0;
    let mut y = 0;
    let mut load = 0;
    loop {
        if hs[y][x] > 0 {
            let d = hs[y][x];
            load += d;
            hs[y][x] = 0;
            println!("+{}", d)
        } else if hs[y][x] < 0 && load > 0 {
            let p = hs[y][x];
            let d = min(load, -p);
            load -= d;
            hs[y][x] += d;
            println!("-{}", d);
        }
        match dir {
            Direction::Right => {
                if x == n-1 {
                    dir = Direction::Left;
                    y += 1;
                    println!("D");
                } else {
                    x += 1;
                    println!("R");
                }
            }
            Direction::Left => {
                if x == 0 {
                    if y == n-1 {
                        break;
                    }
                    dir = Direction::Right;
                    y += 1;
                    println!("D");
                } else {
                    x -= 1;
                    println!("L");
                }
            }
        }
    }
    dir = Direction::Right;
    while load > 0 {
        if hs[y][x] < 0 {
            let p = hs[y][x];
            let d = min(load, -p);
            load -= d;
            hs[y][x] += d;
            println!("-{}", d);
        }
        match dir {
            Direction::Right => {
                if x == n-1 {
                    dir = Direction::Left;
                    y -= 1;
                    println!("U");
                } else {
                    x += 1;
                    println!("R");
                }
            }
            Direction::Left => {
                if x == 0 {
                    if y == 0 {
                        break;
                    }
                    dir = Direction::Right;
                    y -= 1;
                    println!("U");
                } else {
                    x -= 1;
                    println!("L");
                }
            }
        }
    }
}

enum Direction {
    Left,
    Right,
}

fn get_hs(n: usize) -> Vec<Vec<i64>> {
    let mut hs = Vec::new();
    for _ in 0..n {
        hs.push(get_vec());
    }
    return hs;
}

#[allow(dead_code)]
fn get_line() -> String {
    let mut s = String::new();
    match stdin().read_line(&mut s){
        Ok(_) => {s.trim().to_string()}
        Err(_) => String::new()
    }
}

#[allow(dead_code)]
fn get_vec<T: std::str::FromStr>() -> Vec<T> {
    let line = get_line();
    line.split_whitespace().filter_map(|x| x.parse().ok()).collect()
}

#[allow(dead_code)]
fn get_one<T: FromStr + Copy>() -> T {
    let v = get_vec();
    v[0]
}

#[allow(dead_code)]
fn get_pair<T: FromStr + Copy>() -> (T, T) {
    let v = get_vec();
    (v[0], v[1])
}

#[allow(dead_code)]
fn get_triple<T: FromStr + Copy>() -> (T, T, T) {
    let v = get_vec();
    (v[0], v[1], v[2])
}

#[allow(dead_code)]
fn get_chars() -> Vec<char> {
    get_line().chars().collect()
}

#[allow(dead_code)]
fn vec_min(xs: &Vec<i64>) -> i64 {
    xs.iter().map(|&x|x).fold(std::i64::MAX, std::cmp::min)
}

#[allow(dead_code)]
fn vec_max(xs: &Vec<i64>) -> i64 {
    xs.iter().map(|&x|x).fold(std::i64::MIN, std::cmp::max)
}

#[allow(dead_code)]
fn vec_sum(xs: &Vec<i64>) -> i64 {
    xs.iter().fold(0, |acc, &x| acc+x)
}
