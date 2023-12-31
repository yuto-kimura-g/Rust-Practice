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
        _n: u32,
        s: Chars
    }
    let mut st = HashSet::<&char>::new();
    for (i, si) in s.iter().enumerate() {
        st.insert(si);
        if st.len() == 3 {
            println!("{}", i + 1);
            break;
        }
    }
}
