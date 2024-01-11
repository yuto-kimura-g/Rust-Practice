use itertools::{iproduct, Itertools};
use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    let ans = iproduct!(0..=n, 0..=n, 0..=n)
        .filter(|&(x, y, z)| x + y + z <= n)
        .sorted()
        .map(|(x, y, z)| format!("{} {} {}", x, y, z))
        .collect_vec()
        .join("\n");
    println!("{}", ans);
}
