use ferris_says;
use std;

fn main() {
    let stdout = std::io::stdout();
    let mut writer = std::io::BufWriter::new(stdout.lock());
    let message = String::from("hello with ferris");
    let message_len = message.chars().count();
    ferris_says::say(message.as_bytes(), message_len, &mut writer).unwrap();
}

/*
MEMO:
詳しくは，cargo new --helpを参照

hoge@hige $ cargo new <project name> --bin
(project name)という名前のプロジェクト(gitファイルとかcargo.tomlファイルとか色々)を作成

hoge@hige $ cargo init <project name> --bin
(project name)という名前のプロジェクトを作成
git関係のファイルと，cargo.lockファイル，targetディレクトリは作らない
チュートリアルではnewよりinitの方が便利かも

hoge@hige $ cargo build
(rustc?でコンパイル)
リリース用の最適化オプション --release

hoge@hige $ cargo check
(コンパイルが通るかだけ確認．実際にコンパイルはしない)

hoge@hige $ cargo run
(コンパイルして実行まで)

hoge@hige $ cargo test
テスト？を実行してくれる

hoge@hige $ cargo install clippy
hoge@hige $ cargo clippy
rust公式のリンターを適用

hoge@hige $ rustup component add rustfmt
hoge@hige $ cargo fmt
現在のクレートの全ての.rsコードを自動フォーマット

hoge@hige $ cargo fix
コンパイル時のwarning(使ってない変数名をアンダーバー始まりにするとか)を自動で直してくれる

hoge@hige $ cargo tree
依存関係？を木構造で表示

*/
