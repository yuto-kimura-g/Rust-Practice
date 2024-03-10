use proconio::input;

fn main() {
    input! {
        s: String
    }
    // sに含まれる各文字の個数を数える
    let cnt = s.chars().fold([0; 26], |mut cnt, c| {
        cnt[c as usize - 'a' as usize] += 1;
        cnt
    });
    eprintln!("{:?}", cnt);
    // 出現回数が1の文字がsで何文字目か探す
    let ans = s
        .chars()
        .enumerate()
        .find_map(|(i, c)| {
            if cnt[c as usize - 'a' as usize] == 1 {
                Some(i + 1)
            } else {
                None
            }
        })
        .unwrap();
    println!("{}", ans);
}
