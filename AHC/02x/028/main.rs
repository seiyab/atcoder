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
            v.push(get_chars());
        }
        v
    };
    
    let mut words = Vec::new();
    for (i, s) in t.iter().enumerate() {
        words.push(Word::new(s.clone(), &a, i));
    }
    
    let mut todo: HashSet<usize> = HashSet::new();
    for i in 0..m {
        todo.insert(i);
    }
    
    let cur = (si, sj);
    let init = Env::new(m, &cur, a[si][sj]);
    let mut b = BeamSearch::new(100, vec![init]);
    for _ in 0..m {
        b.search(&words);
    }

    for (_, i, j) in b.best().steps.iter() {
        println!("{} {}", i, j);
    }
}

#[derive(Clone)]
struct Env {
    score: i64,
    steps: Vec<(char, usize, usize)>,
    todo: HashSet<usize>,
}

impl Env {
    fn new(m: usize, s: &(usize, usize), c: char) -> Env {
        let mut todo: HashSet<usize> = HashSet::new();
        for i in 0..m {
            todo.insert(i);
        }
        return Env{score: 0, steps: vec![(c, s.0, s.1)], todo: todo};
    }
}

impl State for Env {
    type Action = Word;
    type Input = Vec<Word>;
    
    fn estimate(&self, action: &Self::Action) -> i64 {
        let last_t = self.steps.last().unwrap();
        let last = (last_t.1, last_t.2);
        if last_chunk::<2>(&self.steps) == Some([action.s[0], action.s[1]]) {
            return action.estimate2(last);
        }
        if last_chunk::<1>(&self.steps) == Some([action.s[0]]) {
            return action.estimate1(last);
        }
        return action.estimate0(last);
    }

    fn apply(mut self, action: &Self::Action) -> Self {
        let e = self.estimate(action);
        self.score += e;
        let dup = if last_chunk::<2>(&self.steps) == Some([action.s[0], action.s[1]]) {
            2
        } else if last_chunk::<1>(&self.steps) == Some([action.s[0]]) {
            1
        } else { 0 };
        for i in dup..action.steps.len() {
            self.steps.push((action.s[i], action.steps[i].0, action.steps[i].1));
        }
        self.todo.remove(&action.idx);
        return self;
    }

    fn available_actions(&self, input: &Self::Input) -> Vec<Self::Action> {
        let mut acs = Vec::new();
        for i in self.todo.iter() {
            acs.push(input[*i].clone());
        }
        return acs;
    }
}

fn last_chunk<const N: usize>(v: &Vec<(char, usize, usize)>) -> Option<[char; N]> {
    if v.len() < N {
        return None;
    }
    let mut a = ['a'; N];
    for i in 0..N {
        a[i] = v[v.len()-N+i].0;
    }
    return Some(a);
}

#[derive(Clone)]
struct Word {
    idx: usize,
    s: Vec<char>,
    steps: Vec<(usize, usize)>,
}

impl Word {
    fn new(s: Vec<char>, a: &Vec<Vec<char>>, idx: usize) -> Word {
        let mut dict = HashMap::new();
        for (i, x) in a.iter().enumerate() {
            for (j, y) in x.iter().enumerate() {
                let c = dict.entry(*y).or_insert(Vec::new());
                c.push((i, j));
            }
        }
        let mut b = BeamSearch::new(10, vec![WordEnv{n: a.len(), word: s.clone(), path: Vec::new()}]);
        for _ in 0..s.len() {
            b.search(&dict);
        }
        return Word{idx: idx, s: s, steps: b.best().path.clone()};
    }
    
    fn estimate0(&self, pos: (usize, usize)) -> i64 {
        -cost(pos, self.steps[0])
    }
    
    fn estimate1(&self, pos: (usize, usize)) -> i64 {
        let c = cost(pos, self.steps[1]);
        let d = cost(self.steps[0], self.steps[1]);
        d - c
    }
    
    fn estimate2(&self, pos: (usize, usize)) -> i64 {
        let c = cost(pos, self.steps[2]);
        let d0 = cost(self.steps[1], self.steps[2]);
        let d1 = cost(self.steps[1], self.steps[2]);
        d0 + d1 - c
    }
}

fn cost(f: (usize, usize), t: (usize, usize)) -> i64 {
    let (fi, fj) = f;
    let (ti, tj) = t;
    1 + ((fi as i64 - ti as i64).abs() + (fj as i64 - tj as i64).abs()) as i64
}

#[derive(Clone)]
struct WordEnv {
    n: usize,
    word: Vec<char>,
    path: Vec<(usize, usize)>,
}

impl State for WordEnv {
    type Action = (usize, usize);
    type Input = HashMap<char, Vec<(usize, usize)>>;

    fn estimate(&self, action: &(usize, usize)) -> i64 {
        let center = (self.n/2, self.n/2);
        let prev = self.path.last().unwrap_or(&center);
        let mut d = cost(prev.clone(), action.clone());
        if self.path.len() == 4 {
            d += cost(action.clone(), center) / 2;
        }
        return -d;
    }
    fn apply(mut self, action: &Self::Action) -> Self {
        self.path.push(action.clone());
        return self;
    }
    fn available_actions(&self, dict: &Self::Input) -> Vec<Self::Action> {
        let l = self.path.len();
        let nc = self.word[l];
        return dict.get(&nc).unwrap().clone();
    }
}

struct BeamSearch<S: Clone + State> {
    beam_width: usize,
    beam: Vec<S>,
}


impl <S: Clone + State> BeamSearch<S> {
    fn new(beam_width: usize, initial_states: Vec<S>) -> Self {
        BeamSearch {beam_width, beam: initial_states}
    }
    
    fn search(&mut self, input: &S::Input) {
        let mut next_beam = Vec::new();
        for state in self.beam.iter() {
            for action in state.available_actions(input) {
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
    type Input;
    fn estimate(&self, action: &Self::Action) -> i64;
    fn apply(self, action: &Self::Action) -> Self;
    fn available_actions(&self, input: &Self::Input) -> Vec<Self::Action>;
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