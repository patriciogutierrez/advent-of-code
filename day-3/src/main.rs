use std::{env, fs};

fn main() {
    println!("{}",env::var("FILE").unwrap());
    let asd =fs::read_to_string(env::var("FILE").unwrap()).expect("file not found");
    println!("{}",asd);
}
