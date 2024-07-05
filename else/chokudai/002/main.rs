use std::collections::HashSet;

const MAX: i64 = 10_i64.pow(9);

fn main() {
    let mut ans = Vec::new();
    for i in 13..1000 {
        if ans.len() == 100 {
            break;
        }
        if !p(i) {
            continue;
        }
        ans.push(f(i));
    }
    for a in ans.iter() {
        println!("{}", a);
    }
}

fn p(x: i64) -> bool {
    for i in 2..x {
        if i * i > x {
            break;
        }
        if x % i == 0 {
            return false;
        }
    }
    return true;
}

fn f(x: i64) -> i64 {
    let mut y = 1;
    let mut s = 0;
    for a in 3..15 {
        let aa = 2_i64.pow(a);
        for b in 2..10 {
            let bb = 3_i64.pow(b);
            if aa * bb * x > MAX {
                break;
            }
            for c in 1..10 {
                let cc = 5_i64.pow(c);
                if aa * bb * cc * x > MAX {
                    break;
                }
                for d in 0..3 {
                    let dd = 7_i64.pow(d);
                    if aa * bb * cc * dd * x > MAX {
                        break;
                    }
                    let rem = MAX / (aa * bb * cc * dd * x);
                    let (e, ee) = g(rem as u32, 11);
                    let ls = (a+1) * (b+1) * (c+1) * (d+1) * (e+1);
                    if ls > s {
                        s = ls;
                        y = aa * bb * cc * dd * ee * x;
                    }
                }
            }
        }
    }
    return y;
}

fn g(x: u32, b: i64) -> (u32, i64) {
    let mut y = 0;
    for i in 1..x {
        if b.pow(i) > x.into() {
            break;
        }
        y = i;
    }
    return (y, b.pow(y));
}
