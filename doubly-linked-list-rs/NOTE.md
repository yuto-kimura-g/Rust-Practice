# NOTE

> 脆弱エンジニアの日常, 低レイヤ―ガールと学ぶRustシリーズ
> <https://www.youtube.com/playlist?list=PLIuwYg2kLfAYN2WQqVYViYCoqsPXboP0V>

## chap 1
- cargo init
- cargo new
- cargo build
- cargo run
- cargo clean

`cargo clean` 以外は知ってた

## chap 2
- rust-analyzer
- fn
- unimplemented!()
- impl
- Option

`()` ユニット型ってあんまり使った記憶無いな...
voidと何が違うんだろう？
つまり，
```rust
fn f() -> () {
    ()
}
```
と
```rust
fn f() {
}
```
の違いが分からん
→ ChatGPTに聞いてみたら，同じみたい．ユニット型の方がより明示的という感じらしい．

## chap 3
- RUST_BACKTRACE=1
- rustc --explain Exxx
- cargo fmt
- Box
- let
- Some
- clone()

amend
- `git commit -m <hoge>`
のコミットメッセージを修正したい時，push前なら
- `git commit --amend -m <foo>`
とすればできる．便利

Box
- Box: heap上にオブジェクトを確保する．
- 通常はstack上にオブジェクトを確保する．

以下はChatGPTに「rustのBox型を説明」させた結果．だいたい合ってそう？
```txt
Box<T>の特徴
1. ヒープ割り当て:
  Box<T>はヒープメモリを確保し、その中にT型のデータを格納します。これにより、スタックにはポインタのみが格納され、実際のデータはヒープに存在します。
2. サイズの固定:
  スタック上のBox<T>自体のサイズは一定（通常はポインタのサイズ）で、ヒープ上のデータのサイズに依存しません。これは、コンパイル時にサイズが定まらない型を扱う際に役立ちます。
3. 所有権と生存期間の管理:
  Box<T>は所有権を持つポインタであり、Box<T>がスコープを抜けると、自動的にヒープ上のデータも解放されます。これにより、メモリリークのリスクを減少させます。
4. 再帰型のサポート:
  自身をフィールドとして含む構造体（再帰型）は、通常、そのサイズがコンパイル時に計算不可能です。しかし、Box<T>を使用することで、再帰的なデータ構造を作ることができます。
```

## chap 4
## chap 5
