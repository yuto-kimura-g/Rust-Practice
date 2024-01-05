use proconio::input;

fn main() {
    input! {
        b: i32,
        g: i32
    }
    let ans: &str = if b > g { "Bat" } else { "Glove" };
    println!("{}", ans);
}
