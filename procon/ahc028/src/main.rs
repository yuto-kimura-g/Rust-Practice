use std::time::Instant;

use proconio::{input, marker::Chars};

const N: usize = 15;
const M: usize = 200;
const L_LIMIT: usize = 5000;

struct Position {
    y: usize,
    x: usize,
}
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.y, self.x)
    }
}

struct Input {
    s: Position,
    a: Vec<Vec<char>>,
    t: Vec<Vec<char>>,
}
impl Input {
    fn new() -> Self {
        input! {
            n: usize,
            m: usize,
            sy: usize,
            sx: usize,
            a: [Chars; n],
            t: [Chars; m],
        }
        assert!(n == N && m == M);
        Input {
            s: Position { y: sy, x: sx },
            a,
            t,
        }
    }
}

struct Output {
    ans_yx: Vec<Position>,
}
impl Output {
    fn write(&self) {
        assert!(self.ans_yx.len() <= L_LIMIT);
        for ans_yx in self.ans_yx.iter() {
            assert!(ans_yx.y < N && ans_yx.x < N);
            println!("{}", ans_yx);
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    ans_t: Vec<usize>,
    ans_t_head: Vec<usize>,
    ans_t_tail: Vec<usize>,
    score: i64,
}
impl State {
    fn new() -> Self {
        State {
            ans_t: Vec::new(),
            ans_t_head: Vec::new(),
            ans_t_tail: Vec::new(),
            score: 0,
        }
    }

    fn eval_score(&mut self, input: &Input) -> i64 {
        self.score
    }
}

fn solve(input: Input) -> Output {
    let start_time = Instant::now();
    let mut state = State::new();
    let init_score = state.eval_score(&input);
    let mut best_state = state.clone();
    let mut best_score = init_score;

    let mut iter_cnt: i64 = 0;
    loop {
        iter_cnt += 1;
        if (iter_cnt & ((1 << 4) - 1)) == 0 {
            // ref: terry_u16さんの実装(https://atcoder.jp/contests/ahc028/submissions/49221892)
            // 16回に1回，温度を更新
            // 剰余演算（iter%16==0）は遅いので，シフト演算を使って高速化
            let elaplsed = start_time.elapsed().as_millis();
            eprintln!("elaplsed:{}[ms]", elaplsed);
        }
        if iter_cnt == 100 {
            break;
        }
    }

    eprintln!("=============== annealing ===============");
    eprintln!("iter cnt\t:{}", iter_cnt);
    eprintln!("init score\t:{}", init_score);
    eprintln!("best score\t:{}", best_score);
    eprintln!("=========================================");

    Output {
        ans_yx: vec![Position {
            y: input.s.y,
            x: input.s.x,
        }],
    }
}

fn main() {
    eprintln!("Hello, AHC028");
    let input = Input::new();
    let output = solve(input);
    output.write();
}
