use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }
    let x_sum = xy.iter().map(|(x, _)| x).sum::<i32>();
    let y_sum = xy.iter().map(|(_, y)| y).sum::<i32>();
    let ans = if x_sum > y_sum {
        "Takahashi"
    } else if x_sum < y_sum {
        "Aoki"
    } else {
        "Draw"
    };
    println!("{}", ans);
}
