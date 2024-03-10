use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }
    for ai in a.iter() {
        let mut ans = Vec::with_capacity(n);
        for (j, &aij) in ai.iter().enumerate() {
            if aij == 1 {
                ans.push(j + 1);
            }
        }
        let ans = ans
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", ans);
    }
}
