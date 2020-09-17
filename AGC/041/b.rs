use std::io::stdin;
use std::str::FromStr;
use std::cmp::max;
use std::cmp::min;

fn main(){
    let (_n, m, v, p)  = {
        let v: Vec<usize> = get_vec();
        (v[0], v[1], v[2], v[3])
    };
    let mut xs: Vec<i64> = get_vec();
    xs.sort();
    xs.reverse();

    let s = p as i64 - 1;
    let u = max(0, v as i64 - s);
    let zs = &xs[s as usize..];

    let loc = solve_local(zs, u, m as i64) as i64;

    println!("{}", s + loc);
}

fn solve_local(zs: &[i64], v: i64, m: i64) -> usize {
    let ok = |i: usize| {
        let a = zs[i] + m;
        if a < zs[0] { return false };
        let sum: i64 = zs.iter().map(|z| min(a-z, m)).sum();
        if sum < v*m { return false };
        true
    };
    upper_bound(&ok, 0, zs.len()) + 1
}

fn upper_bound<F>(ok: &F, left: usize, right: usize) -> usize 
    where F: Fn(usize) -> bool {
    if left+1 >= right {
        return left;
    }
    let m = (left + right) / 2;
    if ok(m) {
        return upper_bound(ok, m, right);
    } else {
        return upper_bound(ok, left, m);
    }
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
