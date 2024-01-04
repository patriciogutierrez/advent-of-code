use std::{error::Error, fs};

pub struct Cubes {
    pub red: Option<i64>,
    pub green: Option<i64>,
    pub blue: Option<i64>,
}
fn main() {
    let cubes = Cubes {
        red: Some(14),
        green: Some(2),
        blue: Some(2),
    };
    static FILE_PATH: &str = "src/testfile.txt";
    let total = fs::read_to_string(FILE_PATH)
        .expect("file not found")
        .split("\n")
        .into_iter()
        .map(|x| count_line(x, &cubes))
        .sum::<i64>();
    println!("{}", total);
}

fn count_line(line: &str, cubes: &Cubes) -> i64 {
    let data = line.split(":").collect::<Vec<&str>>();
    let index = data
        .first()
        .unwrap()
        .matches(char::is_numeric)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap()
        .to_owned();

    let games = data.last().unwrap().split(";").collect::<Vec<&str>>();

    let parsed_games = games
        .into_iter()
        .filter_map(|game| bigger_than_cubes(game, cubes).ok())
        .collect::<Vec<()>>();
    println!("{:?}", parsed_games);
    if parsed_games.len() == 0 {
        return 0;
    }
    index
}

fn bigger_than_cubes(game: &str, cubes: &Cubes) -> Result<(), ()> {
    println!("{}", cubes.green.unwrap());
    if cubes.green.unwrap() > 1 {
        println!("error");
        return Err(());
    }
    println!("no hay error");
    Ok(())
}
