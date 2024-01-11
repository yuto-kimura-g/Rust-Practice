use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    }
    let n = s.len();
    s[n - 1] = '4';
    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
