//! # Date Utils
//!
//! `date_utils` is a collection of date utilities.
//! 
//! ## Functions
//! 
//! * difference_between_dates
//! * validate_date_format
//! * format_date
//! 
use chrono::NaiveDate;

/// Difference Between Dates
/// # Examples
/// ```
/// use math_utils::date_utils::difference_between_dates;
///
/// assert_eq!(difference_between_dates("2021-01-01", "2021-01-01"), 0);
/// assert_eq!(difference_between_dates("2021-01-01", "2021-01-02"), 1);
/// ```
pub fn difference_between_dates(date1: &str, date2: &str) -> i64 {
    let date1 = NaiveDate::parse_from_str(date1, "%Y-%m-%d").unwrap();
    let date2 = NaiveDate::parse_from_str(date2, "%Y-%m-%d").unwrap();
    (date2 - date1).num_days()
}

/// Validate Date Format
/// # Examples
/// ```
/// use math_utils::date_utils::validate_date_format;
///
/// assert_eq!(validate_date_format("2021-01-28"), true);
/// assert_eq!(validate_date_format("2021-31-02"), false);
/// ```
pub fn validate_date_format(date: &str) -> bool {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok()
}

/// Format Date
/// # Examples
/// ```
/// use math_utils::date_utils::format_date;
/// 
/// assert_eq!(format_date("2021-01-28", "%Y-%m-%d", "%d-%m-%Y"), "28-01-2021");
/// assert_eq!(format_date("2021-01-28", "%Y-%m-%d", "%Y-%m-%d"), "2021-01-28");
/// ```
pub fn format_date(date: &str, source_format: &str, target_format: &str) -> String {
    NaiveDate::parse_from_str(date, source_format)
        .unwrap()
        .format(target_format)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference_between_dates() {
        assert_eq!(difference_between_dates("2021-01-01", "2021-01-01"), 0);
        assert_eq!(difference_between_dates("2021-01-01", "2021-01-02"), 1);
        assert_eq!(difference_between_dates("2021-01-01", "2021-01-03"), 2);
        assert_eq!(difference_between_dates("2021-01-01", "2021-01-04"), 3);
    }

    #[test]
    fn test_validate_date_format() {
        assert_eq!(validate_date_format("2021-01-28"), true);
        assert_eq!(validate_date_format("2021-31-02"), false);
    }

    #[test]
    fn test_format_date() {
        assert_eq!(format_date("2021-01-28", "%Y-%m-%d", "%d-%m-%Y"), "28-01-2021");
        assert_eq!(format_date("2021-01-28", "%Y-%m-%d", "%Y-%m-%d"), "2021-01-28");
    }
}
