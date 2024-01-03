use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut c = vec![];
    let mut a = vec![];
    for _ in 0..n {
        input! {
            ci: usize,
            ai: [i32; ci]
        }
        c.push(ci);
        a.push(ai);
    }
    input! {
        x: i32
    }
    // player = (cnt, id)
    let mut x_players = vec![];
    let mut min_c = std::usize::MAX;
    for (i, ai) in a.iter().enumerate() {
        if ai.contains(&x) {
            x_players.push((ai.len(), i + 1));
            min_c = min_c.min(c[i]);
        }
    }
    x_players.sort();
    let k = x_players
        .iter()
        .filter(|(c, _)| *c == min_c)
        .collect::<Vec<_>>()
        .len();
    let b = x_players
        .iter()
        .filter(|(c, _)| *c == min_c)
        .map(|(_, id)| id.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", k);
    println!("{}", b);
}
