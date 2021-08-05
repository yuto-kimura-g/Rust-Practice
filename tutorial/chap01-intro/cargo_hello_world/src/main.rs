use std;
use ferris_says;

fn main() {
    let stdout = std::io::stdout();
    let mut writer = std::io::BufWriter::new(stdout.lock());
    let message = String::from("hello with ferris");
    let message_len = message.chars().count();
    ferris_says::say(message.as_bytes(), message_len, &mut writer).unwrap();
}


/*
(project name)という名前のプロジェクト(gitファイルとかcargo.tomlファイルとか色々)を作成
詳しくは，cargo new --helpを参照
hoge@hige $ cargo new <project name> --bin

(rustc?でコンパイル)
hoge@hige $ cargo build

(コンパイルが通るかだけ確認．実際にコンパイルはしない)
hoge@hige $ cargo check

(コンパイルして実行まで)
hoge@hige $ cargo run

*/
