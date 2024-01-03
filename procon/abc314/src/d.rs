use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        mut s: Chars,
        q: usize,
        query: [(usize, usize, char); q],
    }
    let mut active_index = HashSet::new();
    let mut state = State::Default;
    for &(t, x, c) in query.iter() {
        match t {
            1 => {
                // s[x] <- c
                let x = x - 1; // 1-indexed -> 0-indexed
                s[x] = c;
                active_index.insert(x);
            }
            2 => {
                // lower
                state = State::Lower;
                active_index.clear();
            }
            3 => {
                // upper
                state = State::Upper;
                active_index.clear();
            }
            _ => unreachable!(),
        }
    }
    // dbg!(&s, &active_index, &state);
    let ans = s
        .iter()
        .enumerate()
        .map(|(i, &c)| match state {
            State::Default => c,
            State::Lower => {
                if active_index.contains(&i) {
                    c
                } else {
                    c.to_ascii_lowercase()
                }
            }
            State::Upper => {
                if active_index.contains(&i) {
                    c
                } else {
                    c.to_ascii_uppercase()
                }
            }
        })
        .collect::<String>();
    println!("{}", ans);
}

#[derive(Debug)]
enum State {
    Default,
    Lower,
    Upper,
}
