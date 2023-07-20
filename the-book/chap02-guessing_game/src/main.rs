// ライブラリの読み込み．例えば一行目は標準ライブラリ(std)の中の入出力ライブラリ(io)を読み込んでいる
// Pythonだと，from std import ioのイメージ
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1, 101); // 1 - 100の乱数を生成．immutable(変更不可)．定数ではない．
    println!("secret number : {}", secret_number);

    loop {
        println!("input your guess");
        let mut guess: String = String::new(); // 空の文字列を作成

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        // 文字列として，標準入力を受け取り．エラー処理も書かないとコンパイル時に警告が出る

        // キャストversion1
        // let guess: u32 = guess.trim().parse().expect("please type a number. not str, emoji, and so on");
        // .trim()で改行文字を取り除き，.parse()でキャスト．キャスト先の型は左辺のものをコンパイラが型推論してくれる
        // 入力時と同様，エラー処理を書かないとコンパイルエラーになる

        // キャストversion2
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        // parseメソッドの戻り値はResultという型であり，Result型はOkかErrを取り得るenum型．
        // .expect()の代わりにmatchを使っている
        // Errの引数?は使わない値という意味で '_'(アンダーバー)しかだめっぽい？

        println!("you guessed : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("match! game end");
                break;
            },
        }
    }
    return;
}
