use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut b = vec![0i64];
    for &ai in a.iter() {
        b.push(b.last().unwrap() + ai);
    }
    let start = -*b.iter().min().unwrap();
    eprintln!("b={:?}, start={}", b, start);
    let ans = b.last().unwrap() + start;
    println!("{}", ans);
}
