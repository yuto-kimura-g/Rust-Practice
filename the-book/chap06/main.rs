#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]


enum IpAddressKind {
    IPAddr,
    IPV4(u8, u8, u8, u8),
    IPV6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 構造体
    Write(String),
    ChangeColor(i32, i32, i32), // タプル
}

impl Message { // implはstructだけじゃなくてenumにも定義できる
    fn call(&self) { // selfを引数に取るので，これはメソッド
        return;
    }
}

fn first() {
    let ipaddr = IpAddressKind::IPAddr; // enumの要素にはコロン二つでアクセスする
    let home = IpAddressKind::IPV4(127, 0, 0, 1);
    let loopback = IpAddressKind::IPV6(String::from("::1"));

    let message = Message::Write(String::from("hello world"));
    message.call();

    let number: Option<i32> = Some(123); // Option型はSomeとNoneのenum
    let text: Option<&str> = Some("literal text");
    let absent_number: Option<i32> = None; // 他の言語で言うnullの変わりがOption None
    // rustはコンパイル時に不明なものがあったらだめ(これが安全性の所以だけど)だから，
    // Noneを入れる時は型指定(Option<>)が必要(Someで具体的な値を入れる時はどっちでも良い)

    let x: i8 = 32;
    let y: Option<i8> = Some(16);
    // let sum = x + y; // <i8>型とOption<i8>型は区別されるからこれはエラーになる
    return;
}


#[derive(Debug)]
enum UsState {
    Alaska,
    Los,
    // and so on
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn convert_cent_to_value(coin: Coin) -> u32 {
    match coin { // matchは最初に当てはまった部分の処理のみを実施してreturnする．Cで言うswitch的な感じかな？
                // matchとifは似ているけど，ifは条件文で論理値しか使えないが，matchなら色々つかえるというメリットがある
                // 今回の例では条件式にCoinというenum型を使っている
                // もしifで実装するならif coin == Coin::Pennyのようにしないといけない
        Coin::Penny => {
            println!("coin is penny");
            return 1;
        },
        Coin::Penny => {
            // もしcoinがPennyだとしても，一つ上のブロックの処理が必ず実行されるのでこのブロックには到達しない
            // (コンパイラもwarningを出してくれる)．さすがrustですね
            println!("this is not execute");
            return 100_000
        },
        Coin::Nickel => return 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state : {:?}", state);
            return 25;
        },
    }
}

fn add_value(x: Option<i32>) -> Option<i32> {
    match x { // rustではよく，matchとenumを組み合わせて使われるらしい
        None => None,
        // もしもこの行を書き忘れるとコンパイルエラーになる．これがrustの凄い所．
        // もしもifで書いていたらコンパイラは気付いてくれない．
        // 例えばポインタに対してmatchを使うと，nullの場合の処理も書かないとエラーになるからnull pointer exceptionは起こり得ない
        Some(value) => Some(value + 1),
    }
}

fn second() {
    println!("value of Penny : {}", convert_cent_to_value(Coin::Penny));
    println!("value of Nickel : {}", convert_cent_to_value(Coin::Nickel));
    println!("value of Los : {}", convert_cent_to_value(Coin::Quarter(UsState::Los)));

    let u8_value = 2u8;
    match u8_value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => (), // u8_valueが1, 2, 3の時のみに興味がある場合，アンダーバーで余事象を表せる．
                // 括弧は何も処理しないという意味
    }
    return;
}


fn third() {
    // matchは条件式が取り得る全ての値を考慮しないとコンパイルエラーになる
    // (いい所でもあるけど，場合によってはめんどくさい)
    // そこで，rustにはif-let記法というものがある
    // 例えば値が31かそれ以外かで区別したい時のmatchとif-letの記述例は以下の通り
    let u16_value = Some(31u16);
    // let u16_value: Option<u16> = None;
    match u16_value {
        Some(31) => println!("🍦食べたい"),
        _ => println!("wowow"),
    }
    if let Some(31) = u16_value {
    // 普通のifと比べて左右逆なことと，イコールが一つであることに注意
        println!("🍦食べたい");
    } else { // matchだとこの部分も必須だけど，if-letなら無くても良い
        println!("wowow");
    }
    return;
}


fn main() {
    first();
    second();
    third();
    return;
}
