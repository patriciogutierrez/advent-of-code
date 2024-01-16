use std::{collections::VecDeque, env, fs};

static INDICES_TO_CHECK: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
fn main() {
    let asd = fs::read_to_string(env::var("FILE").unwrap()).expect("file not found");

    let lines = asd.split("\r\n").collect::<Vec<&str>>();
    let mut matrix = lines
        .into_iter()
        .map(|l| {
            let mut vec = l.split("").collect::<VecDeque<&str>>();
            vec.pop_front();
            vec.pop_back();
            return vec;
        })
        .collect::<Vec<VecDeque<&str>>>();
    let mut total_sum = 0;
    let mut part_two_sum = 0;
    for (matrix_index, line_characters) in matrix.clone().into_iter().enumerate() {
        for (index, character) in line_characters.into_iter().enumerate() {
            let part_two_array: &mut Vec<i64> = &mut vec![];

            if !char::is_numeric(*character.chars().collect::<Vec<char>>().first().unwrap())
                && character.to_owned().ne(".")
            {
                for indices in INDICES_TO_CHECK {
                    let partial_sum = check_number(&mut matrix, (matrix_index, index), indices);
                    total_sum += partial_sum;
                    if matrix[matrix_index][index].eq("*") {
                        part_two_array.push(partial_sum);
                    }
                }
            }
            let available_data = part_two_array
                .clone()
                .into_iter()
                .filter(|x| *x != 0)
                .collect::<Vec<i64>>();

            if available_data.len().eq(&2) {
                part_two_sum += available_data[0] * available_data[1];
            }
        }
    }
    println!("{}", total_sum);
    println!("{}", part_two_sum);
}

fn check_number(
    matrix: &mut Vec<VecDeque<&str>>,
    actual_index: (usize, usize),
    indices: (isize, isize),
) -> i64 {
    let index_i = actual_index.0.checked_add_signed(indices.0).unwrap();
    let index_j = actual_index.1.checked_add_signed(indices.1).unwrap();
    let existance = matrix
        .get(index_i)
        .is_some_and(|x| x.get(index_j).is_some());
    let mut result = 0;
    if existance && matrix[index_i][index_j].contains(char::is_numeric) {
        let found_index = get_whole_number(&matrix[index_i], index_j);
        let temp_str = matrix[index_i]
            .range(found_index.0..found_index.1 + 1)
            .copied()
            .collect::<Vec<&str>>()
            .join("");
        if !temp_str.is_empty() {
            result = temp_str.parse::<i64>().unwrap();
        }

        for value in matrix[index_i].range_mut(found_index.0..found_index.1 + 1) {
            *value = ".";
        }
    }
    result
}

fn get_whole_number(line: &VecDeque<&str>, position: usize) -> (usize, usize) {
    let mut start = position;
    let mut end = position;
    while start > 0 && line.get(start - 1).is_some() && line[start - 1].contains(char::is_numeric) {
        start -= 1;
    }

    while line.get(end + 1).is_some() && line[end + 1].contains(char::is_numeric) {
        end += 1;
    }

    (start, end)
}
