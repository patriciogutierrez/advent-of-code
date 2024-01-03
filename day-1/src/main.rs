use std::fs;

fn main() {

    static FILE_PATH: &str = "src/testfile.txt";
    let contents: i64 = fs::read_to_string(FILE_PATH)
        .expect("file not found")
        .split("\n")
        .into_iter()
        .map(sum_numbers)
        .sum();
    println!("{}", contents);
}

fn sum_numbers(line: &str) -> i64 {
    let last = line
        .chars()
        .rfind(|character| match character.to_string().parse::<i64>() {
            Ok(_) => true,
            _ => false,
        })
        .unwrap()
        .to_string()
        .parse::<i64>()
        .unwrap();
    let first = line
        .chars()
        .find(|character| match character.to_string().parse::<i64>() {
            Ok(_) => true,
            _ => false,
        })
        .unwrap()
        .to_string()
        .parse::<i64>()
        .unwrap();
    first * 10 + last
}
