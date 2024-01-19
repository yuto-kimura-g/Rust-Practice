use my_util::*;
use proconio::{fastout, input, marker::Chars};
use rand::Rng;
use std::{cmp::max, cmp::min, time::Instant};

#[fastout]
fn main() {
    let input = Input::new();
    let output = solve(input);
    output.write();
}

/// グリッドの大きさ
const N: usize = 15;
/// 文字列の個数
const M: usize = 200;
/// 操作回数の上限
const L_LIMIT: usize = 5000;
/// 実行時間制限 [msec]
const TIME_LIMIT: f64 = 1950.0;
// const TIME_LIMIT: f64 = 15000.0;

/// 入力によって一意に定まる情報
struct Input {
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
                    .map(|&c| char_to_usize(c))
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
        let max_char_usize = *alphabet.iter().max().unwrap() + 1;
        let mut neighbor = vec![vec![vec![(0, 0); max_char_usize]; N]; N];
        let mut neighbor_dist = vec![vec![vec![u32::MAX; max_char_usize]; N]; N];
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
struct Output {
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

/// 解を表現する情報
#[derive(Clone, Debug)]
struct State {
    /// ans_t[i] := t[i]を訪れる順番
    ans_t: Vec<usize>,
    total_cost: i64,
}
impl State {
    fn new() -> Self {
        State {
            ans_t: Vec::new(),
            total_cost: 0,
        }
    }

    fn insert_mark(&mut self) {
        self.ans_t.insert(0, usize::MAX);
    }

    fn remove_mark(&mut self) {
        self.ans_t.remove(0);
    }

    /// コストからスコアを計算：Θ(1)
    fn eval_score(&self) -> i64 {
        max(10000 - self.total_cost, 1001)
    }

    /// ans_t からコストを再計算：O(M)
    /// 高速化のため，コストが閾値より大きくなれば，計算を打ち切ってNoneを返す
    fn eval_cost_from_t(&mut self, input: &Input, cost_threshold: i64) -> Option<i64> {
        self.total_cost = 0;
        let (mut cur_y, mut cur_x) = (input.sy, input.sx);
        for &i in self.ans_t.iter() {
            if i == usize::MAX {
                // skip mark
                continue;
            }
            let ti0 = *input.t[i].first().unwrap();
            // total_cost: move cur to t[i][0]
            self.total_cost += input.neighbor_dist[cur_y][cur_x][ti0] as i64;
            // move cur to t[i][0]
            (cur_y, cur_x) = input.neighbor[cur_y][cur_x][ti0];
            // total_cost: move t[i][0] to t[i][-1]
            self.total_cost += input.nn_path_dist[cur_y][cur_x][i] as i64;
            // move t[i][0] to t[i][-1]
            (cur_y, cur_x) = *input.nn_path[cur_y][cur_x][i].last().unwrap();
            if self.total_cost > cost_threshold {
                return None;
            }
        }
        Some(self.total_cost)
    }
}

/// 解く
fn solve(input: Input) -> Output {
    // 初期解
    let mut state = State::new();
    state.ans_t = (0..M).collect::<Vec<usize>>();
    state.insert_mark();
    let _ = state.eval_cost_from_t(&input, i64::MAX); // calc total_cost

    // 焼く
    // TODO: overlapを考慮したい
    state = annealing(&input, &state);
    state.remove_mark();

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
        state.eval_cost_from_t(&input, i64::MAX).unwrap(),
    );
    Output { ans_yx }
}

/// 焼く
fn annealing(input: &Input, init_state: &State) -> State {
    let start_time = Instant::now();
    // parameter
    const START_TEMP: f64 = 50.0;
    const END_TEMP: f64 = 1e-5;
    let mut temp = START_TEMP;
    let mut rng = rand_pcg::Pcg64Mcg::new(42);
    const NEIGHBOR_TYPE_NUM: usize = 2;
    // solution
    let init_score = init_state.eval_score();
    let mut state = init_state.clone();
    let mut best_state = init_state.clone();
    // info
    let mut min_delta = vec![i64::MAX; NEIGHBOR_TYPE_NUM];
    let mut max_delta = vec![i64::MIN; NEIGHBOR_TYPE_NUM];
    let mut iter_cnt = 0i64;
    let mut iter_each_cnt = vec![0; NEIGHBOR_TYPE_NUM];
    let mut move_cnt = vec![0; NEIGHBOR_TYPE_NUM];
    let mut update_cnt = vec![0; NEIGHBOR_TYPE_NUM];
    'main: loop {
        iter_cnt += 1;
        // 時間更新
        if (iter_cnt & ((1 << 4) - 1)) == 0 {
            // NOTE: 高速化
            //      ref: terry_u16さんの実装(https://atcoder.jp/contests/ahc028/submissions/49221892)
            //      時間計測がボトルネックにならないように，頻度を減らす．
            //      この例だと，16回に1回，時間を更新する．
            //      また，%で割り算すると遅いので，&のビット演算で高速化．
            let t = start_time.elapsed().as_millis() as f64 / TIME_LIMIT;
            if t >= 1.0 {
                break 'main;
            }
            // 温度更新
            temp = f64::powf(START_TEMP, 1.0 - t) * f64::powf(END_TEMP, t);
        }

        // 近傍操作
        let neighbor_type: usize = rng.gen_range(0..NEIGHBOR_TYPE_NUM);
        iter_each_cnt[neighbor_type] += 1;
        let mut new_state: State = match neighbor_type {
            0 => {
                // shift近傍：変化小さめ
                // shift(t_i): t_iを一番後ろに
                // t_i-1 -> t_i -> t_i+1 ... t_e
                // t_i-1 -> t_i+1 ... t_e -> t_i
                let e = state.ans_t.len() - 1;
                let i = rng.gen_range(1..(e));
                let mut new_state = state.clone();
                // ans_t
                new_state.ans_t.clear();
                new_state.ans_t.extend_from_slice(&state.ans_t[..=(i - 1)]);
                new_state.ans_t.extend_from_slice(&state.ans_t[(i + 1)..]);
                new_state.ans_t.push(state.ans_t[i]);
                new_state
            }
            1 => {
                // swap近傍：変化大きめ
                // swap(t_i, t_j): t_iとt_jを入れ替える
                // t_i-1 -> t_i -> t_i+1 ... t_j-1 -> t_j -> t_j+1
                // t_i-1 -> t_j -> t_i+1 ... t_j-1 -> t_i -> t_j+1
                let e = state.ans_t.len() - 1;
                let i = rng.gen_range(1..(e - 3));
                let j = rng.gen_range((i + 1)..(e - 1));
                let mut new_state = state.clone();
                // ans_t
                new_state.ans_t.swap(i, j);
                new_state
            }
            _ => unreachable!(),
        };

        // 遷移
        // 最大化なのか，最小化なのか注意
        //      costは小さいほど良い
        //      scoreは大きいほど良い
        // NOTE: 高速化
        //      ref: https://qiita.com/not522/items/cd20b87157d15850d31c
        //      コスト計算関数の引数に閾値を追加して，閾値を超えたら計算を打ち切ることで高速化できる
        let cost_threshold = state.total_cost - (temp * rng.gen::<f64>().ln()).ceil() as i64;
        match new_state.eval_cost_from_t(input, cost_threshold) {
            None => continue,
            Some(new_cost) => {
                // (exp(delta/T) >= rand(0,1)) == true
                // 近傍に移動してみる
                move_cnt[neighbor_type] += 1;
                let cost_delta = new_cost - state.total_cost;
                min_delta[neighbor_type] = min(min_delta[neighbor_type], cost_delta);
                max_delta[neighbor_type] = max(max_delta[neighbor_type], cost_delta);
                state = new_state;
                if state.total_cost < best_state.total_cost {
                    // ベストを更新した
                    update_cnt[neighbor_type] += 1;
                    best_state = state.clone();
                    eprintln!(
                        "update|iter:{:?}/{}|score:{}",
                        update_cnt,
                        iter_cnt,
                        best_state.eval_score()
                    );
                }
            }
        };
    } // end of 'main

    eprintln!("==================== annealing ====================");
    eprintln!("elapsed\t\t:{} [ms]", start_time.elapsed().as_millis());
    eprintln!("iter\tmove\t:{:?}", move_cnt);
    eprintln!("\tupdate\t:{:?}", update_cnt);
    eprintln!(
        "\ttotal\t:{:?},{}",
        iter_each_cnt,
        iter_each_cnt.iter().sum::<u32>()
    );
    eprintln!("delta\tmin/max\t:{:?} / {:?}", min_delta, max_delta);
    eprintln!("score\tinit\t:{}", init_score);
    eprintln!("\tbest\t:{}", best_state.eval_score());
    eprintln!("===================================================");

    best_state
}

pub mod my_util {
    pub fn char_to_usize(c: char) -> usize {
        c as usize - 'A' as usize
    }

    pub fn usize_to_char(c: usize) -> char {
        (c as u8 + b'A') as char
    }

    pub fn eval_dist((y, x): (usize, usize), (yy, xx): (usize, usize)) -> u32 {
        (y.abs_diff(yy) + x.abs_diff(xx) + 1) as u32
    }

    #[test]
    fn test_char() {
        assert_eq!(char_to_usize('A'), 0);
        assert_eq!(char_to_usize('Z'), 25);
        assert_eq!(usize_to_char(0), 'A');
        assert_eq!(usize_to_char(25), 'Z');
        assert_eq!('A', usize_to_char(char_to_usize('A')));
        assert_eq!('Z', usize_to_char(char_to_usize('Z')));
    }
    #[test]
    fn test_dist() {
        assert_eq!(eval_dist((0, 0), (0, 0)), 1);
        assert_eq!(eval_dist((0, 0), (0, 1)), 2);
        assert_eq!(eval_dist((0, 0), (1, 0)), 2);
        assert_eq!(eval_dist((0, 0), (1, 1)), 3);
    }
}
