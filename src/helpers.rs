use std::{cmp::Ordering, str::FromStr};

/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub fn split_to_ints(input: &str, delim: &str) -> Option<(u32, u32)> {
    let (first_str, second_str) = input.trim().split_once(delim)?;

    let first = u32::from_str(first_str).ok()?;
    let second = u32::from_str(second_str).ok()?;

    Some((first, second))
}

pub fn normalize(n: i32) -> i32 {
    match n.cmp(&0) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[test]
fn test_parse_input() {
    // Test valid input
    let input = "4-5";
    let result = split_to_ints(input, "-");
    assert_eq!(result, Some((4, 5)));

    // Test input with leading and trailing whitespace
    let input = " 4-5 ";
    let result = split_to_ints(input, "-");
    assert_eq!(result, Some((4, 5)));

    // Test input with only one number
    let input = "4-";
    let result = split_to_ints(input, "-");
    assert_eq!(result, None);

    // Test input with non-numeric characters
    let input = "4-5a";
    let result = split_to_ints(input, "-");
    assert_eq!(result, None);

    // Test input with negative numbers
    let input = "-4--5";
    let result = split_to_ints(input, "-");
    assert_eq!(result, None);
}

#[test]
fn test_normalize() {
    assert_eq!(normalize(-5), -1);
    assert_eq!(normalize(-1), -1);
    assert_eq!(normalize(0), 0);
    assert_eq!(normalize(1), 1);
    assert_eq!(normalize(5), 1);
}
