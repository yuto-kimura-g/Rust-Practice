use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }

    let mut grid = vec![vec!['.'; w]; h];
    let (mut y, mut x) = (0, 0);
    let (mut dy, mut dx) = (-1, 0);
    let pi = std::f64::consts::PI;

    for _ in 0..n {
        match grid[y][x] {
            '.' => {
                grid[y][x] = '#';
                // 時計回り
                (dx, dy) = rot(dx, dy, pi / 2.0);
                let cy = (y as isize + dy) % h as isize;
                let cx = (x as isize + dx) % w as isize;
                y = if cy == -1 { h - 1 } else { cy as usize };
                x = if cx == -1 { w - 1 } else { cx as usize };
            }
            '#' => {
                grid[y][x] = '.';
                // 反時計回り
                (dx, dy) = rot(dx, dy, -pi / 2.0);
                let cy = (y as isize + dy) % h as isize;
                let cx = (x as isize + dx) % w as isize;
                y = if cy == -1 { h - 1 } else { cy as usize };
                x = if cx == -1 { w - 1 } else { cx as usize };
            }
            _ => unreachable!(),
        }
    }

    let ans = grid
        .iter()
        .map(|row| row.iter().collect::<String>())
        .join("\n");
    println!("{}", ans);
}

fn rot(x: isize, y: isize, theta: f64) -> (isize, isize) {
    (
        (x as f64 * f64::cos(theta) - y as f64 * f64::sin(theta)) as isize,
        (x as f64 * f64::sin(theta) + y as f64 * f64::cos(theta)) as isize,
    )
}

#[test]
fn test_rot() {
    // (dx, dy)
    // up: (0, -1), right: (1, 0), down: (0, 1), left: (-1, 0)
    let pi = std::f64::consts::PI;

    assert_eq!(rot(0, -1, pi / 2.0), (1, 0));
    assert_eq!(rot(1, 0, pi / 2.0), (0, 1));
    assert_eq!(rot(0, 1, pi / 2.0), (-1, 0));
    assert_eq!(rot(-1, 0, pi / 2.0), (0, -1));

    assert_eq!(rot(0, -1, -pi / 2.0), (-1, 0));
    assert_eq!(rot(1, 0, -pi / 2.0), (0, -1));
    assert_eq!(rot(0, 1, -pi / 2.0), (1, 0));
    assert_eq!(rot(-1, 0, -pi / 2.0), (0, 1));
}
