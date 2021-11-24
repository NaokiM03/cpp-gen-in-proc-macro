use crate::foo;

#[test]
fn test_err() {
    assert_ne!(foo(), 41);
}

// use crate::bar;

// #[bar]
// fn foo() -> i32 {}

// #[test]
// fn test_ok() {
//     assert_eq!(foo(), 42);
// }
