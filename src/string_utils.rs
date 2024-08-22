//! String utilities
//! 
//! `string_utils` is a collection of string utilities.
//! 
//! ## Functions
//! 
//! * is_polindrome
//! * count_of_char
//! * reverse
//! 

/// Is Polindrome
/// # Examples
/// ```
/// use math_utils::string_utils::is_polindrome;
/// 
/// assert_eq!(is_polindrome("hello"), false);
/// assert_eq!(is_polindrome("hannah"), true);
/// ```
pub fn is_polindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

/// Count of Char
/// # Examples
/// ```
/// use math_utils::string_utils::count_of_char;
/// 
/// assert_eq!(count_of_char("hello", 'l'), 2);
/// ```
pub fn count_of_char(s: &str, c: char) -> usize {
    s.chars().filter(|&x| x == c).count()
}

/// Reverse
/// # Examples
/// ```
/// use math_utils::string_utils::reverse;
/// 
/// assert_eq!(reverse("hello"), "olleh");
/// ```
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_polindrome() {
        assert_eq!(is_polindrome("hello"), false);
        assert_eq!(is_polindrome("world"), false);
        assert_eq!(is_polindrome("hannah"), true);
        assert_eq!(is_polindrome("racecar"), true);
    }

    #[test]
    fn test_count_of_char() {
        assert_eq!(count_of_char("hello", 'l'), 2);
        assert_eq!(count_of_char("world", 'l'), 1);
        assert_eq!(count_of_char("hannah", 'h'), 2);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
        assert_eq!(reverse("world"), "dlrow");
        assert_eq!(reverse("hannah"), "hannah");
    }
}