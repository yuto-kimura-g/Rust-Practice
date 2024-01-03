use proconio::input;

fn main() {
    input! {
        n: usize
    }
    // println!("pi:{}", std::f64::consts::PI);
    let pi = String::from("3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679");
    let ans = &pi[0..(n + 2)];
    println!("{}", ans);
}
