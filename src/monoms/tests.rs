use super::*;

#[test]
fn test_new() {
    let monom = MonomBits::new();
    assert!(monom.bits == 0);
    assert!(!monom.is_zero());
    assert!(!monom.is_one());
}
#[test]
fn test_from_int() {
    let monom = MonomBits::from(0);
    assert!(monom.bits == 1);
    let monom = MonomBits::from(1);
    assert!(monom.bits == 2);
}
#[test]
#[should_panic]
fn test_from_large_int() {
    // TODO Do not have implementation for large variable
    let _monom = MonomBits::from(1000000);
}
#[test]
fn test_from_vec() {
    let monom = MonomBits::from(vec![0]);
    assert!(monom.bits == 1);
    let monom = MonomBits::from(vec![0, 1, 2]);
    assert!(monom.bits == 7);
}
#[test]
#[should_panic]
fn test_from_large_vec() {
    // TODO Do not have implementation for large variable
    let _monom = MonomBits::from(vec![100]);
}
// Group of tests for a multiplication
#[test]
fn test_mul() {
    let a = MonomBits::from(0);
    let b = MonomBits::from(1);
    let ab = MonomBits::from(vec![0, 1]);

    let res = a * b;
    assert_eq!(ab, res);
}
#[test]
fn test_mul_bench() {
    let a = MonomBits::from(0);
    let b = MonomBits::from(1);
    let ab = MonomBits::from(vec![0, 1]);

    let res = a * b;
    assert_eq!(ab, res);
}

#[test]
fn test_is_divisible() {
    let a = MonomBits::from(0);
    let b = MonomBits::from(1);

    assert!(!a.is_divisible(&b));
    assert!(!b.is_divisible(&a));

    let _1 = MonomBits::one();
    assert!(a.is_divisible(&_1));
    assert!(!_1.is_divisible(&a));

    let _0 = MonomBits::zero();
    assert!(!_0.is_divisible(&a));
}
#[test]
#[should_panic]
fn test_is_divisible_by_zero() {
    let a = MonomBits::from(0);
    let _0 = MonomBits::zero();

    let _res = a.is_divisible(&_0);
}

#[test]
fn test_mul_right_zero() {
    let a = MonomBits::from(0);
    let _0 = MonomBits::zero();

    let res = a * _0;
    assert!(res.is_zero());
}
#[test]
fn test_mul_left_zero() {
    let a = MonomBits::from(0);
    let _0 = MonomBits::zero();

    let res = _0 * a;
    assert!(res.is_zero());
}
// Group of tests for a division
#[test]
fn test_div_res_zero() {
    let a = MonomBits::from(0);
    let b = MonomBits::from(1);

    let res = a / b;
    assert!(res.is_zero());
}
#[test]
fn test_div() {
    let a = MonomBits::from(0);
    let ab = MonomBits::from(vec![0, 1]);
    let b = MonomBits::from(1);

    let res = ab / b;
    assert_eq!(res, a);
}
