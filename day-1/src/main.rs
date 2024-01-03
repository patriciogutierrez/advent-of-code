use regex::Regex;
use std::fs;

fn main() {
    let regex_digits: Regex = Regex::new(r"(\d+)(.*\d)?").unwrap();

    static FILE_PATH: &str = "FILE";
    let contents: i64 = fs::read_to_string(FILE_PATH)
        .expect("file not found")
        .split("\n")
        .into_iter()
        .map(|x| sum_numbers(&regex_digits, x))
        .sum();
    println!("{}", contents);
}

fn sum_numbers(regex: &Regex, line: &str) -> i64 {
    let iterate_digits = regex.captures(line).unwrap();
    (iterate_digits[0].chars().nth(0).unwrap().to_string()
        + &iterate_digits[0].chars().last().unwrap().to_string())
        .parse()
        .unwrap()
}
