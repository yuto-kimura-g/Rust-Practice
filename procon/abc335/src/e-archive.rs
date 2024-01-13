use std::{cmp::max, collections::HashMap};

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        uv: [(Usize1, Usize1); m]
    }

    let max_a = *a.iter().max().unwrap();
    let mut g = vec![vec![]; max_a + 1];
    let mut uf: UnionFind<usize> = UnionFind::new(n);
    for &(u, v) in uv.iter() {
        let (u, v) = if a[u] <= a[v] { (u, v) } else { (v, u) };
        if a[u] == a[v] {
            uf.union(u, v);
        } else {
            g[a[u]].push((u, v));
        }
    }
    // for &(u, v) in uv.iter() {
    //     let (uu, vv) = (uf.find(u), uf.find(v));
    //     let (uu, vv) = if a[uu] <= a[vv] { (uu, vv) } else { (vv, uu) };
    //     g[uu].push((u, v));
    // }

    // score[i] := 0..=i までのスコア
    let mut score: HashMap<usize, i64> = HashMap::new();
    score[&uf.find(0)] = 1;
    for gi in g.iter() {
        for &(u, v) in gi.iter() {
            let (uu, vv) = (uf.find(u), uf.find(v));
            if score[uu] > 0 {
                score[vv] = max(score[vv], score[uu] + 1);
            }
        }
    }
    eprintln!("{:?}", score);

    let ans = score[uf.find(n - 1)];
    println!("{}", ans);
}
