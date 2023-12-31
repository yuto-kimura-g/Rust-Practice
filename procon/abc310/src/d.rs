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
        n: i32,
        t: i32,
        m: i32,
        ab:[(i32, i32); m]
    }
    let mut A: Vec<i32> = Vec::new();
    let mut B: Vec<i32> = Vec::new();
    for (a, b) in ab {
        A.push(a);
        B.push(b);
    }
}
