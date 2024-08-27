use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::stdin;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;

fn main() {
    let (n, m, _t, la, lb): (usize, usize, usize, usize, usize) = get_five();
    let edges = get_edges(n, m);
    let ts: Vec<usize> = get_vec();
    discard_xys(n);
    
    let (aa, steps) = solve(n, &edges, &ts, la, lb);
    
    if env::var("SCORE") == Ok("1".to_string()) { 
        if steps.len() > 100_000 {
            panic!("too many elements");
        }
        let s = steps.iter().filter(|a| match a {
            &&Step::Signal(_, _, _) => true,
            _ => false,
        }).count();
        println!("{}", s);
    } else {
        for (i, a) in aa.iter().enumerate() {
            if i != 0 {
                print!(" ");
            }
            print!("{}", a);
        }
        println!();
        for s in steps {
            println!("{}", s.to_string());
        }
    }
}

fn solve(
    n: usize,
    edges: &Vec<HashSet<usize>>,
    ts: &Vec<usize>,
    la: usize,
    lb: usize,
) -> (Vec<usize>, Vec<Step>) {
    let path = entire_path(n, edges, ts);
    // let mut as_fw = Vec::new();
    let mut as_fw = greedy_as(&path);
    let mut as_rv = rev_as(&as_fw, n);
    let mut as_yet: HashSet<_> = (0..n).collect();
    let mut bs: HashSet<usize> = HashSet::new();
    let mut steps: Vec<Step> = Vec::new();
    let mut pos = 0;
    for i in 0..path.len() {
        let p = path[i];
        if !bs.contains(&p) {
            let sig = select_bs(&as_fw, &as_rv, n, &path, i, lb).unwrap();
            steps.push(sig.step());
            bs = sig.bs(&as_fw);
        } /* else {
            let mut pp = Vec::new();
            for j in 0..lb {

            }
            pp.push(p);
        } */
        // steps.push(signal(1, p, 0));
        // bs = HashSet::new();
        // bs.insert(p);
        steps.push(mv(p));
        pos = p;
    }
    (fill_dummy(&as_fw, la), steps)
}

fn entire_path(
    n: usize,
    edges: &Vec<HashSet<usize>>,
    ts: &Vec<usize>,
) -> Vec<usize> {
    let mut path = Vec::new();
    let mut pos = 0;
    for t in ts.iter().copied() {
        let p = dijkstra(&edges, pos, t);
        path.extend(p);
        pos = t;
    }
    return path;
}

enum Step {
    Signal(usize, usize, usize),
    Move(usize),
}

fn signal(len: usize, src: usize, dst: usize) -> Step {
    Step::Signal(len, src, dst)
}

fn mv(dst: usize) -> Step {
    Step::Move(dst)
}

impl Step {
    fn to_string(&self) -> String {
        match self {
            Step::Signal(len, src, dst) => format!("s {} {} {}", len, src, dst),
            Step::Move(dst) => format!("m {}", dst),
        }
    }
}

struct Signal(usize, usize, usize);

impl Signal {
    fn step(&self) -> Step {
        Step::Signal(self.0, self.1, self.2)
    }
    
    fn bs(&self, as_fw: &Vec<usize>) -> HashSet<usize> {
        let mut bs = HashSet::new();
        let ai = self.1;
        for i in ai..ai+self.len() {
            bs.insert(as_fw[i]);
        }
        return bs;
    }
    
    fn len(&self) -> usize {
        self.0
    }
}

fn discard_xys(n: usize) {
    for _ in 0..n {
        get_line();
    }
}

fn get_edges(n: usize, m: usize) -> Vec<HashSet<usize>> {
    let mut e: Vec<_> = (0..n).map(|_| HashSet::new()).collect();
    for _ in 0..m {
        let (u, v): (usize, usize) = get_pair();
        e[u].insert(v);
        e[v].insert(u);
    }
    return e;
}

fn dijkstra(edges: &Vec<HashSet<usize>>, start: usize, goal: usize) -> Vec<usize> {
    let mut dist: Vec<_> = (0..edges.len()).map(|_| usize::MAX).collect();
    let mut from: Vec<_> = (0..edges.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(PathState { cost: 0, position: start });
    
    while let Some(PathState { cost, position }) = heap.pop() {
        if position == goal {
            break;
        }
        if cost > dist[position] {
            continue;
        }
        for &next in edges[position].iter() {
            let next_cost = cost + 1;
            if next_cost < dist[next] {
                heap.push(PathState { cost: next_cost, position: next });
                dist[next] = next_cost;
                from[next] = position;
            }
        }
    }
    let mut path = Vec::new();
    let mut p = goal;
    while p != start {
        path.push(p);
        p = from[p];
    }
    path.reverse();
    return path;
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct PathState {
    cost: usize,
    position: usize,
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
fn select_bs(as_fw: &Vec<usize>, as_rv: &Vec<HashSet<usize>>, n: usize, path: &Vec<usize>, i: usize, lb: usize) -> Option<Signal> {
    let mut sts = as_rv[path[i]].iter().copied();
    let hd = sts.next()?;
    let mut sig = select_bs_local(hd, as_fw, as_rv, n, path, i, lb);
    for st in sts {
        let s = select_bs_local(st, as_fw, as_rv, n, path, i, lb);
        if s.len() > sig.len() {
            sig = s;
        }
    }
    return Some(sig);
}

fn select_bs_local(start: usize, as_fw: &Vec<usize>, as_rv: &Vec<HashSet<usize>>, n: usize, path: &Vec<usize>, i: usize, lb: usize) -> Signal {
    let mut u = start;
    let mut v = u + 1;
    for j in 1..(lb*2) {
        if i + j >= path.len() {
            break;
        }
        let p = path[i+j];
        let mut qs = as_rv[p].iter().copied();;
        let mut q = match qs.next() {
            Some(q) => q,
            None => break,
        };
        for c in qs {
            if c < u {
                if u - c < u - q {
                    q = c;
                }
            } else if c < v {
                q = c;
            } else {
                if c - v < c - q {
                    q = c;
                }
            }
        }
        if q < u {
            if v - q <= lb {
                u = q;
            } else {
                break;
            }
        } else if v <= q {
            if q + 1 - u <= lb {
                v = q + 1;
            } else {
                break;
            }
        } else {
            continue;
        }
    }
    let mut bs = HashSet::new();
    for j in u..v {
        bs.insert(as_fw[j]);
    }
    return Signal(v - u, u, 0);
}

#[allow(dead_code)]
fn rev_as(v: &Vec<usize>, n: usize) -> Vec<HashSet<usize>> {
    let mut r = vec![HashSet::new(); n];
    for (i, x) in v.iter().copied().enumerate() {
        r[x].insert(i);
    }
    return r;
}

#[allow(dead_code)]
fn rev_unique(v: &Vec<usize>, n: usize) -> Vec<usize> {
    let mut r = vec![usize::MAX; n];
    for (i, x) in v.iter().copied().enumerate() {
        r[x] = i;
    }
    return r;
}

#[allow(dead_code)]
fn greedy_as(path: &Vec<usize>) -> Vec<usize> {
    let mut as_fw = Vec::new();
    let mut as_yet = HashSet::new();
    for p in path.iter().copied() {
        if !as_yet.contains(&p) {
            as_fw.push(p);
            as_yet.insert(p);
        }
    }
    return as_fw;
}

#[allow(dead_code)]
fn fill_dummy(v: &Vec<usize>, l: usize) -> Vec<usize> {
    let mut f = v.clone();
    for i in v.len()..l {
        f.push(0);
    }
    return f;
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
fn get_five<T: FromStr + Copy>() -> (T, T, T, T, T) {
    let v = get_vec();
    (v[0], v[1], v[2], v[3], v[4])
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

#[allow(dead_code)]
fn solve_baseline(
    n: usize,
    edges: Vec<HashSet<usize>>,
    ts: Vec<usize>,
    la: usize,
    lb: usize,
) -> (Vec<usize>, Vec<Step>) {
    let aa: Vec<_> = (0..la).map(|i| i % n).collect();
    let mut bs: HashSet<usize> = HashSet::new();
    let mut steps: Vec<Step> = Vec::new();
    let mut pos = 0;
    for t in ts.iter().copied() {
        let path = dijkstra(&edges, pos, t);
        for p in path.iter().copied() {
            if !bs.contains(&p) {
                steps.push(signal(1, p, 0));
                bs = HashSet::new();
                bs.insert(p);
            }
            steps.push(mv(p));
            pos = p;
        }
    }
    (aa, steps)
}