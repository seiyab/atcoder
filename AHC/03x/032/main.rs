use std::io::stdin;
use std::str::FromStr;

const MOD: i64 = 998244353;

fn main() {
    let (n, m, k): (usize, usize, usize) = get_triple();
    let mut A = get_square(n);
    let mut Ss = Vec::new();
    for _ in 0..m {
        Ss.push(get_square(3));
    }
    
    let mut env = Env::new(n, m, k, A.clone(), Ss.clone());
    for i in 0..(n-2) {
        for j in 0..(n-2) {
            let mut c = None;
            let mut l = env.A[i][j];
            for t in 0..m {
                let s = (env.A[i][j] + Ss[t][0][0]) % MOD;
                if s > l {
                    l = s;
                    c = Some(step(t, i, j));
                }
            }
            if let Some(step) = c {
                env.apply(&step);
            }
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
    actions: Vec<Step>,
}

impl Env {
    fn new(n: usize, m: usize, k: usize, A: Vec<Vec<i64>>, Ss: Vec<Vec<Vec<i64>>>) -> Self {
        Env {n, m, k, A, Ss, actions: Vec::new()}
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
                        x += self.Ss[step.t][step.p + i][step.q + j]
                }
                s += x % MOD;
            }
        }
        return s;
    }
    
    fn apply(&mut self, step: &Step) {
        self.actions.push(step.clone());
        for i in 0..3 {
            for j in 0..3 {
                self.A[step.p + i][step.q + j] = (self.A[step.p + i][step.q + j] + self.Ss[step.t][i][j]) % MOD;
            }
        }
    }
    
    fn print(&self) {
        println!("{}", self.actions.len());
        for a in self.actions.iter() {
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