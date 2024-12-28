use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let input = get_input();
    solve_puzzle_1(input.clone());
    solve_puzzle_2(input);
}

fn solve_puzzle_1(input: Input) {
    let mut total_calibration_result = 0;

    let mut allowed_operators = BTreeSet::new();
    allowed_operators.insert(Operator::Add);
    allowed_operators.insert(Operator::Multiply);

    for operation in input.operations {
        if operation.is_valid(&allowed_operators) {
            total_calibration_result += operation.result;
        }
    }

    println!(
        "Puzzle 1: Total calibration result: {}",
        total_calibration_result
    );
}

fn solve_puzzle_2(input: Input) {
    let mut total_calibration_result = 0;

    let mut allowed_operators = BTreeSet::new();
    allowed_operators.insert(Operator::Add);
    allowed_operators.insert(Operator::Multiply);
    allowed_operators.insert(Operator::Concatenate);

    for operation in input.operations {
        if operation.is_valid(&allowed_operators) {
            total_calibration_result += operation.result;
        }
    }

    println!(
        "Puzzle 2: Total calibration result: {}",
        total_calibration_result
    );
}

#[derive(Debug, Clone)]
struct Operation {
    result: u64,
    operands: Vec<u64>,
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operation {
    fn is_valid(&self, allowed_operators: &BTreeSet<Operator>) -> bool {
        assert!(!self.operands.is_empty());
        let permutation = Vec::new();
        let mut permutations = Vec::new();

        Self::create_permutations(
            allowed_operators,
            &self.operands,
            permutation,
            &mut permutations,
        );

        for permutation in &permutations {
            let mut iter = self.operands.iter();
            let mut result = iter.next().copied().unwrap();
            for operator in permutation {
                let right_operand = iter.next().unwrap();
                match operator {
                    Operator::Add => result += right_operand,
                    Operator::Multiply => result *= right_operand,
                    Operator::Concatenate => {
                        let result_as_string = result.to_string();
                        let right_operand_as_string = right_operand.to_string();

                        let concatenated = result_as_string + &right_operand_as_string;

                        result = concatenated.parse().unwrap()
                    }
                }
            }

            if result == self.result {
                return true;
            }
        }

        false
    }

    fn create_permutations(
        operators: &BTreeSet<Operator>,
        operands: &[u64],
        permutation: Vec<Operator>,
        permutations: &mut Vec<Vec<Operator>>,
    ) {
        if operands.len() <= 1 {
            permutations.push(permutation);
        } else {
            for operator in operators {
                let mut cloned = permutation.clone();
                cloned.push(*operator);
                Self::create_permutations(operators, &operands[1..], cloned, permutations);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    operations: Vec<Operation>,
}

fn get_input() -> Input {
    let file = File::open("./src/resources/day_7.txt").expect("could not open file");
    let buf_reader = BufReader::new(file);

    let mut operations = Vec::new();

    for line in buf_reader.lines() {
        let line = line.expect("could not read line");

        let mut parts = line.split(':');

        let result = parts.next().unwrap().parse::<u64>().unwrap();

        let operands: Vec<u64> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        operations.push(Operation { result, operands })
    }

    Input { operations }
}
