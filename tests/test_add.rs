use ry26::add;

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn test_add_with_zero() {
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(5, 0), 5);
    assert_eq!(add(0, 5), 5);
}

#[test]
fn test_add_large_numbers() {
    assert_eq!(add(1_000_000, 2_000_000), 3_000_000);
    assert_eq!(add(u64::MAX - 1, 1), u64::MAX);
}

#[test]
#[should_panic]
fn test_add_overflow() {
    let _ = add(u64::MAX, 1);
}
