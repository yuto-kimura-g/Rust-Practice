use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        t: String,
        n: usize,
    }
    let m = t.len();
    let mut a = Vec::with_capacity(n);
    let mut s = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            ai: usize,
        }
        a.push(ai);
        let mut si = Vec::with_capacity(ai);
        for _ in 0..ai {
            input! {
                sij: String,
            }
            si.push(sij);
        }
        s.push(si);
    }
    #[cfg(feature = "local")]
    {
        eprintln!("t={:?}", t);
        eprintln!("a={:?}", a);
        eprintln!("s={:?}", s);
        // dbg!(&s);
    }

    // dp[i][j]: i番目の袋までを使って，SとT[..j]までを一致させるために必要な最小の金額
    let inf = usize::MAX;
    #[cfg(feature = "local")]
    let inf = 9;
    let mut dp = vec![vec![inf; m + 1]; n + 1];
    dp[0][0] = 0;
    // O(n m a |s|)
    // ループ回数は高々 10^6 回
    for i in 0..n {
        'j: for j in 0..=m {
            // dp[i][j]から配る
            if dp[i][j] == inf {
                // (i,j)から配れない
                continue 'j;
            }
            // 操作２：何もしない
            dp[i + 1][j] = min(dp[i + 1][j], dp[i][j]);
            // 操作１：1円を支払い，袋iからちょうど一つの文字列を選択してSの末尾に連結
            'k: for k in 0..a[i] {
                let step = s[i][k].len();
                if j + step > m {
                    // Sが溢れる
                    continue 'k;
                }
                if t[j..(j + step)] != s[i][k] {
                    // (i+1,j+step)に配れない
                    continue 'k;
                }
                dp[i + 1][j + step] = min(dp[i + 1][j + step], dp[i][j] + 1);
            }
        }
    }
    #[cfg(feature = "local")]
    {
        for i in 0..=n {
            eprintln!("dp[{}]={:?}", i, dp[i]);
        }
    }

    let ans = dp[n][m];
    println!(
        "{}",
        if ans == inf {
            "-1".to_string()
        } else {
            ans.to_string()
        }
    );
}

// problem statement
// 空文字列Sと文字列Tを一致させたい
// 以下のいずれかの操作をi=1,...,Nについて一回ずつ行う
// 操作１：1円払って，Sの末尾に袋iの文字列s_{i,j} (jは一つだけ選ぶ) を追加する
// 操作２：なにもしない
// SとTを一致させるために必要な最小の金額を求める
// 一致させることが出来ない場合は-1を出力
