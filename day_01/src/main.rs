use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let _query = &args[1];
    let file_path = &args[2];

    println!("Reading Input from {file_path}");

    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();

        left_list.push(split[0].parse().unwrap());
        right_list.push(split[1].parse().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let mut distance = 0;

    for i in 0..left_list.len() {
        distance += i32::abs(left_list[i] - right_list[i]);
    }

    println!("Distance: {distance}");

    let mut simalirity = 0;
    for value in left_list {
        let count = right_list.iter().filter(|&x| *x == value).count() as i32;
        simalirity += count * value;
    }

    println!("Similarity: {simalirity}");
}
