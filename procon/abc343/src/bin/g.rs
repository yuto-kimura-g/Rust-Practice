use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};

#[fastout]
fn main() {
    let input = Input::new();
    solver::solve(input);
}

/// 入力によって一意に定まる情報
pub struct Input {
    /// 1 <= n <= 20
    n: usize,
    /// 1 <= \sum |s_i| <= 2*10^5
    s: Vec<Vec<char>>,
    /// overlap(X, Y): Xの末尾k文字とYの先頭k文字が一致する最大のk
    overlap: Vec<Vec<usize>>,
}
impl Input {
    fn new() -> Self {
        input! {
            n: usize,
            mut s: [Chars; n],
        }

        // sの重複を消す
        s.dedup();
        // 完全に包含されていて要らないsを消す
        let mut is_unique = vec![true; n];
        for (i, si) in s.iter().enumerate() {
            for (j, sj) in s.iter().enumerate() {
                if i == j {
                    continue;
                }
                if string::is_contain(si, sj) {
                    is_unique[j] = false;
                }
            }
        }
        let s = s
            .iter()
            .enumerate()
            .filter(|&(i, _)| is_unique[i])
            .map(|(_, si)| si.clone())
            .collect_vec();
        // reset
        let n = s.len();

        // z_algorithm を使ってoverlap(X, Y)を前計算
        // overlap(X, Y): Xの末尾k文字とYの先頭k文字が一致する最大のk
        let mut overlap = vec![vec![0; n]; n];
        for (i, si) in s.iter().enumerate() {
            let ni = si.len();
            for (j, sj) in s.iter().enumerate() {
                let nj = sj.len();
                if i == j {
                    continue;
                }
                let sep = vec!['#'];
                let t = [sj.clone(), sep, si.clone()].concat();
                let z = string::z_algorithm(&t);
                for (k, &zk) in z.iter().enumerate().skip(ni) {
                    let x = ni + nj + 1 - k;
                    if zk == x {
                        overlap[i][j] = max(overlap[i][j], x);
                    }
                }
            }
        }
        #[cfg(feature = "local")]
        {
            eprintln!("n = {}", n);
            eprintln!("s:");
            for si in s.iter() {
                eprintln!("{:?}", si);
            }
            eprintln!("overlap:\n\\|{:?}", (0..n).collect_vec());
            for (i, fi) in overlap.iter().enumerate() {
                eprintln!("{}|{:?}", i, fi);
            }
        }

        Input { n, s, overlap }
    }
}

mod solver {
    use super::*;

    pub fn solve(input: Input) {
        // dp[subset][i]:
        //      点 s ∈ set から出発し，
        //      subset ⊆ set を全て経由し，
        //      点 i ∈ subset で終わる経路の最小コスト
        // dp[{i}][i] = |s_i|
        // O(2^n n^2)
        let mut dp = vec![vec![usize::MAX; input.n]; 1 << input.n];
        for i in 0..input.n {
            dp[1 << i][i] = input.s[i].len();
        }
        for subset in 0..(1 << input.n) {
            // dp[subset][i] から dp[subset | (1 << j)][j] へ遷移する
            for i in 0..input.n {
                if ((subset >> i) & 1) == 0 {
                    // i ∉ subset なのでダメ
                    // i から遷移できない
                    continue;
                }
                for j in 0..input.n {
                    if ((subset >> j) & 1) == 1 {
                        // j ∈ subset なのでダメ
                        // subset に既に j が含まれているので，subset の末尾に j を追加できない
                        continue;
                    }
                    // s_i -> s_j の順に並べる遷移
                    dp[subset | (1 << j)][j] = min(
                        dp[subset | (1 << j)][j],
                        dp[subset][i] + input.s[j].len() - input.overlap[i][j],
                    );
                }
            }
        }
        // bit_mask: 0b11...1
        let bit_mask = (1 << input.n) - 1;
        let ans = *dp[bit_mask].iter().min().unwrap();
        println!("{}", ans);
    }
}

/// 文字列ライブラリ
mod string {
    /// # Z Algorithm
    /// * Longest Commom Prefix (LCP) を求めるアルゴリズム
    /// * 文字列sに対して，z(i) = sとs(i:)の最長共通接頭辞の長さ を返す
    /// * 時間計算量: `O(|S|)`
    /// * 空間計算量: `O(|S|)`
    #[allow(dead_code)]
    pub fn z_algorithm(s: &[char]) -> Vec<usize> {
        let n: usize = s.len();
        let mut z: Vec<usize> = vec![0; n];
        z[0] = n;
        let mut head = 1; // 今見ている区間の左端
        let mut offset = 0; // 今見ている区間の幅（共通接頭辞の長さ）
        while head < n {
            while head + offset < n && s[offset] == s[head + offset] {
                offset += 1;
            }
            z[head] = offset;
            if offset == 0 {
                head += 1;
                continue;
            }
            let mut z_index = 1;
            while head + z_index < n && z_index + z[z_index] < offset {
                z[head + z_index] = z[z_index];
                z_index += 1;
            }
            head += z_index;
            offset -= z_index;
        }
        z
    }

    /// s が t を含むかどうか，z_algorithm を使って求める
    /// * 時間計算量: `O(|S| + |T|)`
    /// * 空間計算量: `O(|S| + |T|)`
    #[allow(dead_code)]
    pub fn is_contain(s: &[char], t: &[char]) -> bool {
        let sep = vec!['#'];
        let u = [t, sep.as_slice(), s].concat();
        let z = super::string::z_algorithm(&u);
        for &zi in z.iter().skip(t.len() + 1) {
            // skip: len(t) + len(sep)
            if zi >= t.len() {
                // z(m+i)つまりs(i)と，z(0)つまりt(0)からの共通接頭辞の長さがt.len()以上
                // sはtを含む
                return true;
            }
        }
        false
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn test_z_algorithm() {
            // Copilotくんが作ったやつ
            let s: Vec<char> = "abababab".chars().collect();
            let z: Vec<usize> = super::z_algorithm(&s);
            assert_eq!(z, vec![8, 0, 6, 0, 4, 0, 2, 0]);

            // ref: https://qiita.com/Pro_ktmr/items/16904c9570aa0953bf05
            let s: Vec<char> = "momomosumomomomo".chars().collect();
            let z: Vec<usize> = super::z_algorithm(&s);
            assert_eq!(z, vec![16, 0, 4, 0, 2, 0, 0, 0, 6, 0, 6, 0, 4, 0, 2, 0]);
        }

        #[test]
        fn test_is_contain() {
            let s: Vec<char> = "abcdef".chars().collect();
            let t: Vec<char> = "abc".chars().collect();
            let u: Vec<char> = "def".chars().collect();
            let v: Vec<char> = "xyz".chars().collect();
            assert!(super::is_contain(&s, &t));
            assert!(super::is_contain(&s, &u));
            assert!(!super::is_contain(&s, &v));
        }
    }
}
