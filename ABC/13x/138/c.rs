use std::io::stdin;
use std::str::FromStr;
use std::collections::HashMap;

fn main(){
    let s = get_chars();
    let t = get_chars();
    let lookup = new_lookup(&s);
    let mut z = 0;
    let mut i = 0;
    let mut impossible = false;
    for &c in t.iter() {
        if i >= s.len() {
            i = 0;
            z = z+1;
            // println!("next");
        }
        match find_next(&lookup, c, i) {
            Some(j) => {
                i = j+1;
                // println!("{}: {}", c, j);
            },
            None => match find_next(&lookup, c, 0) {
                Some(k) => {
                    i = k+1;
                    z = z+1;
                    // println!("{}: {} (new s)", c, k);
                },
                None => {
                    impossible = true
                },
            },
        }
    }
    if impossible {
        println!("-1");
    } else {
        println!("{}", z * s.len() + i)
    }
}

fn new_lookup(s: &Vec<char>) -> Vec<HashMap<char, usize>> {
    let mut ret = vec![HashMap::new(); s.len()];
    for (i, &c) in s.iter().enumerate() {
        let mut j = i as i64;
        while j >= 0 {
            let k = j as usize;
            if i != k && s[k] == c {
                break
            }
            ret[k].insert(c, i);
            j -= 1;
        }
    }
    ret
}

fn find_next(lookup: &Vec<HashMap<char, usize>>, c: char, from: usize) -> Option<usize> {
    lookup[from].get(&c).map(|i| i.clone())
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
