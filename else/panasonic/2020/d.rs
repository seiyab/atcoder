use std::io::stdin;
use std::cmp::max;
use std::str::FromStr;


fn main(){
    let x: usize = get_one();
    let cs: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
    solve("a".to_string(), 0, x, &cs);
}

fn solve(s: String, m: usize, n: usize, cs: &Vec<char>) {
    if s.len() == n {
        println!("{}", s);
        return
    }
    for i in 0..m+2 {
        solve(format!("{}{}", &s, cs[i]), max(m, i), n, cs);
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
