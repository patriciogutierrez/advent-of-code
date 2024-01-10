use std::fs;

fn main() {
    static FILE_PATH: &str = "src/testfile.txt";
    let mut res = fs::read_to_string(FILE_PATH)
        .expect("file not found")
        .split("\n")
        .into_iter()
        .map(|n| sum_numbers(n.to_owned()))
        .sum::<i64>();
    println!("{}", res);
    res = fs::read_to_string(FILE_PATH)
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
    println!("{}", res);
}

fn sum_numbers(line: String) -> i64 {
    let numbers: Vec<i64> = line.matches(char::is_numeric).map(|x|x.parse::<i64>().unwrap()).collect();
    numbers.first().unwrap() * 10 + numbers.last().unwrap()

}
