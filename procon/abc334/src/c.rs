use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; k]
    }
    let mut prefix_sum = vec![0; k + 1];
    let mut suffix_sum = vec![0; k + 1];

    let ans = (0..k + 1)
        .step_by(2)
        .map(|i| prefix_sum[i] + suffix_sum[i])
        .min()
        .unwrap();
    println!("{}", ans);
}
