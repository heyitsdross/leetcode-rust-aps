use std::ops::Range;
use crate::p2::add_two_numbers::Solution;

pub(crate) struct PalindromeNumber {}

impl PalindromeNumber {

    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
           return false
        }

        let digits: Vec<u32> = x.to_string()
            .chars()
            .filter(|d| d.is_numeric())
            .map(|d| d.to_digit(10).unwrap())
            .collect();

        if (digits.len() == 1) {
            return true;
        }

        let half_array_index = digits.len() / 2 - 1;

        (0..=half_array_index)
            .map(|i| (digits[i], digits[digits.len() - i]) )
            .all(|t| t.0 == t.1)
    }
}