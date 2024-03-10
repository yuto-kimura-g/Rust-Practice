use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = String::new();
    let mut is_open = false;
    for &si in s.iter() {
        if si == '|' {
            is_open = !is_open;
        } else if is_open {
            continue;
        } else {
            ans.push(si);
        }
    }
    println!("{}", ans);
}
