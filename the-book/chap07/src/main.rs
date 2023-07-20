
use::std::collections::HashMap;
// rustの慣例として，構造体，enumなど関数以外の要素をuseで持ち込むときはフルパスで書くらしい

// use std::collections::*;
// これもPythonと同じく，全てのモジュールをまとめて持ち込むことも可能
// *をglob演算子という

// use std::fmt;
// use std::io;
// 後は同じ名前だけど違うモジュールを同じスコープに持ち込むときはフルパスで書いたらダメ(区別できなくなる)

// use std::{fmt::Result, io};
// 同じ親を持つものは一行で持ち込むことも出来る

// use std::io::Result as IoResult;
// このように名前を変えて同じモジュールを区別しても良い
// Pythonでいうfrom math import factorial as fact 的な感じ

mod other;
// modの後に定義を書くのではなくセミコロンを付けると他のファイルから読み込むようにrustに命令する
pub use crate::other::other_mod;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    // fmt::Result
    // io::Result<()>
    // 二つともResultだから，親モジュールで区別しないといけない

    println!("call other file pub function in pub mod");
    other_mod::other_fn(); // 他のファイルのモジュールの中の関数を実行
    return;
}
