//! # Math Utils
//! 
//! `math_utils` is a collection of math utilities.
//! 
//! ## Functions
//! 
//! * factorial
//! * greatest_common_divisor
//! * is_prime
//! 
/// Factorial
/// # Examples
/// ```
/// use math_utils::math_utils::factorial;
/// 
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(2), 2);
/// assert_eq!(factorial(3), 6);
/// ```
pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

/// Greatest Common Divisor
/// # Examples
/// ```
/// use math_utils::math_utils::greatest_common_divisor;
/// 
/// assert_eq!(greatest_common_divisor(0, 0), 0);
/// assert_eq!(greatest_common_divisor(0, 1), 1);
/// assert_eq!(greatest_common_divisor(1, 0), 1);
/// assert_eq!(greatest_common_divisor(1, 1), 1);
/// assert_eq!(greatest_common_divisor(9, 8), 1);
/// ```
pub fn greatest_common_divisor(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

/// Is Prime
/// # Examples
/// ```
/// use math_utils::math_utils::is_prime;
/// 
/// assert_eq!(is_prime(0), false);
/// assert_eq!(is_prime(1), false);
/// assert_eq!(is_prime(2), true);
/// assert_eq!(is_prime(3), true);
/// assert_eq!(is_prime(4), false);
/// ```
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(6), 720);
        assert_eq!(factorial(7), 5040);
        assert_eq!(factorial(8), 40320);
        assert_eq!(factorial(9), 362880);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_greatest_common_divisor() {
        assert_eq!(greatest_common_divisor(0, 0), 0);
        assert_eq!(greatest_common_divisor(0, 1), 1);
        assert_eq!(greatest_common_divisor(1, 0), 1);
        assert_eq!(greatest_common_divisor(1, 1), 1);
        assert_eq!(greatest_common_divisor(1, 2), 1);
        assert_eq!(greatest_common_divisor(2, 1), 1);
        assert_eq!(greatest_common_divisor(2, 2), 2);
        assert_eq!(greatest_common_divisor(2, 3), 1);
        assert_eq!(greatest_common_divisor(3, 2), 1);
        assert_eq!(greatest_common_divisor(3, 3), 3);
        assert_eq!(greatest_common_divisor(3, 4), 1);
        assert_eq!(greatest_common_divisor(4, 3), 1);
        assert_eq!(greatest_common_divisor(4, 4), 4);
        assert_eq!(greatest_common_divisor(4, 5), 1);
        assert_eq!(greatest_common_divisor(5, 4), 1);
        assert_eq!(greatest_common_divisor(5, 5), 5);
        assert_eq!(greatest_common_divisor(5, 6), 1);
        assert_eq!(greatest_common_divisor(6, 5), 1);
        assert_eq!(greatest_common_divisor(6, 6), 6);
        assert_eq!(greatest_common_divisor(6, 7), 1);
        assert_eq!(greatest_common_divisor(7, 6), 1);
        assert_eq!(greatest_common_divisor(7, 7), 7);
        assert_eq!(greatest_common_divisor(7, 8), 1);
        assert_eq!(greatest_common_divisor(8, 7), 1);
        assert_eq!(greatest_common_divisor(8, 8), 8);
        assert_eq!(greatest_common_divisor(8, 9), 1);
        assert_eq!(greatest_common_divisor(9, 8), 1);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
    }
}
