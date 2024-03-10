use itertools::Itertools;
use proconio::input;

fn main() {
    let mut a = Vec::new();
    loop {
        input! {
            ai: usize,
        }
        a.push(ai);
        if ai == 0 {
            break;
        }
    }
    let ans = a.iter().rev().join("\n");
    println!("{}", ans);
}
