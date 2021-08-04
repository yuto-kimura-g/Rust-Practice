#![allow(unused_variables)]
#![allow(dead_code)]
// いまさらだけど，rustでは要らない？使用しない変数の名前には先頭にアンダーバーを付けるらしい
// 付けないとコンパイルでwarningが出る(errorじゃなくてwarningなので一応動く)
// 上記の様に，#![allow(unused_variables)]を付けたら無視される
// 一番上に記述すると全ての関数に適用されるが，特定の関数のみに適用したい場合は
// #![allow()]
// fn hoge() { return; }
// このようにその関数の直前に書くと良い


/***** 3.1 *****/
fn first() {
    // rustの変数は基本的に変更不可(immutable)
    let x = 1;
    println!("immutable value : {}", x);

    // ただし，シャドーイング(上書き?)は出来る
    let x = x + 1;
    println!("immutable value : {}", x);

    // 変数を更新したい場合はmutオプションを付ける
    let mut y = 1;
    println!("mutable value : {}", y);
    y = 2;
    println!("mutable value : {}", y);

    // 例えば，同じ変数名に違う型の値を入れたい時にはmutにしてもエラーになるが，
    // シャドーイングすると出来る
    let name: String = "tanaka".to_string();
    let name: usize = name.len();
    println!("name length : {}", name);
    // これは出来るけど
    // let mut name: String = "tanaka".to_string();
    // name: u32 = name.len();
    // これは出来ない(許可されていない)

    // 定数はletではなくconstで宣言
    // (rustでは一般的にUPPER_SNAKE_CASEで命名される)
    const MAX_NUMBER: u32 = 100_000;
    println!("constant value : {}", MAX_NUMBER);
    return;
}


/***** 3.2 *****/
fn second() {
    // (変数名): <型>と書くことを型注釈という
    // rustは静的型付け言語なので，コンパイル時に全ての変数の型を把握しておく必要がある
    // この例の場合，型注釈を付けなければコンパイルエラーになる
    let number: u32 = "1024".parse().expect("not a number");

    // 整数型一覧
    // i8, i16, i32, i64 / u8, u16, u32, u64 → 基準型(推奨？)はi32
    // isize / usize (コンピュータのアーキテクチャに依存(32 or 64bit))
    // f32, f64 ： IEEE-754規格，基準型はf64
    // bool (true / false)

    // 整数リテラル(表現の形式)
    let x = 123u32; //型接尾辞
    let x = 100_000; // 100_000のように３桁ずつ_で区切っても良い？
    let x = 0xff; // 16進数の型接頭辞
    let x = 0o77; // 8進数
    let x = 0b11; // 2進数
    let x = b'A'; // byte？(u8のみ)

    // rustの文字型(文字列ではない)はアスキーコードではなくユニコード(Cとかはアスキーコード)
    // そのため，英語,日本語,絵文字,ゼロ幅スペース？など多くの文字を扱える．すごい
    let emoji: char = '😊';
    println!("emoji : {}", emoji);

    let tuple: (i32, f64, char) = (123, 3.1415, 'a');
    let first_value = tuple.0;
    let second_value = tuple.1;
    let (x, _, z) = tuple;
    // タプルは異なる型が含まれていても良い
    // アクセスの際は .(ドット)を使用する(0-indexed)
    // まとめて取り出すことも可能

    let array = [1, 2, 3];
    let first_value = array[0];
    // let invalid_access = array[128];
    // 配列のインデックスアクセスはドットではなく括弧．ややこしいな
    // ちなみに配列外参照をするとその時点でpanic('プログラムの異常終了'のrust用語)する
    // rustが安全と言われる所以の一つがこれ．さすがですね
    return;
}


/***** 3.3 *****/
fn third() {
    // 関数の引数の書き方はPythonと同じ
    // fn hoge(x: i32, y: char) -> f64 {
        // return 3.14
    // }
    // let pi: f64 = hoge(123, 'a');
    // みたいな感じ

    // '式'と'文'の区別
    // let x = 1; // これは文(値を返さない)
    // x + 1 // これは式(x + 1という値を返す)．
    // x + 1; //式の後ろにセミコロンを付けると文になる
    // C言語だと，x = y = 6みたいに区別されないがrustでそんなことは出来ない
    return;
}


/***** 3.4 *****/
fn fourth() {
    // 今更だけどコメントの書き方
    // 1行コメント
    /*
     * 複数行コメント
     */
    // /// ドキュメント コメント？ その1
    // //! ドキュメントコメント その2
    // ドキュメントコメントについて詳しくはchap14を参照．
    // そのまま書くとエラーになるのでコメントアウトしている
    return;
}


/***** 3.5 *****/
fn fifth() {
    // CやPythonだと
    // x = 1
    // flag = True
    // if x and flag:
    // のように論理値と数値両方で条件式を書けたが，rustでは論理値のみが使える
    // つまりif x != 0とすれば他の言語で言うif x:が実現できる

    // ループはloop, while, forの3種類
    loop {
        println!("this is infinity loop");
        break;
    }

    let mut idx = 0;
    while idx < 3 {
        println!("while loop");
        idx = idx + 1;
    }

    let array = [1, 2, 3, 4, 5];
    for ele in array.iter() {
        println!("range based for? ele : {}", ele);
    }

    for num in (1..4).rev() {
        println!("for loop. num : {}", num);
    }
    // 見てわかる通り，forループが一番安全なのでよく使われている
    return;
}


fn fib(n: i64) -> i64 {
    if n <= 0 {
        println!("invalid value");
        -1
    } else if n == 1 || n == 2{
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn practice() {
    println!("input number : ");
    let mut buffer: String = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .ok();
    let n: i64 = buffer.trim().parse().expect("faild");
    println!("fib[{}] : {}", n, fib(n));
}


fn main() {
    // first();
    // second();
    // third();
    // fourth();
    // fifth();
    practice();
    return;
}
