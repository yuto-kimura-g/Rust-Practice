#![allow(unused_imports)]
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
        n: u32,
        s: u32,
        k: u32,
        pq: [(u32, u32); n]
    }
    let mut ans = 0u32;
    for (p, q) in pq.iter() {
        ans += p * q;
    }
    if ans < s {
        ans += k;
    }
    println!("{}", ans);
}
