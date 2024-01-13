use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        uv: [(Usize1, Usize1); m]
    }

    let mut g = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        if a[u] < a[v] {
            g[u].push(v);
        } else if a[u] > a[v] {
            g[v].push(u);
        } else {
            g[u].push(v);
            g[v].push(u);
        }
        // match a[u].cmp(&a[v]) {
        //     Ordering::Less => {
        //         g[u].push(v);
        //     }
        //     Ordering::Equal => {
        //         g[u].push(v);
        //         g[v].push(u);
        //     }
        //     Ordering::Greater => {
        //         g[v].push(u);
        //     }
        // }
    }

    // score[i] := 0..=i までのスコア
    let mut score = vec![0; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(a[0]), 1, 0));
    while let Some((_, s, u)) = heap.pop() {
        if score[u] > 0 {
            continue;
        }
        score[u] = s;
        for &v in g[u].iter() {
            assert!(a[u] <= a[v]);
            if score[v] > 0 {
                continue;
            }
            let ss = if a[u] < a[v] { score[u] + 1 } else { score[u] };
            heap.push((Reverse(a[v]), ss, v));
        }
    }
    eprintln!("{:?}", score);

    let ans = score.last().unwrap();
    println!("{}", ans);
}
