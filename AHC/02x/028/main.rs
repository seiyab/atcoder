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
    cost: i64,
    steps: Vec<(usize, usize)>,
}

impl Word {
    fn new(s: Vec<char>, a: &Vec<Vec<char>>) -> Word {
        let mut steps = Vec::new();
        let center = (a.len()/2, a.len()/2);
        let mut prev = (a.len()/2, a.len()/2);
        let mut cst = 0;
        for c in s.iter().cloned() {
            let mut d = (a.len() * a.len() * 100) as i64;
            let mut next = prev;
            for i in 0..a.len() {
                for j in 0..a.len() {
                    if a[i][j] == c {
                        let dd1 = cost((i, j), prev);
                        let dd2 = cost((i, j), center);
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