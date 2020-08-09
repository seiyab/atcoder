use std::hash::Hash;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::io::stdin;
use std::str::FromStr;

fn main(){
    let (n, m): (usize, usize) = get_pair();
    let (s, t): (u64, u64) = get_pair();
    let mut graph = HashMap::new();
    for _ in 0..m {
      let (x, y, d): (u64, u64, u64) = get_triple();
      graph.entry(x).or_insert(BTreeMap::new()).insert(y, d);
      graph.entry(y).or_insert(BTreeMap::new()).insert(x, d);
    }

    if let Some(ans) = solve(&graph, s, t, n) {
      println!("{}", ans);
    } else {
      println!("{}", -1);
    }
}

fn solve(g: &Graph<u64>, s: u64, t: u64, n: usize) -> Option<u64> {
  let from_s = dijkstra(g, s);
  let from_t = dijkstra(g, t);
  for uu in 1..n+1 {
    let u = uu as u64;
    if let Some(a) = from_s.get(&u) {
      if let Some(b) = from_t.get(&u) {
        if a == b { 
          return Some(u);
        }
      }
    }
  }
  None
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


type Graph<N> = HashMap<N, BTreeMap<N, u64>>;

#[derive(PartialEq, Eq)]
struct RevOrd<M: Ord + Eq>(M);

#[allow(dead_code)]
impl <M: Ord + Eq> PartialOrd for RevOrd<M> {
    fn partial_cmp(&self, other: &RevOrd<M>) -> Option<Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

#[allow(dead_code)]
impl <M: Ord + Eq> Ord for RevOrd<M> {
    fn cmp(&self, other: &RevOrd<M>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

#[allow(dead_code)]
struct MinHeap<M: Ord>(BinaryHeap<RevOrd<M>>);

#[allow(dead_code)]
impl <M: Ord> MinHeap<M> {
    fn new() -> MinHeap<M> {
        MinHeap(BinaryHeap::new())
    }

    fn push(&mut self, elm: M) {
        self.0.push(RevOrd(elm))
    }

    fn pop(&mut self) -> Option<M> {
        match self.0.pop() {
            Some(RevOrd(elm)) => Some(elm),
            _ => None
        }
    }
}

fn dijkstra<N>(graph: &Graph<N>, start: N) -> HashMap<N, u64>
    where N: Eq + Ord + Hash + Copy
{
    let mut heap = MinHeap::new();
    let mut visited = HashMap::new();

    heap.push((0, start));

    while let Some((cost, current)) = heap.pop() {
        if visited.contains_key(&current) { continue; }
        // if current == end { return Some(cost); }
        visited.insert(current, cost);
        if let Some(edges) = graph.get(&current) {
            for (next, path_cost) in edges {
              if !visited.contains_key(&next) {
                heap.push((cost + *path_cost, next.clone()))
              }
            }
        }
    }
    visited
}
