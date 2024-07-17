use std::collections::HashMap;
use std::collections::HashSet;
use std::io::stdin;
use std::str::FromStr;

fn main() {
    let (n, m): (usize, usize) = get_pair();
    let (si, sj): (usize, usize) = get_pair();
    let a: Vec<Vec<char>> = get_square(n);
    let t = {
        let mut v = Vec::new();
        for _ in 0..m {
            v.push(get_line());
        }
        v
    };
    
    let mut words = Vec::new();
    for s in t.iter() {
        words.push(Word::new(s.clone(), &a));
    }
    
    let mut dict = HashMap::new();
    for (i, s) in t.iter().enumerate() {
        let mut c = s.chars().next().unwrap();
        dict.insert(c, i);
    }
    
    let mut todo: HashSet<usize> = HashSet::new();
    for i in 0..m {
        todo.insert(i);
    }
    
    let mut i = 0;
    let mut cur = None;
    let mut steps = Vec::new();
    while todo.len() > 0 {
        if let Some(w) = cur {
            if let Some(&j) = dict.get(&w) {
                todo.remove(&j);
                dict.remove(&w);
                let x = &words[j];
                for k in 1..x.steps.len() {
                    steps.push(x.steps[k]);
                }
                cur = x.s.chars().last();
                continue
            }
        }
        while !todo.contains(&i) {
            i += 1;
        }
        todo.remove(&i);
        for s in words[i].steps.iter() {
            steps.push(s.clone());
        }
        cur = words[i].s.chars().last();
    }

    for (i, j) in steps {
        println!("{} {}", i, j);
    }
}

struct Word {
    s: String,
    cost: i64,
    steps: Vec<(usize, usize)>,
}

impl Word {
    fn new(s: String, a: &Vec<Vec<char>>) -> Word {
        let mut steps = Vec::new();
        let center = ((a.len()/2) as i64, (a.len()/2) as i64);
        let mut prev = (a.len()/2, a.len()/2);
        let mut cst = 0;
        for c in s.chars() {
            let mut d = (a.len() * a.len() * 100) as i64;
            let mut next = prev;
            for i in 0..a.len() {
                for j in 0..a.len() {
                    if a[i][j] == c {
                        let ii = i as i64;
                        let jj = j as i64;
                        let dd1 = (ii - prev.0 as i64).abs() + (jj - prev.1 as i64).abs();
                        let dd2 = (ii - center.0).abs() + (jj - center.1).abs();
                        let dd = dd1 * (a.len()) as i64 * 2 + dd2;
                        if dd < d {
                            d = dd;
                            next = (i, j);
                        }
                    }
                }
            }
            steps.push(next);
            cst += cost(prev, next);
            prev = next;
        }
        return Word{s: s, cost: cst, steps: steps};
    }
}

fn cost(f: (usize, usize), t: (usize, usize)) -> i64 {
    let (fi, fj) = f;
    let (ti, tj) = t;
    1 + ((fi as i64 - ti as i64).abs() + (fj as i64 - tj as i64).abs()) as i64
}

#[allow(dead_code)]
fn get_square(n: usize) -> Vec<Vec<char>> {
    let mut a = Vec::new();
    for _ in 0..n {
        a.push(get_chars());
    }
    return a;
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