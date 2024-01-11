use proconio::input;

fn main() {
    input! {
        _n: usize,
        k: usize,
        a: [i64; k]
    }

    // prefix_sum[i] := a[..i]の差の累積
    let mut prefix_sum = vec![0; k + 1];
    for i in 1..=k {
        prefix_sum[i] = prefix_sum[i - 1];
        if i % 2 == 0 {
            prefix_sum[i] += (a[i - 1] - a[i - 2]).abs();
        }
    }
    // dbg!(&prefix_sum);
    // suffix_sum[i] := a[i+1..]の差の累積
    let mut suffix_sum = vec![0; k + 1];
    for i in (0..k).rev() {
        suffix_sum[i] = suffix_sum[i + 1];
        if (k - i) % 2 == 0 {
            suffix_sum[i] += (a[i + 1] - a[i]).abs();
        }
    }
    // dbg!(&suffix_sum);

    // a[i]以外を使う場合を全探索
    let ans = (0..prefix_sum.len())
        .step_by(2)
        .map(|i| prefix_sum[i] + suffix_sum[i])
        .min()
        .unwrap();
    println!("{}", ans);
}
