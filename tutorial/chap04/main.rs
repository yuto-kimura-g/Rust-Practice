#![allow(unused_variables)]
#![allow(dead_code)]


fn first() {
    // 所有権に関するルール
    // 1. Rustの各値は所有者と呼ばれる変数と対応している
    // 2. いかなる場合も所有者は一つ
    // 3. 所有者がスコープから外れた時，所有されていた値は破棄される

    // String型変数はヒープ領域に確保される
    let text = "hello world"; // text変数はstack領域に確保される
    println!("text : {}", text);
    let mut text_string: String = String::from("hello world"); // text_string変数はheap領域に確保される
    text_string.push_str(", this is add text by push function");
    println!("text string : {}", text_string);

    // シャロ―コピーとディープコピー
    // 文字列は基本的にシャロ―コピーで動作する(文字列はサイズは不明なので，いちいち全コピーしていたら低速になるから)
    let str1 = String::from("hoge");
    let str2 = str1;
    // shallow copy．str2とstr1は両方ヒープ領域の中の同じ場所を参照する．
    // (str1, str2の情報(pointer, length, capacity)はstack領域の別の場所に確保される)
    // println!("str1 : {}", str1); // コンパイラがs1は不要だと判断して既に無効化しているから，これはエラーになる
    let str3 = str2.clone(); // deep copy．ヒープ領域のデータまでコピーする．
    println!("str1 : {}", str3); // 正しく動作する

    // 数値の場合は宣言した時点でサイズが確定するので，ヒープを使わずスタックだけで済む(ヒープはサイズが可変の時に使う場所)
    // → シャロ―コピーもくそもない
    let x = 128;
    let y = x;
    println!("(x, y) : ({}, {})", x, y); // これは正常に動作する

    // 数値と文字列の違い
    let sample = String::from("hello"); // stack, heap領域に変数が確保される
    // string_function(sample); // 関数に渡された(ムーブされた)
    // この時点でsampleは無効
    let z = 128; // stack領域に確保
    // integer_function(z);
    // この時点でzは普通に使える(有効)

    // fn string_function(text: String) {
    //     return;
    // } // この時点でtextに対してdropが呼ばれ，heapが解放される

    // fn integer_function(num: i32) {
    //     return;
    // } // numがスコープを抜ける．そもそもstack領域にある変数なのでpopされるだけ
    return;
}


fn calc_length(s: &String) -> usize {
    // (不変として)借用しているので，sを変えることは出来ない(所有権が無いから)
    // s.push_str("error maker"); // これはエラーになる
    s.len()
}

fn could_change(s: &mut String) {
    s.push_str(", ok"); // 可変として借用したので変更できる
    return;
}

// fn dangle() -> &String {
//     let str = String::from("tmp");
//     &str // strはheapに確保されているのでこの関数を抜けた時点で解放される．
// } // 解放された後の変数のアドレスなど無いからこんなことをしたらだめだけど，それをコンパイラが見つけてくれるよという話

fn no_dangle() -> String {
    let str = String::from("wow");
    str // この時点で所有権がsecondに移る
}

fn second() {
    // 4.1の例では値を直接受け渡ししていたので所有権がめんどくさかった
    let mut str1 = String::from("hello");
    let str_len = calc_length(&str1); // このように参照渡しすれば所有権はこの関数(second)に残る
    println!("the length of {} : {}", str1, str_len); // 所有権が残ったままだからここでstr1を使える
    could_change(&mut str1); // このように参照渡しをすると，参照先で値を変えられる(ミュータブル)

    // この他にも
    // ・同時に二つ以上の'可変の借用'は出来ない(ただし，新たにスコープ{}を作って順番を付ければOK)
    // ・'不変の借用'中に可変の借用は出来ない
    // 等のルールがある

    // 安全性を謳うだけあって，Rustはダングリング(dangling)をコンパイラが検知してくれる
    // let reference_nothing = dangle();
    let str_from_function = no_dangle(); // ちなみにこれは所有権が移っているので解放されない→エラーにならない
    return;
}


fn get_first_word(str: &str) -> &str { //&strは文字列スライスを表す型
    let bytes_str = str.as_bytes(); // Stringオブジェクトをbytes配列に変換
    for (idx, &ele) in bytes_str.iter().enumerate() {
        if ele == b' ' { // b' 'で空白文字のバイトを表す
            return &str[0..idx];
        }
    }
    return &str[..];
    // return strとしたい所だけど，複数の単語が存在した場合はスライスを返すので，それと型を合わせるためにこっちもスライスを返している
}

fn third() {
    // スライスは&str[s..t];でstrのs文字目からt - 1文字目までを切り出す(Pythonと同じ)
    // &を付けているのでこれも参照
    let hello_world = String::from("hello world");
    let hello = &hello_world[0..5];
    let world = &hello_world[6..11];
    // ちなみに，
    let same_str = &hello_world[..];
    // とすれば文字列全体への参照のスライス(つまり&sと同値)を得られる

    // スライスを利用して，文字列の中から先頭の単語を抜き出す関数を考えると
    // ちなみに&hello_worldは不変な借用．これは文字列リテラル(Stringは文字列であり，文字列リテラルではない)が不変である理由
    let first_word = get_first_word(&hello_world[..]); // 文字列(String)をスライス([..])して，借用(&)した → 文字列スライスの不変な借用
    let literal_hello_world = "literal hello world"; // literal_hello_world(文字列リテラル)はそれ自体が文字列スライス
    let first_word = get_first_word(&literal_hello_world); // なので，&literal_hello_worldで文字列スライスの不変な借用となる

    // 数値型でも考え方は同じ．あくまで所有権は無く，参照(不変)であることに注意
    let array = [1, 2, 3, 4, 5];
    let array_slice: &[i32] = &array[1..3]; // 型は&[i32]という変なのになる
    return;
}


fn main() {
    first();
    second();
    third();
    return;
}
