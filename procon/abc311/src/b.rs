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
        n: usize,
        d: usize,
        s: [Chars; n]
    }

    if d == 1 {
        for j in 0..n {
            if s[j][0] == 'x' {
                println!("{}", 0);
                return;
            }
        }
        println!("{}", 1);
        return;
    }

    let mut sd = vec![' '; d];
    for i in 0..d {
        let mut ok = true;
        for j in 0..n {
            if s[j][i] == 'x' {
                ok = false;
                break;
            }
        }
        sd[i] = if ok { 'o' } else { 'x' };
    }
    let rle = run_length_encoding(sd);
    eprintln!("rle:{:?}", rle);
    let ans = rle
        .iter()
        .map(|(k, v)| if *k == 'o' { *v } else { 0 })
        .max()
        .unwrap();
    println!("{}", ans);
}

fn run_length_encoding(a: Vec<char>) -> Vec<(char, u32)> {
    let mut ret = Vec::<(char, u32)>::new();
    let mut cursor = a[0];
    let mut cnt = 0;
    for ai in a {
        if ai == cursor {
            cnt += 1;
        } else {
            ret.push((cursor, cnt));
            cursor = ai;
            cnt = 1;
        }
    }
    if cnt > 1 {
        ret.push((cursor, cnt));
    }
    ret
}
