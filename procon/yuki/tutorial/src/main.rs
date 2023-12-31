fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf
        .trim()
        .split_whitespace()
        .map(|c| c.parse::<f64>().unwrap());
    let (a, b) = (iter.next().unwrap(), iter.next().unwrap());
    let ans = b / a;
    println!("{}", ans.ceil());
}
