use std::io::stdin;
use std::str::FromStr;

fn main(){
    let _n: usize = get_one();
    let mut s = get_chars();
    s.reverse();
    let m = s.iter().fold(0, |acc, &c| if c == '1' {acc + 1} else {acc});
    let mut z: Vec<Vec<usize>> = Vec::new();
    let mut d: Vec<usize> = vec![1, 1, 1];
    for _i in 0..s.len() {
        z.push(d.iter().map(|&x| x).collect());
        if m==0 {
            d = vec![1, 1, d[2]*2 % (m+1)]
        } else if m == 1 {
            d = vec![1, d[1]*2 % m, d[2]*2 % (m+1)];
        } else {
            d = vec![d[0]*2 % (m-1), d[1]*2 % m, d[2]*2 % (m+1)];
        }
    }
    // println!("m: {}", m);
    // for i in 0..d.len() {
        // println!("d: {}", d[i]);
    // }
    // println!("");

    let mut start_bases = Vec::new();
    for i in 0..3 {
        let mut b = 0;
        for j in 0..s.len() {
            if s[j] == '1' {
                b += z[j][i];
                if m == 1 && i ==0 {
                    b = 1;
                } else {
                    b %= m-1+i;
                }
            }
        }
        start_bases.push(b);
    }

    let mut starts = Vec::new();
    for i in 0..s.len() {
        if s[i] == '0' {
            let mut l = start_bases[2];
            l += z[i][2];
            starts.push(Some(l % (m+1)));
        } else {
            if m == 1 {
                starts.push(None);
            } else {
                let mut l = start_bases[0];
                l += (m-1) - z[i][0];
                starts.push(Some(l % (m-1)));
            }
        }
    }

    // for i in 0..starts.len() {
    //     println!("s: {}", starts[i]);
    // }
    // println!("");

    let anss: Vec<usize> = starts.iter().rev().map(
        |i| i.map(|i| f(i) + 1).unwrap_or(0)
    ).collect();

    for ans in anss {
        println!("{}", ans);
    }
}

fn f(n: usize) -> usize {
    if n == 0 { 0 }
    else {
        let p = popcount(n);
        1 + f(n % p)
    }
}

fn popcount(n: usize) -> usize {
    if n == 0 { 0 }
    else { n % 2 + popcount(n/2) }
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
