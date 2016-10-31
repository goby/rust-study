

#[test]
#[should_panic(expected = "assertion failed")]
fn should_panic() {
    assert_eq!(1,2);
}

#[test]
fn main() {
    let x = 5;
    let raw = &x as *const i32;

    // assert_eq!(5, *raw);
    let value = unsafe { *raw };
    assert_eq!(5, value);
}
