use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut k = Vec::new();
    let mut x = 1;
    while x * x * x <= n {
        k.push(x * x * x);
        x += 1;
    }
    // eprintln!("{:?}, {}", k, k.len());
    for &ki in k.iter().rev() {
        if is_palindrome(ki) {
            println!("{}", ki);
            return;
        }
    }
}

fn is_palindrome(s: u64) -> bool {
    let s = s.to_string();
    s.chars().eq(s.chars().rev())
}
