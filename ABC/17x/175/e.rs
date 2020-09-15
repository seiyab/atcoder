use std::io::stdin;
use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::max;

fn main(){
    let (r, c, k): (usize, usize, usize) = get_triple();
    let rcv = {
        let mut rcv = HashMap::new();
        for _ in 0..k {
            let (r, c, v): (usize, usize, usize) = get_triple();
            rcv.entry(r).or_insert(HashMap::new()).insert(c, v);
        }
        rcv
    };
    let mut field = {
        let mut field = Vec::new();
        for _ in 0..r+2 {
            let mut row = Vec::new();
            for _ in 0..c+2 {
                row.push(vec![0; 4]);
            }
            field.push(row);
        }
        field
    };

    for i in 1..r+1 {
        for j in 1..c+1 {
            for p in 0..4 {
                field[i][j+1][p] = max(field[i][j+1][p], field[i][j][p]);
                field[i+1][j][0] = max(field[i+1][j][0], field[i][j][p]);

                if p == 3 { continue; }
                if let Some(v) = rcv.get(&i).and_then(|cv| cv.get(&j)) {
                    field[i][j+1][p+1] = max(field[i][j+1][p+1], field[i][j][p] + v);
                    field[i+1][j][0] = max(field[i+1][j][0], field[i][j][p] + v);
                }
            }
        }
    }

    let v = rcv.get(&r).and_then(|cv| cv.get(&c)).unwrap_or(&0);

    println!("{}", max(field[r][c][0] + v, max(field[r][c][1] + v, max(field[r][c][2] + v, field[r][c][3]))));
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
