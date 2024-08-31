use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::stdin;
use std::str::FromStr;
use std::collections::HashSet;
use std::collections::HashMap;
use std::env;

use rand::prelude::*;

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
    let start = std::time::Instant::now();
    let fast = env::var("FAST") == Ok("1".to_string());
    let hub = pickup_hub_nodes(edges, 10);
    let paths = partitioned_path(edges, &hub, ts, la, lb);
    let mut path = Vec::new();
    for p in paths.iter() {
        path.extend(p);
    }
    let (mut cs, mut steps) = solve_for_fixed_path(&path, n, la, lb);
    
    let mut loss = eval(&steps);
    let mut rng = thread_rng();
    for i in 0..40 {
        if fast && i > 4 {
            break;
        }
        if start.elapsed().as_millis() > 1500 {
            break;
        }

        let new_paths = suggest_paths(&mut rng, &paths, edges, &hub, ts, la, lb);
        let mut new_path = Vec::new();
        for p in new_paths.iter() {
            new_path.extend(p);
        }
        let (new_cs, new_steps) = solve_for_fixed_path(&new_path, n, la, lb);
        let l = eval(&new_steps);
        if l <= loss {
            cs = new_cs;
            steps = new_steps;
            loss = l;
        }
    }
    return (cs, steps)
}

fn partitioned_path(
    edges: &Vec<HashSet<usize>>,
    hub_nodes: &HashSet<usize>,
    ts: &Vec<usize>,
    la: usize,
    lb: usize,
) -> Vec<Vec<usize>> {
    let mut paths = Vec::new();
    let mut pos = 0;
    let mut frequent_edges = HashSet::new();
    for (i, t) in ts.iter().copied().enumerate() {
        let p = dijkstra(&edges, &frequent_edges, hub_nodes, pos, t, lb);
        if i % 100 == 50 {
            frequent_edges = pickup_frequent_edges(&paths, la);
        }
        paths.push(p);
        pos = t;
    }
    return paths;
}

fn eval(steps: &Vec<Step>) -> usize {
    return steps.iter().filter(|a| match a {
        &&Step::Signal(_, _, _) => true,
        _ => false,
    }).count();
}

fn solve_for_fixed_path(
    path: &Vec<usize>,
    n: usize,
    la: usize,
    lb: usize,
) -> (Vec<usize>, Vec<Step>) {
    let as_fw = greedy_as(&path, la, lb);
    let as_rv = rev_as(&as_fw, n);
    let mut bs: HashSet<usize> = HashSet::new();
    let mut bs_arr = vec![usize::MAX; lb];
    let mut steps: Vec<Step> = Vec::new();
    let mut bs_i_next = 0;
    for i in 0..path.len() {
        let p = path[i];
        if !bs.contains(&p) {
            let mut sig = select_bs(&as_fw, &as_rv, &path, i, lb).unwrap();
            if bs_i_next + sig.len() > lb {
                bs_i_next = 0;
            }
            sig = sig.at(bs_i_next);

            steps.push(sig.step());
            for i in 0..sig.len() {
                bs_arr[i + sig.2] = as_fw[i + sig.1];
            }
            bs = bs_arr.iter().copied().collect();
            bs_i_next += sig.len();
        }
        steps.push(mv(p));
    }
    return (fill_dummy(&as_fw, la), steps);
}

fn suggest_paths(
    rng: &mut ThreadRng,
    paths: &Vec<Vec<usize>>,
    edges: &Vec<HashSet<usize>>,
    hub_nodes: &HashSet<usize>,
    ts: &Vec<usize>,
    la: usize,
    lb: usize,
) -> Vec<Vec<usize>> {
    let mut new_paths = vec![Vec::new(); paths.len()];
    let breaks = sample_indices(rng, paths.len(), 400);
    for (i, p) in paths.iter().enumerate() {
        if breaks.contains(&i) {
            new_paths[i] = Vec::new();
            continue;
        }
        new_paths[i] = p.clone();
    }
    let mut frequent_edges = pickup_frequent_edges(&new_paths, la);
    
    let mut done = HashSet::new();
    for i in breaks.iter().copied() {
        if done.len() % 10 == 9 {
            frequent_edges = pickup_frequent_edges(&new_paths, la);
        }
        let start = if i == 0 { 0 } else { ts[i-1] };
        let end = ts[i];
        let p = dijkstra(edges, &frequent_edges, hub_nodes, start, end, lb);
        new_paths[i] = p;
        done.insert(i);
    }
    return new_paths;
}

fn sample_indices(rng: &mut ThreadRng, n: usize, k: usize) -> HashSet<usize> {
    let mut v: Vec<_> = (0..n).collect();
    v.shuffle(rng);
    return v.iter().take(k).copied().collect();
}

enum Step {
    Signal(usize, usize, usize),
    Move(usize),
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
    
    fn len(&self) -> usize {
        self.0
    }
    
    #[allow(dead_code)]
    fn right(&self, lb: usize) -> Signal {
        Signal(self.0, self.1, lb - self.0)
    }
    
    fn at(&self, i: usize) -> Signal {
        Signal(self.0, self.1, i)
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

fn pickup_frequent_edges(paths: &Vec<Vec<usize>>, la: usize) -> HashSet<NormalizedEdge> {
    let size = la * 2 / 3;
    let mut freq = HashMap::new();
    for p in paths.iter() {
        if p.len() < 2 {
            continue;
        }
        for i in 0..p.len()-1 {
            let e = NormalizedEdge::from((p[i], p[i+1]));
            freq.entry(e).and_modify(|c| *c += 1).or_insert(1);
        }
    }
    return freq.keys().filter(|k| freq.get(k).unwrap_or(&0) > &2).take(size).cloned().collect();
}

fn pickup_hub_nodes(edges: &Vec<HashSet<usize>>, size: usize) -> HashSet<usize> {
    let r = node_ranking(edges);
    return r.iter().take(size).copied().collect();
}

fn node_ranking(edges: &Vec<HashSet<usize>>) -> Vec<usize> {
    let v = eigenvector(edges);
    let mut r: Vec<_> = (0..edges.len()).collect();
    r.sort_by(|&i, &j| v[j].partial_cmp(&v[i]).unwrap_or(Ordering::Equal));
    return r;
}

fn eigenvector(edges: &Vec<HashSet<usize>>) -> Vec<f64> {
    let mut v = vec![1.0; edges.len()];
    let mut w = vec![0.0; edges.len()];
    for _ in 0..10 {
        for i in 0..edges.len() {
            w[i] = edges[i].iter().map(|&j| v[j]).sum();
        }
        let s = w.iter().sum::<f64>();
        for i in 0..edges.len() {
            v[i] = w[i] / s;
        }
    }
    return v;
}

fn dijkstra(
    edges: &Vec<HashSet<usize>>,
    visited_edges: &HashSet<NormalizedEdge>,
    hub_nodes: &HashSet<usize>,
    start: usize,
    goal: usize,
    lb: usize
) -> Vec<usize> {
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
            let e = NormalizedEdge::from((position, next));
            let cost_delta = if visited_edges.contains(&e)  {
                if lb > 15 { 5 } else if lb > 8 { 20 } else  { 40 }
            } else if hub_nodes.contains(&next){ 55 }
                else { 100 };
            let next_cost = cost + cost_delta;
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

#[derive(PartialEq, Eq, Hash, Clone)]
struct NormalizedEdge(usize, usize);

impl From<(usize, usize)> for NormalizedEdge {
    fn from((u, v): (usize, usize)) -> Self {
        if u < v {
            NormalizedEdge(u, v)
        } else {
            NormalizedEdge(v, u)
        }
    }
}

#[allow(dead_code)]
fn select_bs_2(as_fw: &Vec<usize>, as_rv: &Vec<Vec<usize>>, path: &Vec<usize>, i: usize, lb: usize) -> Option<(Signal, usize)> {
    let mut sts = as_rv[path[i]].iter().copied();
    let hd = sts.next()?;
    let (mut sig, mut score) = select_bs_local(hd, as_fw, as_rv, path, i, lb);
    for st in sts {
        let (sg, sc) = select_bs_local(st, as_fw, as_rv, path, i, lb);
        if sc > score {
            sig = sg;
            score = sc;
        }
    }
    return Some((sig, score));
}

#[allow(dead_code)]
fn select_bs(as_fw: &Vec<usize>, as_rv: &Vec<Vec<usize>>, path: &Vec<usize>, i: usize, lb: usize) -> Option<Signal> {
    let mut sts = as_rv[path[i]].iter().copied();
    let hd = sts.next()?;
    let (mut sig, mut score) = select_bs_local(hd, as_fw, as_rv, path, i, lb);
    for st in sts {
        let (sg, sc) = select_bs_local(st, as_fw, as_rv, path, i, lb);
        if sc > score {
            sig = sg;
            score = sc;
        }
    }
    return Some(sig);
}

fn select_bs_local(start: usize, as_fw: &Vec<usize>, as_rv: &Vec<Vec<usize>>, path: &Vec<usize>, i: usize, lb: usize) -> (Signal, usize) {
    let mut u = start;
    let mut v = u + 1;
    let mut score = 0;
    for j in 1..(lb*2) {
        if i + j >= path.len() {
            break;
        }
        let p = path[i+j];
        let mut qs = as_rv[p].iter().copied();
        let mut q = match qs.next() {
            Some(q) => q,
            None => break,
        };
        let mut loss = if q < u { u - q } else { q - u };
        for c in qs {
            if c < u {
                let l = u - c;
                if l < loss {
                    q = c;
                    loss = l;
                }
            } else if c < v {
                q = c;
                loss = 0;
            } else {
                let l = c + 1 - v;
                if l < loss {
                    q = c;
                    loss = l;
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
        }
        score += 1;
    }
    let mut bs = HashSet::new();
    for j in u..v {
        bs.insert(as_fw[j]);
    }
    return (Signal(v - u, u, 0), score);
}

#[allow(dead_code)]
fn rev_as(v: &Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    let mut r = vec![Vec::new(); n];
    for (i, x) in v.iter().copied().enumerate() {
        r[x].push(i);
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

#[derive(PartialEq, Eq, Hash)]
struct NormalizedQuad(usize, usize, usize, usize);

impl From<(usize, usize, usize, usize)> for NormalizedQuad {
    fn from((a, b, c, d): (usize, usize, usize, usize)) -> Self {
        let mut v = vec![a, b, c, d];
        v.sort();
        return Self(v[0], v[1], v[2], v[3]);
    }
}

#[allow(dead_code)]
fn greedy_as(path: &Vec<usize>, la: usize, lb: usize) -> Vec<usize> {
    let mut as_fw = Vec::new();
    let mut as_rv = vec![Vec::new(); 600];
    let mut as_yet: HashSet<_> = path.iter().copied().collect();
    let buf_len = lb;
    let mut skip_until = 0;
    for i in 0..path.len() {
        let p = path[i];
        if as_yet.contains(&p) {
            as_rv[p].push(as_fw.len());
            as_fw.push(p);
            as_yet.remove(&p);
        } else {
            if i < skip_until {
                continue;
            }
            if let Some((_, score)) = select_bs_2(&as_fw, &as_rv, path, i, lb) {
                if score >= lb / 2 + 1 {
                    skip_until = i + score;
                    continue;
                }
            }

            if as_yet.len() < la - as_fw.len() {
                let l = as_fw.len();
                let st = if l < buf_len { 0 } else { l - buf_len };
                if !(st..l).any(|j| as_fw[j] == p) {
                    as_rv[p].push(as_fw.len());
                    as_fw.push(p);
                }
            }
        }
    }
    return as_fw;
}

#[allow(dead_code)]
fn naive_greedy_as(path: &Vec<usize>) -> Vec<usize> {
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
    for _ in v.len()..l {
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
