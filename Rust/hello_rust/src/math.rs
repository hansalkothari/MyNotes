pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> Option<f64> {
    if b == 0 {
        None
    } else {
        Some(a as f64 / b as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10, 3), 7);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Some(5.0));
        assert_eq!(divide(5, 0), None);
    }
}
