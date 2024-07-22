use std::io::stdin;
use std::str::FromStr;
use std::env;

fn main() {
    let (n, _m, t): (usize, usize, usize) = get_triple();
    let mut x = get_seeds(n);
    let mut a = Vec::new();
    for i in 0..t {
        if i != 0 {
            x = harvest(n, &x, &a);
        }
        a = if i < 3 {
            planting_max(n, &x)
        } else {
            planting_sum(n, &x)
        };
        print_planting(&a);
    }
}

fn planting_max(n: usize, x: &Vec<Seed>) -> Vec<Vec<usize>> {
    let stat = Stat::new(x);
    let mut xi = x.iter().enumerate().collect::<Vec<_>>();
    xi.sort_by_key(|&(_, ref s)| -eval_max(&s.x, &stat));
    let ii = xi.iter().map(|&(i, _)| i).collect::<Vec<_>>();
    return center_first(n, ii);
}

fn planting_sum(n: usize, x: &Vec<Seed>) -> Vec<Vec<usize>> {
    let stat = Stat::new(x);
    let mut xi = x.iter().enumerate().collect::<Vec<_>>();
    xi.sort_by_key(|&(_, ref s)| -eval_sum(&s.x, &stat));
    let ii = xi.iter().map(|&(i, _)| i).collect::<Vec<_>>();
    return center_first(n, ii);
}

fn center_first(n: usize, is: Vec<usize>) -> Vec<Vec<usize>> {
    let (mut s, mut e) = (n/2-1, n/2);
    let mut a = vec![vec![0; n]; n];
    let mut i = 0;
    loop {
        for j in s..=e {
            a[s][j] = is[i];
            i += 1;
        }
        for j in (s+1)..e {
            a[j][s] = is[i];
            i += 1;
            a[j][e] = is[i];
            i += 1;
        }
        for j in s..=e {
            a[e][j] = is[i];
            i += 1;
        }
        if s == 0 {
            break;
        }
        s -= 1;
        e += 1;
    }
    return a;
}

fn eval_max(v: &Vec<i64>, stat: &Stat) -> i64 {
    let mut m = 0;
    for i in 0..v.len() {
        let x = 10_000 * v[i] / stat.best[i];
        if x > m {
            m = x;
        }
    }
    return m;
}

fn eval_sum(v: &Vec<i64>, stat: &Stat) -> i64 {
    return v.iter().sum();
}

fn get_seeds(n: usize) -> Vec<Seed> {
    let mut s = Vec::new();
    for _ in 0..(2 * n * (n-1)) {
        s.push(Seed::new(get_vec()));
    }
    return s;
}

struct Seed {
    x: Vec<i64>,
}

impl Seed {
    fn new(x: Vec<i64>) -> Seed {
        Seed {x: x}
    }
}

struct Stat {
    best: Vec<i64>,
}

impl Stat {
    fn new(seeds: &Vec<Seed>) -> Stat {
        let mut best = seeds[0].x.clone();
        for seed in seeds.iter() {
            for (i, &x) in seed.x.iter().enumerate() {
                if x > best[i] {
                    best[i] = x;
                }
            }
        }
        return Stat {best: best};
    }
}

fn harvest(n: usize, x: &Vec<Seed>, a: &Vec<Vec<usize>>) -> Vec<Seed> {
    if env::var("VISUALIZER") == Ok("1".to_string()) {
        return harvest_visualizer(n, x, a);
    }
    return get_seeds(n);
}

fn harvest_visualizer(n: usize, x: &Vec<Seed>, a: &Vec<Vec<usize>>) -> Vec<Seed> {
    let mut s = Vec::new();
    for i in 0..n {
        let z = get_vec::<String>();
        for (j, d) in z.iter().enumerate() {
            let cs: Vec<usize> = d.chars().map(|c| if c == '0' { 0 } else { 1 }).collect();
            let mut v = Vec::new();
            for (k, c) in cs.iter().enumerate() {
                v.push(x[a[i][j+c]].x[k])
            }
            s.push(Seed::new(v));
        }
    }
    for i in 0..n-1 {
        let z = get_vec::<String>();
        for (j, d) in z.iter().enumerate() {
            let cs: Vec<usize> = d.chars().map(|c| if c == '0' { 0 } else { 1 }).collect();
            let mut v = Vec::new();
            for (k, c) in cs.iter().enumerate() {
                v.push(x[a[i+c][j]].x[k])
            }
            s.push(Seed::new(v));
        }
    }
    return s;
}


#[allow(dead_code)]
fn random_planting(n: usize) -> Vec<Vec<usize>> {
    let mut a = Vec::new();
    for i in 0..n {
        let mut l = Vec::new();
        for j in 0..n {
            l.push(i * n + j);
        }
        a.push(l)
    }
    return a;
}

fn print_planting(a: &Vec<Vec<usize>>) {
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            if j != 0 {
                print!(" ");
            }
            print!("{}", a[i][j]);
        }
        println!();
    }
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