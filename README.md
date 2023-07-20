# Rust Practice

` rust ` のおべんきょう

## Gettings Started

` WSL2 + VSCode ` を使用する

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```bash
source "$HOME/.cargo/env"
```

```bash
code .
```

VSCodeで拡張機能をインストール
- `rust-lang.rust-analyzer`

```bash
rustup component add rust-src
rustup component add rust-analysis
rustup component add rustfmt
rustup component add clippy
```

ここまでやれば，コード補完，実行，デバッグまで一通り出来るようになる

## References
### Ja
#### 環境構築
- <https://www.rust-lang.org/ja/tools/install>
- <https://doc.rust-jp.rs/book-ja/ch01-01-installation.html>
- <https://doc.rust-jp.rs/book-ja/appendix-04-useful-development-tools.html>
#### 全般
- <https://www.rust-lang.org/ja/learn>
- <https://doc.rust-jp.rs/book-ja/>
- <https://doc.rust-jp.rs/rust-by-example-ja/index.html>

### Eng
- <https://doc.rust-lang.org/book/>
