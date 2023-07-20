use proconio::input;

fn main() {
    println!("Hello, world!");
    input! {
        a: i32,
        b: i32,
        s: String
    }
    println!("{} {}", a + b, s);
}

