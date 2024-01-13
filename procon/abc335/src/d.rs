use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut grid = vec![vec!["T".to_string(); n]; n];
    // dy, dx: R, D, L, U
    // !0 = -1を使えば，usizeだけで表現できる．
    let dy: [usize; 4] = [0, 1, 0, !0];
    let dx: [usize; 4] = [1, 0, !0, 0];
    let (mut y, mut x, mut di) = (0, 0, 0);
    for i in 1..n * n {
        grid[y][x] = i.to_string();
        // wrapping_addを使えば，usizeへのキャスト祭りしなくて済む
        let ny = y.wrapping_add(dy[di]);
        let nx = x.wrapping_add(dx[di]);
        if !(ny < n && nx < n) || grid[ny][nx] != "T" {
            di = (di + 1) % 4;
        }
        y = y.wrapping_add(dy[di]);
        x = x.wrapping_add(dx[di]);
    }

    let ans = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", ans);
}
