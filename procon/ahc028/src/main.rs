use my_util::*;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    let input = Input::new();
    let output = solver::solve(input);
    output.write();
}

/// グリッドの大きさ
const N: usize = 15;
/// 文字列の個数
const M: usize = 200;
/// 操作回数の上限
const L_LIMIT: usize = 5000;
/// 実行時間制限 [msec]
const TIME_LIMIT: f64 = 1800.0;
// const TIME_LIMIT: f64 = 18000.0;

/// 入力によって一意に定まる情報
pub struct Input {
    sy: usize,
    sx: usize,
    a: Vec<Vec<usize>>,
    t: Vec<Vec<usize>>,
    /// neighbor[y, x, c] := (y, x)に一番近い，A[yy, xx]==cとなる(yy, xx)
    neighbor: Vec<Vec<Vec<(usize, usize)>>>,
    /// neighbor_dist[y, x, c] := (y, x)に一番近い，A[yy, xx]==cとなる(yy, xx)までの距離
    neighbor_dist: Vec<Vec<Vec<u32>>>,
    /// nearest neighbor path
    /// nn_path[y, x, i] := A[y, x]==T[i, 0]の時，(y, x)からT[i]を巡った時の最短経路
    nn_path: Vec<Vec<Vec<Vec<(usize, usize)>>>>,
    /// nn_path_dist[y, x, i] := A[y, x]==T[i, 0]の時，(y, x)からT[i]を巡った時の最短経路の距離
    nn_path_dist: Vec<Vec<Vec<u32>>>,
}
impl Input {
    /// read and preprocess
    fn new() -> Self {
        input! {
            n: usize, m: usize,
            sy: usize, sx: usize,
            a: [Chars; n],
            t: [Chars; m],
        }
        assert!(n == N && m == M);

        // charの扱いが面倒なので，usizeに変換
        let a = a
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&c| c as usize - 'A' as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        let t = t
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&c| char_to_usize(c))
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        // neighbor, neighbor_dist を作る
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .map(|c| char_to_usize(c))
            .collect::<Vec<usize>>();
        let mut neighbor = vec![vec![vec![(0, 0); alphabet.len()]; N]; N];
        let mut neighbor_dist = vec![vec![vec![u32::MAX; alphabet.len()]; N]; N];
        for y in 0..N {
            for x in 0..N {
                for &c in alphabet.iter() {
                    for yy in 0..N {
                        for xx in 0..N {
                            let cc = a[yy][xx];
                            if c != cc {
                                continue;
                            }
                            let dist = eval_dist((y, x), (yy, xx));
                            if dist < neighbor_dist[y][x][c] {
                                neighbor_dist[y][x][c] = dist;
                                neighbor[y][x][c] = (yy, xx);
                            }
                        }
                    }
                }
            }
        }

        // nn_path, nn_path_dist を作る
        let mut nn_path = vec![vec![vec![vec![]; M]; N]; N];
        let mut nn_path_dist = vec![vec![vec![u32::MAX; M]; N]; N];
        for ssy in 0..N {
            for ssx in 0..N {
                for (i, ti) in t.iter().enumerate() {
                    if a[ssy][ssx] != ti[0] {
                        continue;
                    }
                    let mut path_dist = 0;
                    let (mut cur_y, mut cur_x) = (ssy, ssx);
                    nn_path[ssy][ssx][i].push((cur_y, cur_x));
                    for (j, &c) in ti.iter().enumerate() {
                        if j > 0 {
                            // t[i][0]の時は加算しない
                            path_dist += neighbor_dist[cur_y][cur_x][c];
                        }
                        (cur_y, cur_x) = neighbor[cur_y][cur_x][c];
                        nn_path[ssy][ssx][i].push((cur_y, cur_x));
                    }
                    nn_path_dist[ssy][ssx][i] = path_dist;
                }
            }
        }

        Input {
            sy,
            sx,
            a,
            t,
            neighbor,
            neighbor_dist,
            nn_path,
            nn_path_dist,
        }
    }
}

/// 出力する情報
pub struct Output {
    ans_yx: Vec<(usize, usize)>,
}
impl Output {
    fn write(&self) {
        assert!(self.ans_yx.len() <= L_LIMIT);
        for &(y, x) in self.ans_yx.iter() {
            assert!(y < N && x < N);
            println!("{} {}", y, x);
        }
    }
}

pub mod solver {
    use super::*;
    use rand::Rng;
    use std::{cmp::max, cmp::min, time::Instant};

    /// 解を表現する情報
    #[derive(Clone, Debug)]
    struct State {
        /// ans_t[i] := t[i]を訪れる順番
        ans_t: Vec<usize>,
        /// ans_t_head[i] := ans_t[i]の先頭の座標
        ans_t_head: Vec<(usize, usize)>,
        /// ans_t_tail[i] := ans_t[i]の末尾の座標
        ans_t_tail: Vec<(usize, usize)>,
        total_cost: i64,
    }
    impl State {
        fn new() -> Self {
            State {
                ans_t: Vec::new(),
                ans_t_head: Vec::new(),
                ans_t_tail: Vec::new(),
                total_cost: 0,
            }
        }

        /// コストからスコアを計算：O(1)
        fn eval_score(&self) -> i64 {
            max(10000 - self.total_cost, 1001)
        }
    }

    /// 解く
    pub fn solve(input: Input) -> Output {
        // 初期解
        // stateを作る
        let mut state = State::new();
        let (mut cur_y, mut cur_x) = (input.sy, input.sx);
        for (i, ti) in input.t.iter().enumerate() {
            // ans_t
            state.ans_t.push(i);
            // total_cost
            // move cur to t[i][0]
            state.total_cost += input.neighbor_dist[cur_y][cur_x][ti[0]] as i64;
            // eprintln!(
            //     "1:cost:{}/{}, {},{},{:?}",
            //     input.neighbor_dist[cur_y][cur_x][ti[0]],
            //     state.total_cost,
            //     cur_y,
            //     cur_x,
            //     ti.iter().map(|&c| usize_to_char(c)).collect::<Vec<char>>()
            // );
            (cur_y, cur_x) = input.neighbor[cur_y][cur_x][ti[0]];
            // ans_t_head
            let t_head = *input.nn_path[cur_y][cur_x][i].first().unwrap();
            state.ans_t_head.push(t_head);
            // ans_t_tail
            let t_tail = *input.nn_path[cur_y][cur_x][i].last().unwrap();
            state.ans_t_tail.push(t_tail);
            // total_cost
            // move t[i][0] to t[i][-1]
            state.total_cost += input.nn_path_dist[cur_y][cur_x][i] as i64;
            // eprintln!(
            //     "2:cost:{}/{}, {},{},{}",
            //     input.nn_path_dist[cur_y][cur_x][i], state.total_cost, cur_y, cur_x, i
            // );
            (cur_y, cur_x) = t_tail;
        }

        // 焼く
        // state = solver::annealing(&input, &state);

        // ans_tからans_yxを作る
        let mut ans_yx = Vec::new();
        let (mut cur_y, mut cur_x) = (input.sy, input.sx);
        for &i in state.ans_t.iter() {
            for &c in input.t[i].iter() {
                (cur_y, cur_x) = input.neighbor[cur_y][cur_x][c];
                ans_yx.push((cur_y, cur_x));
            }
        }

        eprintln!(
            "Score = {}, Cost = {}",
            state.eval_score(),
            state.total_cost
        );
        Output { ans_yx }
    }

    /// 焼く
    fn annealing(input: &Input, init_state: &State) -> State {
        let start_time = Instant::now();
        // annealing parameter
        // let start_temp = 10f64;
        let start_temp = 1e-5; // 山登り
        let end_temp = 1e-5;
        let mut temp = start_temp;
        let delta_threshold = 3;
        let mut rng = rand_pcg::Pcg64Mcg::new(42);
        // solution
        let init_score = init_state.eval_score();
        let mut state = init_state.clone();
        let mut best_state = init_state.clone();
        // info
        let mut min_delta = i64::MAX;
        let mut max_delta = i64::MIN;
        let mut mean_delta = 0;
        let mut iter_cnt: i64 = 0;
        let mut move_cnt: i64 = 0;
        let mut update_cnt: i64 = 0;
        'main: loop {
            iter_cnt += 1;
            // 温度更新
            if (iter_cnt & ((1 << 4) - 1)) == 0 {
                // ref: terry_u16さんの実装(https://atcoder.jp/contests/ahc028/submissions/49221892)
                // 16回に1回，温度を更新
                let t = start_time.elapsed().as_millis() as f64 / TIME_LIMIT;
                if t >= 1.0 {
                    break 'main;
                }
                temp = f64::powf(start_temp, 1.0 - t) * f64::powf(end_temp, t);
            }

            // 近傍操作
            // insert(t_i, t_j): t_iをt_jの後ろに挿入
            let i = rng.gen_range(1..(M - 4));
            let j = rng.gen_range((i + 1)..(M - 1));
            let mut new_state = state.clone();
            // ans_t
            // t_i-1 -> t_i -> t_i+1 ... t_j-1 -> t_j -> t_j+1
            // t_i-1 -> t_i+1 ... t_j-1 -> t_j -> t_i -> t_j+1
            new_state.ans_t.clear();
            new_state.ans_t.extend_from_slice(&state.ans_t[..i]);
            new_state.ans_t.extend_from_slice(&state.ans_t[(i + 1)..=j]);
            new_state.ans_t.extend_from_slice(&state.ans_t[i..(i + 1)]);
            new_state.ans_t.extend_from_slice(&state.ans_t[(j + 1)..]);
            eprintln!("insert ti:{} -> tj:{}", i, j);
            eprintln!("state|t:\n{:?}", state.ans_t);
            eprintln!("new state|t:\n{:?}", new_state.ans_t);
            // ans_t_head
            // head[i+1], head[j], head[i], head[j+1]が変わる可能性あり
            // ans_t_tail
            // tail[i+1], tail[j], tail[i], tail[j+1]が変わる可能性あり
            for (k1, k2) in [(i - 1, i + 1), (j - 1, j), (j, i), (i, j + 1)] {
                let (k1_tail_y, k1_tail_x) = state.ans_t_tail[k1];
                let k2_top_c = input.t[k2][0];
                let (k2_head_y, k2_head_x) = input.neighbor[k1_tail_y][k1_tail_x][k2_top_c];
                new_state.ans_t_head[k2] = (k2_head_y, k2_head_x);
                new_state.ans_t_tail[k2] = *input.nn_path[k2_head_y][k2_head_x][k2].last().unwrap();
            }

            // 良くなりますか？
            let mut cur_cost = 0;
            cur_cost += eval_dist(state.ans_t_tail[i - 1], state.ans_t_head[i]);
            cur_cost += eval_dist(state.ans_t_tail[i], state.ans_t_head[i + 1]);
            cur_cost += eval_dist(state.ans_t_tail[j - 1], state.ans_t_head[j]);
            cur_cost += eval_dist(state.ans_t_tail[j], state.ans_t_head[j + 1]);
            let mut new_cost = 0;
            new_cost += eval_dist(new_state.ans_t_tail[i - 1], new_state.ans_t_head[i + 1]);
            new_cost += eval_dist(new_state.ans_t_tail[j - 1], new_state.ans_t_head[j]);
            new_cost += eval_dist(new_state.ans_t_tail[j], new_state.ans_t_head[i]);
            new_cost += eval_dist(new_state.ans_t_tail[i], new_state.ans_t_head[j + 1]);
            let delta = new_cost as i64 - cur_cost as i64;
            if delta > delta_threshold {
                continue;
            }
            new_state.total_cost += delta;

            // 遷移
            if delta <= 0 || rng.gen_bool(f64::exp(-delta as f64 / temp)) {
                // 近傍に移動してみる
                move_cnt += 1;
                state = new_state;
                min_delta = min(min_delta, delta);
                max_delta = max(max_delta, delta);
                mean_delta += delta;
                if state.total_cost < best_state.total_cost {
                    // ベストを更新した
                    update_cnt += 1;
                    best_state = state.clone();
                    eprintln!(
                        "update|iter:{}/{}|score:{}",
                        update_cnt,
                        iter_cnt,
                        best_state.eval_score()
                    );
                }
            }

            break 'main; // DEBUG:
        } // end of 'main

        eprintln!("==================== annealing ====================");
        eprintln!("iter\tmove\t:{}", move_cnt);
        eprintln!("\tupdate\t:{}", update_cnt);
        eprintln!("\ttotal\t:{}", iter_cnt);
        eprintln!("delta\tmin/max\t:{} / {}", min_delta, max_delta);
        eprintln!("\tmean\t:{}", mean_delta / iter_cnt);
        eprintln!("score\tinit\t:{}", init_score);
        eprintln!("\tbest\t:{}", best_state.eval_score());
        eprintln!("===================================================");

        best_state
    }
}

pub mod my_util {
    pub fn char_to_usize(c: char) -> usize {
        c as usize - 'A' as usize
    }

    pub fn usize_to_char(i: usize) -> char {
        (i as u8 + b'A') as char
    }

    pub fn eval_dist((y, x): (usize, usize), (yy, xx): (usize, usize)) -> u32 {
        (y.abs_diff(yy) + x.abs_diff(xx) + 1) as u32
    }
}
