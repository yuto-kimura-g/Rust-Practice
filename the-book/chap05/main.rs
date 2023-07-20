#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]


// 構造体の定義．フィールド名: 型
struct User {
    username: String,
    // &str(文字列スライス型)ではなく，所有権があるString型を使用している
    // &str型にするとエラーになる (user1, 2のインスタンスが所有権を持っていないと，ライフタイム指定子が無い
    // (つまり，どのタイミングで無効化して良いのかコンパイラが分からない)からだめ)

    active: bool,
    sign_in_cnt: u64
} // セミコロン要らない

struct Color(i32, i32, i32); // タプル構造体の定義

fn make_user(username: String) -> User {
    // 少し前(chap3.3)に出てきたけど，User {} と return User{}; は同じ動作をする
    // 式そのままか文をreturnするかの違い
    return User {
        // username: username, // フィールド名: ユーザー名
        username, // 仮引数名と構造体のフィールド名が同じなので，このように省略した初期化も可能
        active: true,
        sign_in_cnt: 1
    };
}

fn first() {
    // 構造体のインスタンスを生成
    let mut user1 = make_user(String::from("hoge"));
    user1.username = String::from("hige"); // Cと同じくドットでアクセス

    let mut user2 = User {
        username: String::from("foobar"),
        ..user1 // 明示的に指定されていないフィールドの値をuser1の値で初期化する
    }; // セミコロン要る

    println!("user name : {}, {}", user1.username, user2.username);

    let black = Color(0, 0, 0);
    return;
}


#[derive(Debug)] // 構造体のデバッグ出力をすることをコンパイラに伝える
struct RectAngle {
    width: u32,
    height: u32,
}

fn calc_area(rectangle: &RectAngle) -> u32 {
    // return (rectangle.width * rectangle.height); // 括弧を付けるとwarningが出る
    return rectangle.width * rectangle.height;
}

fn second() {
    let rect1 = RectAngle {width: 30, height: 50};

    println!("rect1 : {:#?}", rect1);
    // {:?}を付けて，構造体の定義の時に#deriveしていると，構造体をまとめて出力できる．便利！
    // 書式指定？を{:#?}にすると，成型して表示してくれる
    // デバッグトレイト？というらしい

    println!("the area of rectangle : {}", calc_area(&rect1));
    return;
}


// implementation(実装)ブロック
impl RectAngle {
    // area メソッド
    fn area(&self) -> u32 { // self := RectAngle型
        return self.width * self.height;
    }

    // 引数の多いメソッド
    fn can_include(&self, other_rect: &RectAngle) -> bool {
        return other_rect.width < self.width && other_rect.height < self.height;
    }

    // 関連関数(selfを引数に取らないメソッド)
    fn make_square(size: u32) -> RectAngle {
        return RectAngle { width: size, height: size };
    }
}

fn third() {
    let rect1 = RectAngle { width: 30, height: 50 };
    let rect2 = RectAngle { width: 10, height: 40 };
    let rect3 = RectAngle { width: 80, height: 60 };
    println!("the area of the rectangle : {}", rect1.area());
    println!("rect2 : {:#?}", rect2);
    println!("can rect1 include rect2 ? : {}", rect1.can_include(&rect2));
    println!("rect3 : {:#?}", rect3);
    println!("can rect1 include rect3 ? : {}", rect1.can_include(&rect3));

    let rect_square = RectAngle::make_square(rect1.width);
    // 関連関数の呼び出しはコロン二つ．String::fromとかstd::io::stdinとかと同じ
    println!("square : {:#?}", rect_square);
    return;
}


fn main() {
    first();
    second();
    third();
    return;
}
