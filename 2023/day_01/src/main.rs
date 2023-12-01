use std::fs::{read_to_string};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let mut total:u32 = 0;

    for line in read_to_string(path).unwrap().lines() {
        let digits = extract_digits_from_line(line);

        let mut vec_iterator = digits.iter();
        let first = vec_iterator.nth(0);
        let last = vec_iterator.last();

        let first_digit = match first {
            None => panic!("Couldn't find the first digit"),
            Some(x) => x,
        };

        let last_digit = match last {
            None => first_digit,
            Some(x) => x
        };

        let num = combine_numbers(first_digit, last_digit);
        total += num;
    }
    println!("{}", total);
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