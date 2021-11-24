pub use cpp_gen_in_proc_macro_impl::bar;

mod baz;

#[bar]
fn foo() -> i32 {}

#[test]
fn test_ok() {
    assert_eq!(foo(), 42);
}
