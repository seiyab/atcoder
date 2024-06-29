use std::io::stdin;
use std::str::FromStr;
use rand::Rng;
use std::time::{Duration, SystemTime};

const MOD: i64 = 998244353;

fn main() {
    let start = SystemTime::now();
    let mut rng = rand::thread_rng();
    let (n, m, k): (usize, usize, usize) = get_triple();
    let mut A = get_square(n);
    let mut Ss = Vec::new();
    for _ in 0..m {
        Ss.push(get_square(3));
    }
    
    let mut env = Env::new(n, m, k, A.clone(), Ss.clone());
    
    loop {
        let el = start.elapsed().map(|e| e.as_millis()).unwrap_or(2_000);
        if el > 1_900 {
            break;
        }
        let t: usize = rng.gen_range(0..m);
        let z: usize = rng.gen_range(0..k);
        let p: usize = rng.gen_range(0..n - 2);
        let q: usize = rng.gen_range(0..n - 2);
        let step = step(t, p, q);
        let d = env.dry_patch(z, &step);
        let th: i64 = rng.gen_range(0..=(MOD * (1900 - el as i64) / 1900) / 3);
        if d + th > 0 {
            env.patch(z, Some(step));
        }
    }

    env.print();
}

struct Env {
    n: usize,
    m: usize,
    k: usize,
    A: Vec<Vec<i64>>,
    Ss: Vec<Vec<Vec<i64>>>,
    actions: Vec<Option<Step>>,
}

impl Env {
    fn new(n: usize, m: usize, k: usize, A: Vec<Vec<i64>>, Ss: Vec<Vec<Vec<i64>>>) -> Self {
        Env {n, m, k, A, Ss, actions: vec![None; k]}
    }
    
    fn score(&self) -> i64 {
        let mut s = 0;
        for aa in self.A.iter() {
            for a in aa.iter() {
                s += a % MOD;
            }
        }
        return s;
    }
    
    fn estimate(&self, step: &Step) -> i64 {
        let mut s = 0;
        for i in 0..self.m {
            for j in 0..self.m {
                let mut x = self.A[i][j];
                if i <= step.p && step.p < i + 3 
                    && j <= step.q && step.q < j + 3 {
                        x += self.Ss[step.t][i][j]
                }
                s += x % MOD;
            }
        }
        return s;
    }
    
    fn patch(&mut self, t: usize, step: Option<Step>) {
        let prev = self.actions[t].clone();
        if let Some(ref s) = step {
            for i in 0..3 {
                for j in 0..3 {
                    let inc = self.Ss[s.t][i][j];
                    self.A[s.p + i][s.q + j] += inc;
                    self.A[s.p + i][s.q + j] %= MOD;
                }
            }
        }
        if let Some(s) = prev {
            for i in 0..3 {
                for j in 0..3 {
                    let dec = MOD - self.Ss[s.t][i][j];
                    self.A[s.p + i][s.q + j] += dec;
                    self.A[s.p + i][s.q + j] %= MOD;
                }
            }
        }
        self.actions[t] = step;
    }
    
    fn dry_patch(&mut self, t: usize, step: &Step) -> i64 {
        let prev = self.actions[t].clone();
        let mut diff = 0;
        for i in 0..3 {
            for j in 0..3 {
                let inc = self.Ss[step.t][i][j];
                let prevA = self.A[step.p + i][step.q + j];
                let newA = (prevA + inc) % MOD;
                diff += newA - prevA;
            }
        }
        if let Some(s) = prev {
            for i in 0..3 {
                for j in 0..3 {
                    let dec = MOD - self.Ss[s.t][i][j];
                    let prevA = self.A[s.p + i][s.q + j];
                    let newA = (prevA + dec) % MOD;
                    diff += newA - prevA;
                }
            }
        }
        return diff;
    }
    
    fn print(&self) {
        let xs = self.actions.iter().filter(|a| a.is_some()).map(|a| a.clone().unwrap()).collect::<Vec<Step>>();
        println!("{}", xs.len());
        for a in xs.iter() {
            println!("{} {} {}", a.t, a.p, a.q);
        }
    }
}

#[derive(Clone)]
struct Step {
    t: usize,
    p: usize,
    q: usize,
}

fn step(t: usize, p: usize, q: usize) -> Step {
    Step {t, p, q}
}

fn get_square(n: usize) -> Vec<Vec<i64>>  {
    let mut result = Vec::new();
    for _ in 0..n {
        let row: Vec<i64> = get_vec();
        result.push(row);
    }
    return result;
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