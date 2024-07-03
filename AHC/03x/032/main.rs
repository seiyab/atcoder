use std::io::stdin;
use std::str::FromStr;

const MOD: i64 = 998244353;

fn main() {
    let (n, m, k): (usize, usize, usize) = get_triple();
    let A = get_square(n);
    let mut Ss = Vec::new();
    for _ in 0..m {
        Ss.push(get_square(3));
    }
    
    let env = Env::new(n, m, k, A.clone(), Ss.clone());
    let mut beam_search = BeamSearch::new(30, vec![env]);
    for _ in 0..((n-2) * (n-2)) {
        beam_search.search();
    }
    beam_search.best().print();
}

#[derive(Clone)]
struct Env {
    n: usize,
    m: usize,
    k: usize,
    i: usize,
    j: usize,
    A: Vec<Vec<i64>>,
    Ss: Vec<Vec<Vec<i64>>>,
    fixed_score: i64,
    actions: Vec<Step>,
}

impl Env {
    fn new(n: usize, m: usize, k: usize, A: Vec<Vec<i64>>, Ss: Vec<Vec<Vec<i64>>>) -> Self {
        Env {
            n, m, k, A, Ss,
            i: 0,
            j: 0,
            fixed_score: 0,
            actions: Vec::new(),
        }
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
    
    fn estimate_local(&self, step: &Step, i: usize, j: usize) -> i64 {
        let mut x = self.A[i][j];
        if i <= step.p && step.p < i + 3 
            && j <= step.q && step.q < j + 3 {
                x += self.Ss[step.t][step.p + i][step.q + j]
        }
        return x % MOD;
    }

    fn gain(&self, step: &Step) -> i64 {
        if step.p == self.n-3 && step.q == self.n-3 {
            return {
                let mut s = 0;
                for i in 0..3 {
                    for j in 0..3 {
                        let next = (self.A[step.p + i][step.q + j] + self.Ss[step.t][i][j]) % MOD;
                        s += next;
                    }
                }
                s
            }
        }
        if step.p <= step.q {
            return if step.q == self.n-3 {
                self.gain_top(step)
            } else {
                (self.A[step.p][step.q] + self.Ss[step.t][0][0]) % MOD
            }
        }
        return if step.p == self.n-3 {
            self.gain_left(step)
        } else {
            (self.A[step.p][step.q] + self.Ss[step.t][0][0]) % MOD
        }
    }

    fn gain_top(&self, step: &Step) -> i64 {
        let mut d = 0;
        let i = 0;
        for j in 0..3 {
            let next = (self.A[step.p + i][step.q + j] + self.Ss[step.t][i][j]) % MOD;
            d += next;
        }
        return d;
    }
    
    fn gain_left(&self, step: &Step) -> i64 {
        let mut d = 0;
        let j = 0;
        for i in 0..3 {
            let next = (self.A[step.p + i][step.q + j] + self.Ss[step.t][i][j]) % MOD;
            d += next;
        }
        return d;
    }
    
    fn print(&self) {
        println!("{}", self.actions.len());
        for a in self.actions.iter() {
            println!("{} {} {}", a.t, a.p, a.q);
        }
    }
}

impl State for Env {
    type Action = Step;
    
    fn estimate(&self, action: &Step) -> i64 {
        return self.fixed_score + self.gain(action);
    }
    
    fn apply(mut self, step: &Step) -> Self {
        self.actions.push(step.clone());
        self.fixed_score += self.gain(step);
        for i in 0..3 {
            for j in 0..3 {
                self.A[step.p + i][step.q + j] = (self.A[step.p + i][step.q + j] + self.Ss[step.t][i][j]) % MOD;
            }
        }
        if self.i <= self.j {
            if self.j == self.n-3 {
                self.j = self.i;
                self.i += 1;
            } else {
                self.j += 1;
            }
        } else {
            if self.i == self.n-3 {
                self.i = self.j + 1;
                self.j = self.i;
            } else {
                self.i += 1;
            }
        }
        return self;
    }
    
    fn available_actions(&self) -> Vec<Step> {
        let mut actions = Vec::new();
        for t in 0..self.m {
            actions.push(step(t, self.i, self.j));
        }
        return actions;
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

struct BeamSearch<S: Clone + State> {
    beam_width: usize,
    beam: Vec<S>,
}

impl <S: Clone + State> BeamSearch<S> {
    fn new(beam_width: usize, initial_states: Vec<S>) -> Self {
        BeamSearch {beam_width, beam: initial_states}
    }
    
    fn search(&mut self) {
        let mut next_beam = Vec::new();
        for state in self.beam.iter() {
            for action in state.available_actions() {
                next_beam.push((state, action));
            }
        }
        next_beam.sort_by_key(|(state, action)| -state.estimate(action));
        next_beam.truncate(self.beam_width);
        self.beam = next_beam
            .iter()
            .map(|(&ref state, action)| state.clone().apply(action))
            .collect();
        return;
    }
    
    fn best(&self) -> &S {
        self.beam.first().unwrap()
    }
}

trait State {
    type Action;
    fn estimate(&self, action: &Self::Action) -> i64;
    fn apply(self, action: &Self::Action) -> Self;
    fn available_actions(&self) -> Vec<Self::Action>;
}

fn get_square(n: usize) -> Vec<Vec<i64>> {
    let mut a = Vec::new();
    for _ in 0..n {
        a.push(get_vec());
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