use std::fs;

fn main() {
    static FILE_PATH: &str = "PATH";
    fs::read_to_string(FILE_PATH)
        .expect("file not found")
        .split("\n")
        .into_iter()
        .map(|n| sum_numbers(n.to_owned()))
        .sum::<i64>();
    fs::read_to_string(FILE_PATH)
        .expect("file not found")
        .split("\n")
        .into_iter()
        .map(|x| {
            x.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
        })
        .map(sum_numbers)
        .sum::<i64>();
}

fn sum_numbers(line: String) -> i64 {
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
