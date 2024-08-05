use std::io::stdin;
use std::str::FromStr;
use std::env;

fn main() {
    let n: usize = get_one();
    let a: Vec<Vec<usize>> = get_square(n);
    let mut s = State::new(n, &a);
    let mut os_big = Vec::new();
    let mut os_center = Vec::new();
    let mut os_right = Vec::new();
    
    for _ in 0..3_000 {
        s.post_step();
        if s.expected.iter().all(|e| *e == None) {
            break;
        }
        
        if s.big_crane.target == None && s.big_crane.grabbing == None {
            s.big_crane.target = target_big(&s);
        }
        let cc = s.small_cranes[0].unwrap();
        if cc.target == None &&
            cc.x == 2 &&
            cc.grabbing == None
        {
            let t = target_center(&s);
            if let Some(tc) = s.small_cranes[0].as_mut() {
                tc.target = t;
            }
        }
        let rc = s.small_cranes[1].unwrap();
        if rc.target == None &&
            rc.x == 4 &&
            rc.grabbing == None
        {
            let t = target_right(&s);
            if let Some(rc) = s.small_cranes[1].as_mut() {
                rc.target = t;
            }
        }
        
        let bo = op_big(&s);
        let co = op_center(&s);
        let ro = op_right(&s);
        
        if bo == Operation::Nop && co == Operation::Nop && ro == Operation::Nop {
            // panic!("stopped");
            break;
        }
        
        os_big.push(bo);
        os_center.push(co);
        os_right.push(ro);
        s.step(&vec![bo, co, ro, Operation::Nop, Operation::Nop]);
    }
    if s.expected.iter().any(|e| *e != None) {
        s.post_step();
        if s.small_cranes[0].unwrap().grabbing != None || s.small_cranes[1].unwrap().grabbing != None {
            os_big.push(Operation::Nop);
            let oc = if s.small_cranes[0].unwrap().grabbing != None {
                Operation::Release
            } else {
                Operation::Nop
            };
            os_center.push(oc);
            let or = if s.small_cranes[1].unwrap().grabbing != None {
                Operation::Release
            } else {
                Operation::Nop
            };
            os_right.push(or);
            s.step(&vec![Operation::Nop, oc, or, Operation::Nop, Operation::Nop]);
            s.post_step();
        }
        os_big.push(Operation::Nop);
        os_center.push(Operation::Bomb);
        os_right.push(Operation::Bomb);
        s.step(&vec![Operation::Nop, Operation::Bomb, Operation::Bomb, Operation::Nop, Operation::Nop]);
    }
    for _ in 0..1_000 {
        if s.expected.iter().all(|e| *e == None) {
            break;
        }
        s.post_step();
        for p in plan_pick(&s).iter() {
            s.step_big(p);
            s.post_step();
            os_big.push(*p);
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
                    os_big.push(*p);
                }
                s.step_big(&Operation::Release);
                s.post_step();
                os_big.push(Operation::Release);
            },
            None => {
                for p in plan_put(&s).iter() {
                    s.step_big(p);
                    s.post_step();
                    os_big.push(*p);
                }
            },
        }
    }

    println!("{}", format_ops(&os_big));
    println!("{}", format_ops(&os_center));
    println!("{}", format_ops(&os_right));
    for _ in 3..s.n {
        println!("B");
    }
}

fn op_big(s: &State) -> Operation {
    let cr = s.big_crane;
    if let Some(g) = cr.grabbing {
        if cr.x == 1 {
            return Operation::Release;
        }
        let mut d = 10;
        let mut y = 0;
        for i in empty_rows(s, 1).iter().copied() {
            let dd = diff(cr.y, i);
            if dd < d {
                d = dd;
                y = i;
            }
        }
        if d < 10 {
            if cr.y < y {
                return Operation::Down;
            } else if cr.y > y {
                return Operation::Up;
            } else {
                return Operation::Right;
            }
        }
        for i in empty_rows(s, 0) {
            let dd = diff(cr.y, i);
            if dd < d {
                d = dd;
                y = i;
            }
        }
        if d < 10 {
            if cr.y < y {
                return Operation::Down;
            } else if cr.y > y {
                return Operation::Up;
            } else {
                return Operation::Release;
            }
        }
        return Operation::Nop;
    } else if cr.x > 0 {
        return Operation::Left;
    } else {
        if let Some(t) = cr.target {
            for i in 0..s.n {
                if s.field[i][0] == Some(t) {
                    if i == cr.y {
                        return Operation::Grab;
                    } else if cr.y < i {
                        return Operation::Down;
                    } else {
                        return Operation::Up;
                    }
                }
            }
            panic!("target not found");
        } else {
            return Operation::Nop;
        }
    }
}

fn op_center(s: &State) -> Operation {
    let cr = s.small_cranes[0].unwrap();
    if let Some(g) = cr.grabbing {
       if cr.x < 2 {
           return Operation::Right;
       } else if cr.x == 3 {
           return Operation::Release;
       } else {
           let mut d = 10;
           let mut y = 0;
           for i in empty_rows(s, 3).iter().copied() {
               let dd = diff(cr.y, i);
               if dd < d {
                   d = dd;
                   y = i;
               }
           }
           if d == 10 {
               return Operation::Nop;
           } else if cr.y < y {
               return Operation::Down;
           } else if cr.y > y {
               return Operation::Up;
           } else {
               return Operation::Right;
           }
       }
    } else if cr.x > 2 {
        return Operation::Left;
    } else if cr.x < 2 {
        if let Some(t) = cr.target {
            if s.field[cr.y][1] == Some(t) {
                return Operation::Grab;
            } else {
                return Operation::Right;
            }
        } else {
            return Operation::Right;
        }
    } else {
        if let Some(t) = cr.target {
            for i in 0..s.n {
                if s.field[i][1] == Some(t) {
                    if i == cr.y {
                        return Operation::Left;
                    } else if cr.y < i {
                        return Operation::Down;
                    } else {
                        return Operation::Up;
                    }
                }
            }
            panic!("target not found");
        } else {
            return Operation::Nop;
        }
    }
}

fn op_right(s: &State) -> Operation {
    let cr = s.small_cranes[1].unwrap();
    if let Some(g) = cr.grabbing {
       if cr.x < 4 {
           return Operation::Right;
       } else {
           let y = s.expected.iter().position(|e| *e == Some(g)).unwrap();
            if cr.y < y {
               return Operation::Down;
           } else if cr.y > y {
               return Operation::Up;
           } else {
               return Operation::Release;
           }
       }
    } else if cr.x < 4 {
        if let Some(t) = cr.target {
            if s.field[cr.y][3] == Some(t) {
                return Operation::Grab;
            } else {
                return Operation::Right;
            }
        } else {
            return Operation::Right;
        }
    } else {
        if let Some(t) = cr.target {
            for i in 0..s.n {
                if s.field[i][3] == Some(t) {
                    if i == cr.y {
                        return Operation::Left;
                    } else if cr.y < i {
                        return Operation::Down;
                    } else {
                        return Operation::Up;
                    }
                }
            }
            panic!("target not found");
        } else {
            return Operation::Nop;
        }
    }
}

fn target_big(s: &State) -> Option<usize> {
    let mut l = 10_000_000;
    let mut t = None;
    let mut ok = false;
    for i in 0..s.n {
        if s.field[i][1] == None {
            ok = true;
            break;
        }
        if s.field[i][0] == None {
            ok = true;
            break;
        }
    }
    if !ok {
        return None;
    }
    for e in s.expected_left().iter().copied() {
        for i in 0..s.n {
            if s.field[i][0] == Some(e) {
                let loss = diff(s.big_crane.y, i);
                if loss < l {
                    l = loss;
                    t = Some(e);
                }
            }
        }
        if l < 10 {
            return t;
        }
        for i in 0..s.n {
            let n = s.awaiting[i].len();
            for j in 0..n {
                if s.awaiting[i][j] == e {
                    let mut loss = diff(s.big_crane.y, i) + 5;
                    for k in j..n {
                        loss += 2;
                        if next_expected(e, s.n) != Some(s.awaiting[i][k]) {
                            loss += 10;
                        }
                    }
                    if loss < l {
                        l = loss;
                        t = s.field[i][0];
                    }
                    break;
                }
            }
        }
    }
    return t;
}

fn empty_rows(s: &State, x: usize) -> Vec<usize> {
    let mut v = Vec::new();
    for i in 0..s.n {
        if s.field[i][x] == None {
            v.push(i);
        }
    }
    return v;
}


fn target_center(s: &State) -> Option<usize> {
    let mut l = 10_000_000;
    let mut t = None;
    let cr = s.small_cranes[0].unwrap();
    for e in s.expected_center().iter() {
        for i in 0..s.n {
            if s.field[i][1] == Some(*e) {
                let loss = diff(cr.y, i);
                if loss < l {
                    l = loss;
                    t = Some(*e);
                }
            }
        }
    }
    if t != None {
        return t;
    }
    if empty_rows(s, 3).len() <= 1 {
        return None;
    }
    if empty_rows(s, 1).len() > 3 {
        return None;
    }
    for i in 0..s.n {
        let c = match s.field[i][1] {
            Some(x) => x,
            None => continue,
        };
        let loss = diff(cr.y, i);
        if loss < l {
            l = loss;
            t = Some(c);
        }
    }

    return t;
}

fn target_right(s: &State) -> Option<usize> {
    let mut l = 10_000_000;
    let mut t = None;
    for oe in s.expected.iter() {
        let e = match oe {
            Some(x) => x,
            None => continue,
        };
        for i in 0..s.n {
            if s.field[i][3] == Some(*e) {
                let loss = diff(s.small_cranes[1].unwrap().y, i);
                if loss < l {
                    l = loss;
                    t = Some(e);
                }
            }
        }
    }
    return t.copied();
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
            big_crane: Crane {
                x: 0,
                y: 0,
                grabbing: None,
                target: None,
            },
            small_cranes: (1..n).map(|i| {
                Some(Crane {
                    x: 0,
                    y: i,
                    grabbing: None,
                    target: None,
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
        println!("big: {:?}-{:?}-{:?}", self.big_crane.grabbing, self.big_crane.target, (self.big_crane.x, self.big_crane.y));
        if let Some(sc) = self.small_cranes[0] {
            println!("center: {:?}-{:?}-{:?}", sc.grabbing, sc.target, (sc.x, sc.y));
        };
        if let Some(sr) = self.small_cranes[1] {
            println!("right: {:?}-{:?}-{:?}", sr.grabbing, sr.target, (sr.x, sr.y));
        };
        for i in 0..self.expected.len() {
            print!("{} ", match self.expected[i] {
                Some(x) => x.to_string(),
                None => ".".to_string(),
            });
        }
        println!();
    }
    
    fn expected_center(&self) -> Vec<usize> {
        let mut v = Vec::new();
        for i in 0..self.expected.len() {
            if let Some(x) = self.expected[i] {
                v.push(x);
            }
        }
        for i in 0..v.len() {
            for _ in 0..3 {
                let nx = match next_expected(v[i], self.n) {
                    Some(x) => x,
                    None => break,
                };
                for j in 0..self.n {
                    let f = match self.field[j][3] {
                        Some(x) => x,
                        None => continue,
                    };
                    if f == nx {
                        v.push(f);
                        break;
                    } 
                }
            }
        }
        return v;
    }
    
    fn expected_left(&self) -> Vec<usize> {
        let mut v = self.expected_center();
        for i in 0..v.len() {
            for _ in 0..3 {
                let nx = match next_expected(v[i], self.n) {
                    Some(x) => x,
                    None => break,
                };
                for j in 0..self.n {
                    let f = match self.field[j][1] {
                        Some(x) => x,
                        None => continue,
                    };
                    if f == nx {
                        v.push(f);
                        break;
                    } 
                }
            }
        }
        return v;
    }
    
    fn step_big(&mut self, op: &Operation) {
        let mut v = vec![Operation::Nop; self.n];
        v[0] = op.clone();
        self.step(&v);
    }
    
    fn step(&mut self, ops: &Vec<Operation>) {
        let bo = ops[0];
        let (x, y) = (self.big_crane.x, self.big_crane.y);
        match bo {
            Operation::Grab => {
                self.big_crane.grabbing = self.field[y][x];
                self.big_crane.target = None;
                self.field[y][x] = None;
            },
            Operation::Release => {
                self.field[y][x] = self.big_crane.grabbing;
                self.big_crane.grabbing = None;
            },
            Operation::Up => {
                self.big_crane.y = y - 1;
            },
            Operation::Down => {
                self.big_crane.y = y + 1;
            },
            Operation::Left => {
                self.big_crane.x = x - 1;
            },
            Operation::Right => {
                self.big_crane.x = x + 1;
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
                    cr.grabbing = self.field[y][x];
                    cr.target = None;
                    self.field[y][x] = None;
                },
                Operation::Release => {
                    self.field[y][x] = cr.grabbing;
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
                self.expected[i] = next_expected(d, self.n);
            }
        }
        
        if env::var("DEBUG") == Ok("1".to_string()) {
            self.dbg();
            println!();
        }
    }
}

fn next_expected(c: usize, n: usize) -> Option<usize> {
    let cd = c + 1;
    if cd % n == 0 {
        return None;
    }
    return Some(cd);
}

fn diff(a: usize, b: usize) -> usize {
    if a > b {
        return a - b;
    }
    return b - a;
}

#[derive(Clone, Copy)]
struct Crane {
    x: usize,
    y: usize,
    grabbing: Option<usize>,
    target: Option<usize>,
}

impl Crane {
    fn pos(&self) -> (usize, usize) {
        return (self.x, self.y);
    }
}

#[derive(Clone, Copy, PartialEq)]
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