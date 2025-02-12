use hello_rust::string_utils::reverse;
use hello_rust::math::{add, subtract, multiply, divide};

#[test]
fn test_math_functions() {
    assert_eq!(math::add(5, 5), 10);
    assert_eq!(math::subtract(10, 3), 7);
    assert_eq!(math::multiply(4, 5), 20);
    assert_eq!(math::divide(10, 2), Some(5.0));
}

#[test]
fn test_string_reverse() {
    assert_eq!(reverse::reverse("hello"), "olleh");
}