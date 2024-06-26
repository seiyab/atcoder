use std::io::stdin;
use std::str::FromStr;
use std::cmp::min;

fn main() {
    let n: usize = get_one();
    let hs = get_hs(n);
    let mut env = Env::new(n, hs);
    loop {
        let h = env.height();
        if h > 0 {
            env.load_up(h);
        } else if h < 0 && env.load > 0 {
            env.load_down(min(env.load, -h));
        }
        if env.y == n-1 && env.x == 0 {
            break;
        }
        env.go();
    }
    for h in env.history {
        println!("{}", h);
    }
}

struct Env {
    n: usize,
    hs: Vec<Vec<i64>>,
    dir: Direction,
    x: usize,
    y: usize,
    load: i64,
    
    history: Vec<String>,
}

impl Env {
    fn new(n: usize, hs: Vec<Vec<i64>>) -> Env {
        Env {
            n: n,
            hs: hs,
            dir: Direction::Right,
            x: 0,
            y: 0,
            load: 0,
            history: Vec::new(),
        }
    }
    
    fn height(&self) -> i64 {
        self.hs[self.y][self.x]
    }
    
    fn load_up(&mut self, d: i64) {
        self.load += d;
        self.hs[self.y][self.x] -= d;
        self.history.push(format!("+{}", d));
    }
    
    fn load_down(&mut self, d: i64) {
        self.load -= d;
        self.hs[self.y][self.x] += d;
        self.history.push(format!("-{}", d));
    }
    
    fn go(&mut self) {
        match self.dir {
            Direction::Right => {
                if self.x == self.n-1 {
                    self.dir = Direction::Left;
                    self.y += 1;
                    self.history.push("D".to_string());
                } else {
                    self.x += 1;
                    self.history.push("R".to_string());
                }
            }
            Direction::Left => {
                if self.x == 0 {
                    if self.y == self.n-1 {
                        return;
                    }
                    self.dir = Direction::Right;
                    self.y += 1;
                    self.history.push("D".to_string());
                } else {
                    self.x -= 1;
                    self.history.push("L".to_string());
                }
            }
        }
    }
}

enum Direction {
    Left,
    Right,
}

fn get_hs(n: usize) -> Vec<Vec<i64>> {
    let mut hs = Vec::new();
    for _ in 0..n {
        hs.push(get_vec());
    }
    return hs;
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
