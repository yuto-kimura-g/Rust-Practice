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
        s: [Chars; n]
    }
    let mut set = HashSet::new();
    // let mut set = BTreeSet::new();
    for si in &s {
        let sis = si.iter().collect::<String>();
        let rsis = si.iter().rev().collect::<String>();
        // from_iterでCEになる．ローカルだと動くのに．．．Rustのバージョンの問題かな？
        // let sis = String::from_iter(si.iter());
        // let rsis = String::from_iter(si.iter().rev());
        set.insert(min(sis, rsis));
    }
    // eprintln!("{:?}", s);
    // eprintln!("{:?}", set);
    println!("{}", set.len());
}
