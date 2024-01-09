use std::{
    collections::HashMap,
    env, fs,
    vec,
};

fn main() {
    let file = fs::read_to_string(env::var("FILE").unwrap()).expect("file not found");
    let games = file
        .split("\r\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.split(":").last().unwrap().trim())
        .collect::<Vec<&str>>();
    let mut parsed_cards: Vec<(Vec<_>, Vec<_>)> = vec![];
    games.into_iter().for_each(|x| {
        let decks = x.split("|").collect::<Vec<&str>>();
        let scratchcard_numbers = decks[0]
            .trim()
            .replace(char::is_whitespace, ",")
            .split(",")
            .map(|x| x.parse::<i64>().unwrap_or_default())
            .filter(|value| value != &0)
            .collect::<Vec<i64>>();
        let hand = decks[1]
            .trim()
            .replace(char::is_whitespace, ",")
            .split(",")
            .map(|x| x.parse::<i64>().unwrap_or_default())
            .filter(|value| value != &0)
            .collect::<Vec<i64>>();
        parsed_cards.push((scratchcard_numbers, hand));
    });
    let mut resultado_final: Vec<i64> = vec![];
    let mut mapeo: HashMap<usize, i64> = HashMap::new();
    for indices in 1..=parsed_cards.len() {
        mapeo.insert(indices, 1);
    }
    let mut index = 0;
    while parsed_cards.get(index).is_some() {
        let mut parsed_card = parsed_cards[index].clone();
        parsed_card.0.retain(|f| parsed_card.1.contains(f));
        println!("index: {}, matches: {:?} ", index, parsed_card.0.len());
        if parsed_card.0.len() > 0 {
            resultado_final.push(
                (if parsed_card.0.len() > 2 {
                    (2_i64).pow((parsed_card.0.len() - 1).try_into().unwrap())
                } else {
                    parsed_card.0.len().try_into().unwrap()
                })
                .try_into()
                .unwrap(),
            );
        }
        index += 1;

        let mut old_value: i64;
        for i in 1..=parsed_card.0.len() {
            let indexar = index + i;
            old_value = mapeo[&index];
            if let Some(x) = mapeo.get_mut(&indexar) {
                *x += old_value
            } else {
                mapeo.insert(indexar, 1);
            }
        }
    }
}
