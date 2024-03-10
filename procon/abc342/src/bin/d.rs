use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let is_square = |x: usize| -> bool {
        let y = (x as f64).sqrt();
        (y * y) as usize == x
    };

    let mut ans = 0;
    for i in 0..n {
        if a[i] == 0 {
            ans += n - i - 1;
            continue;
        }
        for j in i + 1..n {
            if is_square(a[i] * a[j]) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
