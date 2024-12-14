use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Input {
    first_list: Vec<i64>,
    second_list: Vec<i64>,
}

impl Input {
    fn new(first_list: Vec<i64>, second_list: Vec<i64>) -> Self {
        Self {
            first_list,
            second_list,
        }
    }
}

pub fn solve() {
    let input = get_input();
    solve_puzzle_1(input.clone());
    solve_puzzle_2(input.clone());
}

fn get_input() -> Input {
    let input = File::open("./src/resources/day_1.txt").expect("input file for day 1 not found");
    let reader = BufReader::new(input);

    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in reader.lines() {
        let line = line.expect("could not read next line");
        let mut parts = line.split_whitespace();

        let left = parts.next().expect("could not get the left number");
        let left = left.parse().expect("could not parse input");
        first_list.push(left);

        let right = parts.next().expect("could not get the right number");
        let right = right.parse().expect("could not parse input");
        second_list.push(right)
    }

    Input::new(first_list, second_list)
}

fn solve_puzzle_1(mut input: Input) {
    input.first_list.sort();
    input.second_list.sort();

    let mut total_distance = 0;

    for (first, second) in input.first_list.iter().zip(&input.second_list) {
        let distance = i64::abs(first - second);
        total_distance += distance;
    }

    println!("Total distance between lists: {}", total_distance);
}

fn solve_puzzle_2(input: Input) {
    let mut value_counts = HashMap::new();

    for id in input.second_list {
        let x = value_counts.entry(id).or_insert(0_u64);
        *x += 1;
    }

    let mut similarity_score = 0_u64;

    for id in input.first_list {
        if let Some(count) = value_counts.get(&id) {
            similarity_score += id as u64 * count;
        }
    }

    println!("Similarity score between lists: {}", similarity_score);
}
