#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: u64
    }
    let mut adj: Vec<Vec<(u32, u64)>> = vec![Vec::new(); n + 1];
    for _ in 0..m {
        input! {
            mut u: u32,
            mut v: u32,
            w: u64,
        }
        u -= 1;
        v -= 1;
        adj[u as usize].push((v, w));
        adj[v as usize].push((u, w));
    }
    let mut dp: Vec<HashSet<u64>> = vec![HashSet::new(); (1 << n) + 1];
    dp[0].insert(0);
    for u in 0..n {
        dp[1 << u].insert(0);
    }
    for S in 0..(1 << n) {
        for u in 0..n {
            if ((S >> u) & 1) == 1 {
                for (v, w) in adj[u].iter() {
                    for c in dp[S].clone().iter() {
                        dp[S ^ (1 << v)].insert((c + w) % k);
                    }
                }
            }
        }
    }
    let ans = *dp[(1 << n) - 1].iter().min().unwrap();
    println!("{}", ans);
}
