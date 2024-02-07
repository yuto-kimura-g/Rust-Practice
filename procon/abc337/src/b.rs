use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let ans = if s.iter().tuple_windows().all(|(x, y)| x <= y) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
