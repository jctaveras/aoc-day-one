use fancy_regex::{Captures, Regex};
use std::fs;

const NUMBERS: [(&'static str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let content = fs::read_to_string("./input.txt").expect("File should exist");
    let result: i32 = content
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(extract_calibration_value)
        .sum();

    println!("Sum of all the calibration values: {:?}", result);
}

fn extract_calibration_value(input_code: &&str) -> i32 {
    let digit_regex = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();
    let mut capture_iterator = digit_regex.captures_iter(input_code);
    let first = get_number(capture_iterator.next());
    let last = get_number(capture_iterator.last());

    match first {
        Some(first) => match last {
            Some(last) => (first * 10) + last,
            None => first * 11,
        },
        None => panic!("No digit found"),
    }
}

fn get_number(value: Option<Result<Captures<'_>, fancy_regex::Error>>) -> Option<i32> {
    match value {
        Some(value) => match value {
            Ok(value) => match value.get(1) {
                Some(value) => match value.as_str().parse() {
                    Ok(number) => Some(number),
                    Err(_) => match NUMBERS
                        .iter()
                        .filter(|number| number.0 == value.as_str())
                        .next()
                    {
                        Some(number) => Some(number.1),
                        None => None,
                    },
                },
                None => None,
            },
            Err(_) => None,
        },
        None => None,
    }
}
