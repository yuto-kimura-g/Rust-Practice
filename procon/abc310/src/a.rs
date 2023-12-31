#![allow(unused_imports)]
use proconio::input;
use std::cmp::min;

fn print_type<T>(_: &T) {
    eprintln!("{}", std::any::type_name::<T>());
}

#[allow(non_snake_case, unused_variables)]
fn main() {
    input! {
        n: i32,
        p: i32,
        q: i32,
        D: [i32; n]
    }
    print_type(&n);
    print_type(&D);
    let min_d: i32 = *D.iter().min().unwrap();
    println!("{}", min(p, min_d + q));
}
