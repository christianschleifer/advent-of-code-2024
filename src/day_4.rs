use std::fs::File;
use std::io::BufRead;

const LEN_XMAS: usize = "XMAS".len();

#[derive(Clone)]
struct Input {
    lines: Vec<String>,
}

pub fn solve() {
    let input = get_input();

    solve_puzzle_1(input.clone());
    solve_puzzle_2(input);
}

#[derive(Copy, Clone)]
struct Position {
    row: usize,
    column: usize,
}

struct Matrix {
    raw_matrix: Vec<Vec<char>>,
}

enum Direction {
    None,
    Forwards,
    Backwards,
}

impl Matrix {
    fn new(input: Input) -> Matrix {
        let raw_matrix = input
            .lines
            .into_iter()
            .map(|line| line.chars().collect())
            .collect();

        Matrix { raw_matrix }
    }

    fn num_xmas(&self, position: Position) -> u8 {
        let mut num_xmas = 0;
        let char_at_position = self
            .raw_matrix
            .get(position.row)
            .and_then(|row| row.get(position.column))
            .expect("illegal input");
        if 'X' != *char_at_position {
            return num_xmas;
        }

        // search in all directions

        // horizontal back
        if self.is_xmas(position, Direction::None, Direction::Backwards) {
            num_xmas += 1;
        }

        // horizontal forward
        if self.is_xmas(position, Direction::None, Direction::Forwards) {
            num_xmas += 1;
        }

        // vertical back
        if self.is_xmas(position, Direction::Backwards, Direction::None) {
            num_xmas += 1;
        }

        // vertical forward
        if self.is_xmas(position, Direction::Forwards, Direction::None) {
            num_xmas += 1;
        }

        // diagonal 1 back
        if self.is_xmas(position, Direction::Backwards, Direction::Backwards) {
            num_xmas += 1;
        }

        // diagonal 1 forward
        if self.is_xmas(position, Direction::Forwards, Direction::Forwards) {
            num_xmas += 1;
        }

        // diagonal 2 back
        if self.is_xmas(position, Direction::Backwards, Direction::Forwards) {
            num_xmas += 1;
        }

        // diagonal 2 forwards
        if self.is_xmas(position, Direction::Forwards, Direction::Backwards) {
            num_xmas += 1;
        }

        num_xmas
    }

    fn is_xmas(
        &self,
        position: Position,
        row_direction: Direction,
        column_direction: Direction,
    ) -> bool {
        let row_check = match row_direction {
            Direction::Forwards => self.len_rows() - position.row >= LEN_XMAS,
            Direction::Backwards => position.row + 1 >= LEN_XMAS,
            Direction::None => true,
        };

        let column_check = match column_direction {
            Direction::Forwards => self.len_columns() - position.column >= LEN_XMAS,
            Direction::Backwards => position.column + 1 >= LEN_XMAS,
            Direction::None => true,
        };

        if row_check && column_check {
            let row_operation: Box<dyn Fn(usize, usize) -> usize> = match row_direction {
                Direction::None => Box::new(|index, _| index),
                Direction::Forwards => Box::new(|index, num| index + num),
                Direction::Backwards => Box::new(|index, num| index - num),
            };

            let column_operation: Box<dyn Fn(usize, usize) -> usize> = match column_direction {
                Direction::None => Box::new(|index, _| index),
                Direction::Forwards => Box::new(|index, num| index + num),
                Direction::Backwards => Box::new(|index, num| index - num),
            };

            let second = self
                .raw_matrix
                .get(row_operation(position.row, 1))
                .and_then(|row| row.get(column_operation(position.column, 1)))
                .unwrap_or_else(|| unreachable!());
            if 'M' == *second {
                let third = self
                    .raw_matrix
                    .get(row_operation(position.row, 2))
                    .and_then(|row| row.get(column_operation(position.column, 2)))
                    .unwrap_or_else(|| unreachable!());

                if 'A' == *third {
                    let fourth = self
                        .raw_matrix
                        .get(row_operation(position.row, 3))
                        .and_then(|row| row.get(column_operation(position.column, 3)))
                        .unwrap_or_else(|| unreachable!());

                    if 'S' == *fourth {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn is_cross_mas(&self, position: Position) -> bool {
        let char_at_position = self
            .raw_matrix
            .get(position.row)
            .and_then(|row| row.get(position.column))
            .expect("illegal input");
        if 'A' != *char_at_position {
            return false;
        }

        let is_mas_diagonal_1 = self.is_mas(position, Direction::Forwards, Direction::Forwards)
            || self.is_mas(position, Direction::Backwards, Direction::Backwards);
        let is_mas_diagonal_2 = self.is_mas(position, Direction::Backwards, Direction::Forwards)
            || self.is_mas(position, Direction::Forwards, Direction::Backwards);

        is_mas_diagonal_1 && is_mas_diagonal_2
    }

    fn is_mas(
        &self,
        position: Position,
        row_direction: Direction,
        column_direction: Direction,
    ) -> bool {
        let row_check = match row_direction {
            Direction::Forwards => position.row >= 1 && position.row < self.len_rows() - 1,
            Direction::Backwards => position.row >= 1 && position.row < self.len_rows() - 1,
            Direction::None => true,
        };

        let column_check = match column_direction {
            Direction::Forwards => position.column >= 1 && position.column < self.len_columns() - 1,
            Direction::Backwards => {
                position.column >= 1 && position.column < self.len_columns() - 1
            }
            Direction::None => true,
        };

        if row_check && column_check {
            let (row_index_1, row_index_2) = match row_direction {
                Direction::Forwards => (position.row + 1, position.row - 1),
                Direction::Backwards => (position.row - 1, position.row + 1),
                Direction::None => unreachable!(),
            };

            let (column_index_1, column_index_2) = match column_direction {
                Direction::Forwards => (position.column + 1, position.column - 1),
                Direction::Backwards => (position.column - 1, position.column + 1),
                Direction::None => unreachable!(),
            };

            let first = self
                .raw_matrix
                .get(row_index_1)
                .and_then(|row| row.get(column_index_1))
                .unwrap_or_else(|| unreachable!());
            if 'M' == *first {
                let third = self
                    .raw_matrix
                    .get(row_index_2)
                    .and_then(|row| row.get(column_index_2))
                    .unwrap_or_else(|| unreachable!());

                if 'S' == *third {
                    return true;
                }
            }
        }

        false
    }

    fn len_rows(&self) -> usize {
        self.raw_matrix.len()
    }

    fn len_columns(&self) -> usize {
        self.raw_matrix
            .first()
            .expect("empty matrix not supported")
            .len()
    }
}

fn solve_puzzle_1(input: Input) {
    let matrix = Matrix::new(input);

    let mut num_xmas: u32 = 0;

    for row_i in 0..matrix.len_rows() {
        for column_i in 0..matrix.len_columns() {
            num_xmas += matrix.num_xmas(Position {
                row: row_i,
                column: column_i,
            }) as u32;
        }
    }
    println!("Puzzle 1: Number of 'XMAS' found: {}", num_xmas);
}

fn solve_puzzle_2(input: Input) {
    let matrix = Matrix::new(input);

    let mut num_cross_mas = 0;

    for row_i in 0..matrix.len_rows() {
        for column_i in 0..matrix.len_columns() {
            if matrix.is_cross_mas(Position {
                row: row_i,
                column: column_i,
            }) {
                num_cross_mas += 1;
            }
        }
    }
    println!("Puzzle 2: Number of cross 'MAS' found: {}", num_cross_mas);
}

fn get_input() -> Input {
    let file = File::open("./src/resources/day_4.txt").expect("could not open file for day 4");
    let buf_reader = std::io::BufReader::new(file);

    let lines = buf_reader
        .lines()
        .map(|line| line.expect("could not read line"))
        .collect();

    Input { lines }
}
