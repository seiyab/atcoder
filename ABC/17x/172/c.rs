use std::io::stdin;
use std::str::FromStr;
use std::cmp::max;

fn main(){
    let (_n, _m, k): (i64, i64, i64) = get_triple();
    let xs: Vec<i64> = get_vec();
    let ys: Vec<i64> = get_vec();
    let xcs = cumsum(&xs);
    let ycs = cumsum(&ys);
    let mut ans = 0;

    for i in 0..xcs.len() {
        let cost_x = xcs[i];
        let cost_rem = k - cost_x;
        if let Some(y_n) = upper_bound(&ycs, &|y| *y <= cost_rem) {
            ans = max(ans, i + y_n);
        } 
    }

    println!("{}", ans);
}

fn cumsum(v: &Vec<i64>) -> Vec<i64> {
    let mut result = vec![0];
    let mut sum = 0;
    for &elm in v.iter() {
        sum += elm;
        result.push(sum);
    }
    result
}

fn upper_bound<T, F>(v: &Vec<T>, f: &F) -> Option<usize>
    where F: Fn(&T) -> bool {
    if v.first().map(f) != Some(true) {
        return None;
    }

    let last_idx = (v.len() as i64 -1) as usize;
    if f(&v[last_idx]) {
        return Some(last_idx);
    }

    return Some(upper_bound(v, f, 0, last_idx));

    fn upper_bound<T, F>(v: &Vec<T>, f: &F, left: usize, right: usize) -> usize 
        where F: Fn(&T) -> bool {
        if left+1 == right {
            return left;
        }
        let m = (left + right) / 2;
        if f(&v[m]) {
            return upper_bound(v, f, m, right);
        } else {
            return upper_bound(v, f, left, m);
        }
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
