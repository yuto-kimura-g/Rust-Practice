use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        l: usize,
        c: [usize; l],
        q: usize,
        x: [usize; q],
    }
    let mut ok: HashMap<usize, bool> = HashMap::new();
    for &ai in a.iter() {
        for &bi in b.iter() {
            for &ci in c.iter() {
                ok.insert(ai + bi + ci, true);
            }
        }
    }
    for &xi in x.iter() {
        let ans = match ok.get(&xi) {
            Some(_) => "Yes",
            None => "No",
        };
        println!("{}", ans);
    }
}
