use std::fs;

pub struct Cubes {
    pub red: Option<i64>,
    pub green: Option<i64>,
    pub blue: Option<i64>,
}

pub enum CubesField {
    Red,
    Green,
    Blue,
}

impl Cubes {
    fn get_property(&self, name: &str) -> Option<i64> {
        match name {
            "red" => self.red,
            "green" => self.green,
            "blue" => self.blue,
            _ => panic!("property not defined"),
        }
    }
    fn set_property(&mut self, name: &str, value: Option<i64>) {
        match name {
            "red" => self.red = value,
            "green" => self.green = value,
            "blue" => self.blue = value,
            _ => panic!("property not defined"),
        }
    }

    fn final_value(&self) -> i64 {
        self.blue.unwrap() * self.green.unwrap() * self.red.unwrap()
    }
}
fn main() {
    let cubes = Cubes {
        red: Some(12),
        green: Some(13),
        blue: Some(14),
    };
    static FILE_PATH: &str = "src/testfile.txt";
    let total = fs::read_to_string(FILE_PATH)
        .expect("file not found")
        .split("\n")
        .into_iter()
        .map(|x| count_line(x, &cubes))
        .sum::<i64>();
    println!("{}", total);
    let total_part_2 = fs::read_to_string(FILE_PATH)
        .expect("file not found")
        .split("\n")
        .into_iter()
        .map(|x| count_line_part_2(x))
        .sum::<i64>();
    println!("{}", total_part_2);
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

    let games = data
        .last()
        .unwrap()
        .split(";")
        .collect::<Vec<&str>>()
        .join(";");
    if bigger_than_cubes(games.as_str(), cubes).is_err() {
        return 0;
    }
    index
}

fn count_line_part_2(line: &str) -> i64 {
    let data = line.split(":").collect::<Vec<&str>>();
    let games = data
        .last()
        .unwrap()
        .split(";")
        .collect::<Vec<&str>>()
        .join(";");
    minimun_cubes(games.as_str())
}

fn bigger_than_cubes(game: &str, cubes: &Cubes) -> Result<(), ()> {
    let rounds = game.split(";").collect::<Vec<&str>>();
    let mut res = Ok(());
    'round_loop: for round in rounds {
        let colors = round.split(",").collect::<Vec<&str>>();
        for color in colors {
            let parsed = color.trim().split(" ").collect::<Vec<&str>>();
            let color_value = Some(parsed.first().unwrap().parse::<i64>().unwrap());
            let final_color = parsed.last().unwrap();
            if color_value.gt(&cubes.get_property(final_color)) {
                res = Err(());
                break 'round_loop;
            }
        }
    }
    return res;
}

fn minimun_cubes(game: &str) -> i64 {
    let rounds = game.split(";").collect::<Vec<&str>>();
    let mut starting_cube = Cubes {
        red: Some(0),
        green: Some(0),
        blue: Some(0),
    };
    for round in rounds {
        let colors = round.split(",").collect::<Vec<&str>>();
        for color in colors {
            let parsed = color.trim().split(" ").collect::<Vec<&str>>();
            let color_value = Some(parsed.first().unwrap().parse::<i64>().unwrap());
            let final_color = parsed.last().unwrap();
            if color_value.gt(&starting_cube.get_property(final_color)) {
                starting_cube.set_property(final_color, color_value);
            }
        }
    }
    return starting_cube.final_value();
}
