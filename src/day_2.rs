use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct Input {
    rows: Vec<Vec<u64>>,
}

pub fn solve() {
    let input = get_input();
    solve_puzzle_1(input.clone());
    solve_puzzle_2(input);
}

fn solve_puzzle_1(input: Input) {
    let mut num_safe_reports = 0;

    for row in input.rows {
        if is_safe(row.clone()) {
            num_safe_reports += 1;
        }
    }

    println!("Puzzle 1: Number of safe reports: {}", num_safe_reports);
}

fn solve_puzzle_2(input: Input) {
    let mut num_safe_reports = 0;

    for row in input.rows {
        if is_safe(row.clone()) {
            num_safe_reports += 1;
        } else {
            for i in 0..row.len() {
                let mut modified_row = row.clone();
                modified_row.remove(i);
                if is_safe(modified_row) {
                    num_safe_reports += 1;
                    break;
                }
            }
        }
    }

    println!("Puzzle 2: Number of safe reports: {}", num_safe_reports);
}

enum Order {
    Unknown,
    Asc,
    Desc,
}

fn is_safe(row: Vec<u64>) -> bool {
    let mut iter = row.into_iter();
    let mut order = Order::Unknown;
    if let Some(mut prev_level) = iter.next() {
        for level in iter {
            let diff = level as i64 - prev_level as i64;

            if !(1..=3).contains(&diff.abs()) {
                return false;
            }

            match order {
                Order::Unknown => {
                    if diff < 0 {
                        order = Order::Desc;
                    } else {
                        order = Order::Asc
                    }
                }
                Order::Asc => {
                    if diff < 0 {
                        return false;
                    }
                }
                Order::Desc => {
                    if diff > 0 {
                        return false;
                    }
                }
            }

            prev_level = level
        }

        true
    } else {
        true
    }
}

fn get_input() -> Input {
    let input = File::open("./src/resources/day_2.txt").expect("input file for day 2 not found");
    let reader = BufReader::new(input);

    let mut rows = Vec::new();

    for line in reader.lines() {
        let line = line.expect("could not read next line");

        let parts = line.split_whitespace();

        let row: Vec<_> = parts
            .map(|num| num.parse::<u64>().expect("could not parse to u64"))
            .collect();

        rows.push(row);
    }

    Input { rows }
}
