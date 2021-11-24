use crate::foo;

#[test]
fn test_err() {
    assert_ne!(foo(), 41);
}
