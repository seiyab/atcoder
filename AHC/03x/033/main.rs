use std::io::stdin;
use std::str::FromStr;
use std::env;

fn main() {
    let n: usize = get_one();
    let a: Vec<Vec<usize>> = get_square(n);
    let mut s = State::new(n, &a);
    s.post_step();
    s.step_big(&Operation::Nop);
    s.post_step();
    let mut os = Vec::new();
    os.push(Operation::Nop);
    
    loop {
        if s.expected.iter().all(|e| *e == None) {
            break;
        }
        for p in plan_pick(&s).iter() {
            s.step_big(p);
            s.post_step();
            os.push(*p);
        }
        let g = match s.big_crane.grabbing {
            Some(g) => g,
            None => continue,
        };
        match s.expected.iter().position(|e| *e == Some(g)) {
            Some(d) => {
                for p in path(&s.big_crane.pos(), &(s.field[d].len()-1, d)).iter() {
                    s.step_big(p);
                    s.post_step();
                    os.push(*p);
                }
                s.step_big(&Operation::Release);
                s.post_step();
                os.push(Operation::Release);
            },
            None => {
                for p in plan_put(&s).iter() {
                    s.step_big(p);
                    s.post_step();
                    os.push(*p);
                }
            },
        };
    }

    println!("{}", format_ops(&os));
    for _ in 1..s.n {
        println!("B");
    }
}


fn plan_pick(s: &State) -> Vec<Operation> {
    let mut ct: Option<usize> = None;
    let mut src: Option<(usize, usize)> = None;
    let mut depth: usize = 10_000;
    for i in 0..s.n {
        let e = match s.expected[i] {
            Some(e) => e,
            None => continue,
        };
        for u in 0..s.field.len() {
            for v in 0..s.field[u].len() {
                if s.field[u][v] == Some(e) {
                    ct = Some(e);
                    src = Some((v, u));
                    break;
                }
            }
            if ct != None {
                break;
            }
        }
        for u in 0..s.awaiting.len() {
            for h in 0..s.awaiting[u].len() {
                let d = s.awaiting[u].len() - h;
                if d > depth {
                    break;
                }
                if s.awaiting[u][h] == e {
                    ct = Some(e);
                    src = Some((0, u));
                    depth = d;
                }
            }
        }
    }
    let mut ops = path(&s.big_crane.pos(), &src.unwrap());
    ops.push(Operation::Grab);
    return ops;
}

fn plan_put(s: &State) -> Vec<Operation> {
    let mut dst: (usize, usize) = (0, 0);
    for i in 0..s.field.len() {
        for j in 1..s.field[i].len()-1 {
            if s.field[i][j] == None {
                dst = (j, i);
                break;
            }
        }
        if dst != (0, 0) {
            break;
        }
    }
    let mut ops = path(&s.big_crane.pos(), &dst);
    ops.push(Operation::Release);
    return ops;
}

fn path(src: &(usize, usize), dst: &(usize, usize)) -> Vec<Operation> {
    let mut ops = Vec::new();
    let (sx, sy) = src;
    let (dx, dy) = dst;
    if sx < dx {
        for _ in 0..dx-sx {
            ops.push(Operation::Right);
        }
    } else {
        for _ in 0..sx-dx {
            ops.push(Operation::Left);
        }
    }
    if sy < dy {
        for _ in 0..dy-sy {
            ops.push(Operation::Down);
        }
    } else {
        for _ in 0..sy-dy {
            ops.push(Operation::Up);
        }
    }
    return ops;
}

struct State {
    n: usize,
    field: Vec<Vec<Option<usize>>>,
    awaiting: Vec<Vec<usize>>,
    expected: Vec<Option<usize>>,
    big_crane: Crane,
    small_cranes: Vec<Option<Crane>>,
}

impl State {
    fn new(n: usize, a: &Vec<Vec<usize>>) -> State {
        let mut e = a.clone();
        for i in 0..n {
            e[i].reverse();
        }
        return State {
            n: n,
            field: vec![vec![None; n]; n],
            awaiting: e,
            expected: (0..n).map(|i| { Some(n * i) }).collect(),
            big_crane: Crane {x: 0, y: 0, grabbing: None},
            small_cranes: (1..n).map(|i| {
                Some(Crane {
                    x: 0,
                    y: i,
                    grabbing: None,
                })
            }).collect(),
        }
    }
    
    fn dbg(&self) {
        for i in 0..self.field.len() {
            for j in 0..self.field[i].len() {
                print!("{} ", match self.field[i][j] {
                    Some(x) => x.to_string(),
                    None => ".".to_string(),
                });
            }
            println!();
        }
        println!("big: {:?}", self.big_crane.grabbing);
        for i in 0..self.expected.len() {
            print!("{} ", match self.expected[i] {
                Some(x) => x.to_string(),
                None => ".".to_string(),
            });
        }
        println!();
    }
    
    fn step_big(&mut self, op: &Operation) {
        let mut v = vec![Operation::Nop; self.n];
        v[0] = op.clone();
        self.step(&v);
    }
    
    fn step(&mut self, ops: &Vec<Operation>) {
        let bo = ops[0];
        let (y, x) = (self.big_crane.x, self.big_crane.y);
        match bo {
            Operation::Grab => {
                self.big_crane.grabbing = self.field[x][y];
                self.field[x][y] = None;
            },
            Operation::Release => {
                self.field[x][y] = self.big_crane.grabbing;
                self.big_crane.grabbing = None;
            },
            Operation::Up => {
                self.big_crane.y = self.big_crane.y - 1;
            },
            Operation::Down => {
                self.big_crane.y = self.big_crane.y + 1;
            },
            Operation::Left => {
                self.big_crane.x = self.big_crane.x - 1;
            },
            Operation::Right => {
                self.big_crane.x = self.big_crane.x + 1;
            },
            Operation::Nop => {},
            _ => {},
        }
        
        for i in 0..self.small_cranes.len() {
            let cr = match self.small_cranes[i].as_mut() {
                Some(c) => c,
                None => continue,
            };
            let so = ops[i+1];
            let (x, y) = (cr.x, cr.y);
            match so {
                Operation::Grab => {
                    cr.grabbing = self.field[x][y];
                    self.field[x][y] = None;
                },
                Operation::Release => {
                    self.field[x][y] = cr.grabbing;
                    cr.grabbing = None;
                },
                Operation::Up => {
                    cr.y = y - 1;
                },
                Operation::Down => {
                    cr.y = y + 1;
                },
                Operation::Left => {
                    cr.x = x - 1;
                },
                Operation::Right => {
                    cr.x = x + 1;
                },
                Operation::Bomb => {
                    self.small_cranes[i] = None;
                },
                _ => {},
            }
        }
    }
    
    fn post_step(&mut self) {
        for i in 0..self.field.len() {
            if self.field[i][0] != None {
                continue
            }
            self.field[i][0] = self.awaiting[i].pop();
        }
        
        for i in 0..self.field.len() {
            let c = self.field[i][self.field[i].len() - 1];
            if let Some(d) = c {
                if Some(d) != self.expected[i] {
                    panic!("unexpected delivery");
                }
                let r = self.field[i].len() - 1;
                self.field[i][r] = None;
                if d+1 < self.n * (i+1) {
                    self.expected[i] = Some(d + 1);
                } else {
                    self.expected[i] = None;
                }
            }
        }
        
        if env::var("DEBUG") == Ok("1".to_string()) {
            self.dbg();
            println!();
        }
    }
}

struct Crane {
    x: usize,
    y: usize,
    grabbing: Option<usize>,
}

impl Crane {
    fn pos(&self) -> (usize, usize) {
        return (self.x, self.y);
    }
}

#[derive(Clone, Copy)]
enum Operation {
    Grab,
    Release,
    Up,
    Down,
    Left,
    Right,
    Nop,
    Bomb,
}

fn format_ops(ops: &Vec<Operation>) -> String {
    let mut s = String::with_capacity(ops.len());
    for o in ops.iter() {
        s.push(match o {
            Operation::Grab => 'P',
            Operation::Release => 'Q',
            Operation::Up => 'U',
            Operation::Down => 'D',
            Operation::Left => 'L',
            Operation::Right => 'R',
            Operation::Nop => '.',
            Operation::Bomb => 'B',
        });
    }
    return s;
}

#[allow(dead_code)]
fn get_square(n: usize) -> Vec<Vec<usize>> {
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