//! # Documentation Test

// inner: about parent item
// outer: about the just following item

// line comment: recommended

/*
 * block comment: NOT recommended
 */

//! inner line document comment: crate

/*!
 * inner block document: NOT recommended
 */

/// outer line document: mod, fn, etc...

/**
 * outer block document: NOT recommended
 */

/// Add
///
/// # Examples
///
/// ```
/// assert_eq!(2f64, documentation_test::div(42, 21));
/// ```
///
/// # Panics
///
/// the following code will panic if y is zero.
///
/// ```should_panic
/// documentation_test::div(42, 0);
/// ```
pub fn div(x: i32, y: i32) -> f64 {
    if y == 0 {
        panic!("zero division error");
    }
    (x / y).into()
}

/// hoge module
pub mod hoge {

    /// f function
    pub fn f() {
        println!("called Hoge::f()");
    }
}

/// Foo enum
pub enum Foo {
    Oh,
    Yeah,
}

/// Bar struct
pub struct Bar {
    /// public field
    pub foo: Foo,
    /// private field
    foofoo: Foo,
}

/// Bar impl
impl Bar {
    /// link: [Foo]
    pub fn new() -> Bar {
        Bar {
            foo: Foo::Oh,
            foofoo: Foo::Yeah,
        }
    }
}

/// Piyo struct
/// this is private struct
struct Piyo {
    foo: Foo,
}
