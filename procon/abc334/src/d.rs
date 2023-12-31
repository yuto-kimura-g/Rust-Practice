use proconio::input;
use superslice::Ext; // upper_bound

fn main() {
    input! {
        n: usize,
        q: usize,
        mut r: [i64; n],
        query: [i64; q],
    }

    r.sort();
    let mut prefix_sum = vec![0];
    for &ri in &r {
        prefix_sum.push(prefix_sum.last().unwrap() + ri);
    }
    dbg!(&prefix_sum); // prints to stderr

    let binary_search = |mut ok: isize, mut ng: isize, x: &i64| -> usize {
        // let is_ok = |m: isize| -> bool {
        //     // template
        //     todo!();
        // };
        // let is_ok = |m: isize| -> bool {
        //     // lower bound
        //     *x <= prefix_sum[m as usize]
        // };
        let is_ok = |m: isize| -> bool {
            // upper bound
            *x < prefix_sum[m as usize]
        };

        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if is_ok(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    };

    for &x in &query {
        assert_eq!(
            binary_search(prefix_sum.len() as isize, -1, &x),
            prefix_sum.upper_bound(&x)
        );
        let ans = binary_search(prefix_sum.len() as isize, -1, &x) - 1;
        // let ans = prefix_sum.upper_bound(&x) - 1;
        println!("{}", ans);
    }
}
