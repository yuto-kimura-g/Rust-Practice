use proconio::input;
use std::process::exit;

fn main() {
    let input = Input::new();
    f(&input, 0, 0, (0, 0));
}

struct Input {
    n: usize,
    p: Vec<Vec<usize>>,
    r: Vec<Vec<usize>>,
    d: Vec<Vec<usize>>,
}
impl Input {
    fn new() -> Self {
        input! {
            n: usize,
            p: [[usize; n]; n],
            r: [[usize; n - 1]; n],
            d: [[usize; n]; n - 1],
        }
        Self { n, p, r, d }
    }
}

const DYDX: [(usize, usize); 3] = [(0, 1), (1, 0), (0, 0)];
fn f(input: &Input, action_cnt: usize, money: usize, (y, x): (usize, usize)) {
    if (y, x) == (input.n - 1, input.n - 1) {
        println!("{}", action_cnt);
        exit(0);
    }
    for &(dy, dx) in DYDX.iter() {
        let (ny, nx) = (y + dy, x + dx);
        if ny >= input.n || nx >= input.n {
            continue;
        }
        match (dy, dx) {
            (0, 1) => {
                let rr = input.r[y][x];
                if money >= rr {
                    f(input, action_cnt + 1, money - rr, (ny, nx));
                }
            }
            (1, 0) => {
                let dd = input.d[y][x];
                if money >= dd {
                    f(input, action_cnt + 1, money - dd, (ny, nx));
                }
            }
            (0, 0) => {
                f(input, action_cnt + 1, money + input.p[y][x], (ny, nx));
            }
            _ => unreachable!(),
        }
    }
}
