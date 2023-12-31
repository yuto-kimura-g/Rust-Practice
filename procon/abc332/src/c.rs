// written with copilot :)

/*
 * note:
 * cargo run --bin c < c1.in
 * cargo clippy --bin c --fix
 */

use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        _n: usize,
        m: u32,
        s: String,
    }
    let mut plain_t = 0;
    let mut logo_t = 0;
    let mut ans = 0;
    for si in s.chars() {
        match si {
            '0' => {
                // clear
                plain_t = 0;
                ans = max(ans, logo_t);
                logo_t = 0;
            }
            '1' => {
                // plain or logo
                if plain_t < m {
                    plain_t += 1;
                } else {
                    logo_t += 1;
                }
            }
            '2' => {
                // logo
                logo_t += 1;
            }
            _ => {}
        }
    }
    ans = max(ans, logo_t);
    println!("{}", ans);
}
