use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: usize,
        ab: [(Usize1, Usize1); q],
    }
    let inv_p = {
        let mut inv_p = vec![0; n];
        for (i, &pi) in p.iter().enumerate() {
            inv_p[pi] = i;
        }
        inv_p
    };
    for &(a, b) in ab.iter() {
        let ans = if inv_p[a] < inv_p[b] { a + 1 } else { b + 1 };
        println!("{}", ans);
    }
}
