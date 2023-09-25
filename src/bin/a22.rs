// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
pub mod test {
    use crate::*;

    /// clamp tests

    #[test]
    fn test_clamp_lower() {
        let result = clamp(0, 1, 5);
        let expected = 1;
        assert_eq!(result, expected, "result is not below lower: {}", result)
    }

    #[test]
    fn test_clamp_upper() {
        let result = clamp(6, 1, 5);
        let expected = 5;
        assert_eq!(result, expected, "result is not above upper: {}", result)
    }

    #[test]
    fn test_clamp_in_between() {
        let result = clamp(3, 1, 5);
        let expected = 3;
        assert_eq!(result, expected, "result is not in between: {}", result)
    }

    #[test]
    fn test_div_some_to_f64() {
        let result = div(0, 1);
        let expected = Some(0);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_div_f64_() {
        let result = div(1, 0);
        let expected = None;
        assert_eq!(result, expected)
    }

    #[test]
    fn test_div_some() {
        let result = div(1, 1);
        let expected = Some(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_concat_strings() {
        let result = concat("first string ", "second string");
        let expected = "first string second string";
        assert_eq!(result, expected)
    }
}
