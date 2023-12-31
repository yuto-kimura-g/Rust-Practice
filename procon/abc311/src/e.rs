#![allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    iter::FromIterator,
};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(usize, usize); n]
    }
    let hole: HashSet<(usize, usize)> = HashSet::from_iter(ab.iter().cloned());
    let mut dp: Vec<Vec<i64>> = vec![vec![0; w + 1]; h + 1];
    for y in 1..h + 1 {
        for x in 1..w + 1 {
            if hole.contains(&(y, x)) {
                continue;
            }
            dp[y][x] = vec![dp[y - 1][x], dp[y][x - 1], dp[y - 1][x - 1]]
                .iter()
                .min()
                .unwrap()
                + 1;
        }
    }
    // let ans: i64 = dp
    //     .iter()
    //     .map(|v: &Vec<i64>| -> i64 { v.iter().sum() })
    //     .sum();
    let ans: i64 = dp.iter().flatten().sum();
    println!("{}", ans);
}
