use std::fs::{read_to_string};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let mut total:u32 = 0;

    for line in read_to_string(path).unwrap().lines() {
        let digits = extract_digits_from_line(line);

        let (first, last) = get_first_and_last_digit(digits);

        let num = combine_numbers(&first, &last);
        total += num;
    }
    println!("{}", total);
}

fn get_first_and_last_digit(digits: Vec<u32>) -> (u32, u32) {
    println!("{:?}", digits);
    let first = *digits.first().expect("Couldn't find the first digit");
    let last = *digits.last().unwrap_or(&first);

    (first, last)
}

fn extract_digits_from_line(line: &str) -> Vec<u32> {
    line
        .chars()
        .filter_map(|a| a.to_digit(10))
        .collect()
}

fn combine_numbers(first_digit: &u32, last_digit: &u32) -> u32 {
    let line_num = format!("{}{}", first_digit, last_digit);
    let num: u32 = line_num.parse().unwrap();
    num
}

#[test]
fn it_correctly_gets_first_and_last_digit() {
    let line = vec![1, 5];
    let (first, last) = get_first_and_last_digit(line);
    assert_eq!(first, 1);
    assert_eq!(last, 5);
}

#[test]
fn it_gets_first_digit_when_last_digit_is_missing() {
    let line = vec![1];
    let (first, last) = get_first_and_last_digit(line);
    assert_eq!(first, 1);
    assert_eq!(last, 1);
}

#[test]
fn it_extracts_digits_from_line() {
    let line = "1abc3";
    let digits = extract_digits_from_line(line);
    assert_eq!(digits, vec![1, 3]);
}

#[test]
fn it_gets_digits_from_words_and_numbers() {
    let line = "abc1twonethreedef2";
    let digits = extract_digits_from_line(line);
    assert_eq!(digits, vec![1, 2]);
}