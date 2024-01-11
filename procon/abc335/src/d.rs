use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut grid = vec![vec!["T".to_string(); n]; n];
    let dy: [isize; 4] = [0, 1, 0, -1]; // R,D,L,U
    let dx: [isize; 4] = [1, 0, -1, 0]; // R,D,L,U
    let (mut y, mut x, mut di) = (0, 0, 0);
    for i in 1..n * n {
        grid[y][x] = i.to_string();
        // rustでgridの走査だるすぎる
        // 本格的に使うなら，ライブラリ整備必須だな...
        let ny = y.wrapping_add_signed(dy[di]);
        let nx = x.wrapping_add_signed(dx[di]);
        if !(ny < n && nx < n) || grid[ny][nx] != "T" {
            di = (di + 1) % 4;
        }
        y = y.wrapping_add_signed(dy[di]);
        x = x.wrapping_add_signed(dx[di]);
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
