use std::io::stdin;
use std::str::FromStr;
use std::cmp::min;

fn main(){
    let _: i64 = get_one();
    let xs: Vec<i64> = get_vec();
    let mut next = 1;
    let mut nodes_count = 0;
    let max_nodes = rev_cumsum(&xs);
    let mut impossible = false;
    for (&x, &m) in xs.iter().zip(max_nodes.iter()) {
        let available = min(m, next);
        if x > available {
            impossible = true;
            break;
        }
        nodes_count += available;
        next = (available - x) * 2;
    }
    if impossible {
        println!("-1");
    } else {
        println!("{}", nodes_count);
    }
}

fn rev_cumsum(v: &Vec<i64>) -> Vec<i64> {
    let mut cur = 0;
    let mut result = Vec::new();
    for &e in v.iter().rev() {
        cur += e;
        result.push(cur);
    }
    result.reverse();
    result
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
