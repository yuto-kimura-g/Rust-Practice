use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        q: usize,
        cd: [(char, char); q],
    }
    let mut pending = HashMap::new();
    for (c, d) in cd.iter().rev() {
        let e = pending.get(d).unwrap_or(d);
        pending.insert(c, *e);
    }
    eprintln!("{:?}", pending);
    let ans = s
        .iter()
        .map(|c| pending.get(c).unwrap_or(c))
        .collect::<String>();
    println!("{}", ans);
}
