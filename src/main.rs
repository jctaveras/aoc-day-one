use regex::{Captures, Regex};
use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").expect("File should exist");
    let result: i32 = content
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(extract_calibration_value)
        .sum();

    println!("Sum of all the calibration values: {result}");
}

fn extract_calibration_value(input_code: &&str) -> i32 {
    let digit_regex = Regex::new(r"(?<digits>\d)").unwrap();
    let digits = digit_regex
        .captures_iter(input_code)
        .map(digits_from_capture)
        .collect::<Vec<i32>>();

    (digits[0] * 10) + digits[digits.len() - 1]
}

fn digits_from_capture(value: Captures<'_>) -> i32 {
    match value.name("digits").unwrap().as_str().parse() {
        Ok(number) => number,
        Err(err) => panic!("Input must be a number {}", err),
    }
}
