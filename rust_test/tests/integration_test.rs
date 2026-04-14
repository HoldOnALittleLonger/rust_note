use rust_test::APIs::*;

#[test]
fn test_unsigned_add() {
    let ret = unsigned_add(2, 2);
    assert_eq!(ret, 4);
}

#[test]
fn test_unsigned_sub() {
    let ret = unsigned_sub(8, 4);
    assert_eq!(ret, 4);
}

#[test]
fn test_unsigned_mul() {
    let ret = unsigned_mul(4, 4);
    assert_eq!(ret, 16);
}

#[test]
fn test_unsigned_div() {
    let ret = unsigned_div(8, 4);
    assert_eq!(ret, 2);
}

#[test]
#[should_panic(expected = "zero")]
fn test_unsigned_div_panic() {
    let ret = unsigned_div(8, 0);
}
