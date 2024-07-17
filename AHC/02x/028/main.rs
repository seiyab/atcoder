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
    for s in t.iter() {
        words.push(Word::new(s.clone(), &a));
    }
    
    /*
    let mut dict = HashMap::new();
    for (i, s) in t.iter().enumerate() {
        let mut c = s.chars().next().unwrap();
        dict.insert(c, i);
    }
    */
    
    let mut todo: HashSet<usize> = HashSet::new();
    for i in 0..m {
        todo.insert(i);
    }
    
    let mut cur = (si, sj);
    let mut steps = Vec::new();
    let mut pw: Option<Vec<char>> = None;
    while todo.len() > 0 {
        let mut gr_cost = 10000;
        let mut candidate = todo.iter().next().unwrap().clone();
        let mut dup = 0;
        for i in todo.iter() {
            let w = words[*i].estimate0(cur);
            if w < gr_cost {
                gr_cost = w;
                candidate = *i;
            }
        }
        
        if let Some(word) = pw {
            let l0 = word[word.len()-1];
            let l1 = word[word.len()-2];
            for i in todo.iter() {
                if l0 != t[*i][0] {
                    continue
                }
                let w = words[*i].estimate1(cur);
                if w == gr_cost {
                    dup = 1;
                    gr_cost = w;
                    candidate = *i;
                }
            }
            
            for i in todo.iter() {
                let c0 = t[*i][0];
                let c1 = t[*i][1];
                if l1 != c0 || l0 != c1 {
                    continue
                }
                let w = words[*i].estimate2(cur);
                if w == gr_cost {
                    dup = 2;
                    gr_cost = w;
                    candidate = *i;
                }
            }
        }

        
        let nx = candidate;
        for (i, x) in words[nx].steps.iter().enumerate() {
            if i < dup {
                continue
            }
            steps.push(*x);
        }
        todo.remove(&nx);
        cur = words[nx].steps[words[nx].steps.len()-1];
        pw = Some(t[nx].clone());
    }

    for (i, j) in steps {
        println!("{} {}", i, j);
    }
}

struct Word {
    s: Vec<char>,
    steps: Vec<(usize, usize)>,
}

impl Word {
    fn new(s: Vec<char>, a: &Vec<Vec<char>>) -> Word {
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
        return Word{s: s, steps: b.best().path.clone()};
    }
    
    fn estimate0(&self, pos: (usize, usize)) -> i64 {
        cost(pos, self.steps[0])
    }
    
    fn estimate1(&self, pos: (usize, usize)) -> i64 {
        let c = cost(pos, self.steps[1]);
        let d = cost(self.steps[0], self.steps[1]);
        c - d
    }
    
    fn estimate2(&self, pos: (usize, usize)) -> i64 {
        let c = cost(pos, self.steps[2]);
        let d0 = cost(self.steps[1], self.steps[2]);
        let d1 = cost(self.steps[1], self.steps[2]);
        c - d0 - d1
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