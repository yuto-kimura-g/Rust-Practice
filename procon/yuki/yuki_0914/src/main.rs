fn main() {
    // proconio使えないの厳しい...
    // ref: https://doc.rust-jp.rs/atcoder-rust-resources/#素早く書くことにはあまり向かない仕様
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf
        .trim()
        .split_terminator(',')
        .map(|c| c.parse::<i32>().unwrap());
    let (j, m, b) = (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    );
    let t = j + m + b;
    let a: f64 = t as f64 / 3.0;
    println!("合計点:{}", t);
    println!("平均点:{:.1}", a);
}
