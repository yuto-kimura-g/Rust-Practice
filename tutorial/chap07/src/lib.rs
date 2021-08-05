#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]


fn serve_order() {
    return;
}


// ディレクトリのツリー構造のイメージ．ルートはcrate
mod front { // modはモジュール(Module)の意味．剰余ではない
    pub mod host {
        pub fn waitlist() {
            return;
        }
    }

    fn cook() {
        return;
    }
    fn fix_incorrect_order() {
        cook();
        super::serve_order();
        return;
    }
}


mod back {
    pub struct BreakFast {
        pub toast: String,
        fruit: String,
    }

    impl BreakFast {
        pub fn summer(toast: &str) -> BreakFast {
            return BreakFast {
                toast: String::from(toast),
                fruit: String::from("banana"),
            };
        }
    }
}


pub fn eat_at_restaurant() {
    // 絶対パス(absolute path)で関数を呼び出し
    // ./crate/front/host/waitlist.bin 的な
    crate::front::host::waitlist();

    // 相対パス(relative path)で関数を呼び出し
    front::host::waitlist();

    let mut meal = back::BreakFast::summer("rye_rice");
    meal.toast = String::from("wheat_bread");
    println!("i'd like {} toast please", meal.toast);
    return;
}


pub use crate::front::host; // 絶対パスver
// use self::front::host; // 相対パスver
// rustの慣例として，関数をuseで持ち込む場合はフルパスで書かずに一歩手前までにするらしい
// 理由は関数の定義されている場所を明確にするため？
pub fn eat_at_restaurant_easy() {
    host::waitlist();
}
