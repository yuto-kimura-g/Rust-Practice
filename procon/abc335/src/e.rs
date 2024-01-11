use std::{cmp::max, cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n],
        uv: [(Usize1, Usize1); m]
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        g[u].push(v);
        g[v].push(u);
    }

    // score[i] := 1..=i までのスコア
    let mut score = vec![0; n];

    // score[0] = 1;
    // let mut q = BinaryHeap::<(Reverse<i32>, usize)>::new();
    // q.push((Reverse(a[0]), 0));
    // // q.push((Reverse(0), move_to(0, 0)));
    // let mut visited = vec![false; n];
    // while let Some((Reverse(u), state)) = q.pop() {
    //     visited[u] = true;
    //     score[u] = max(score[u], state.len());
    //     for &v in g[u].iter() {
    //         if visited[v] {
    //             continue;
    //         }
    //         if a[v] < a[u] {
    //             // 広義単調増加(a[u] <= a[v])じゃないとスコアゼロなので，見る意味無し
    //             continue;
    //         }
    //         state.insert(v);
    //         q.push((Reverse(a[v]), state));
    //     }
    //     eprintln!("score:{:?}", score);
    // }

    let ans = score.last().unwrap();
    println!("{}", ans);
}
