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
    
    let (aa, steps) = solve(n, edges, ts, la, lb);
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

fn solve(
    n: usize,
    edges: Vec<HashSet<usize>>,
    ts: Vec<usize>,
    la: usize,
    lb: usize,
) -> (Vec<usize>, Vec<Step>) {
    let aa: Vec<_> = (0..la).map(|i| i % la).collect();
    let mut bs: HashSet<usize> = HashSet::new();
    let mut pos = 0;
    let path = dijkstra(edges, pos, ts[0]);
    println!("{:?}", path);
    (Vec::new(), Vec::new())
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

fn dijkstra(edges: Vec<HashSet<usize>>, start: usize, goal: usize) -> Vec<usize> {
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