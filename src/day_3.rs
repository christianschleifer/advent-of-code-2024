use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone)]
struct Input {
    lines: Vec<String>,
}

pub fn solve() {
    let input = get_input();

    solve_puzzle_1(input.clone());
    solve_puzzle_2(input)
}

#[derive(PartialEq, Debug)]
enum Token {
    Mul,
    LeftParen,
    RightParen,
    Number(u32),
    Comma,
    Enable,
    Disable,
    Unknown,
}

struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn from(s: &'a str) -> Self {
        Self {
            chars: s.chars().peekable(),
        }
    }

    fn eat(&mut self) -> Option<Token> {
        let char = self.chars.next()?;
        match char {
            '0'..='9' => {
                let mut number = String::from(char);
                while let Some('0'..='9') = self.chars.peek() {
                    let next_digit = self.chars.next().unwrap();
                    number.push(next_digit);
                }
                Some(Token::Number(number.parse().unwrap()))
            }
            'm' => {
                if let Some('u') = self.chars.peek() {
                    self.chars.next();
                    if let Some('l') = self.chars.peek() {
                        self.chars.next();
                        Some(Token::Mul)
                    } else {
                        Some(Token::Unknown)
                    }
                } else {
                    Some(Token::Unknown)
                }
            }
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            ',' => Some(Token::Comma),
            'd' => {
                if let Some('o') = self.chars.peek() {
                    self.chars.next();
                    match self.chars.peek() {
                        Some('n') => {
                            self.chars.next();
                            if let Some('\'') = self.chars.peek() {
                                self.chars.next();
                                if let Some('t') = self.chars.peek() {
                                    self.chars.next();
                                    if let Some('(') = self.chars.peek() {
                                        self.chars.next();
                                        if let Some(')') = self.chars.peek() {
                                            self.chars.next();
                                            Some(Token::Disable)
                                        } else {
                                            Some(Token::LeftParen)
                                        }
                                    } else {
                                        Some(Token::Unknown)
                                    }
                                } else {
                                    Some(Token::Unknown)
                                }
                            } else {
                                Some(Token::Unknown)
                            }
                        }
                        Some('(') => {
                            self.chars.next();
                            if let Some(')') = self.chars.peek() {
                                self.chars.next();
                                Some(Token::Enable)
                            } else {
                                Some(Token::LeftParen)
                            }
                        }
                        _ => Some(Token::Unknown),
                    }
                } else {
                    Some(Token::Unknown)
                }
            }
            _ => Some(Token::Unknown),
        }
    }
}

#[derive(Debug)]
struct MulExpression {
    left_operand: u32,
    right_operand: u32,
}

impl MulExpression {
    fn new(left_operand: u32, right_operand: u32) -> Self {
        Self {
            left_operand,
            right_operand,
        }
    }

    fn evaluate(self) -> u32 {
        self.left_operand * self.right_operand
    }
}

#[derive(Copy, Clone)]
enum State {
    Enabled,
    Disabled,
}

struct Parser<'a> {
    state: State,
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    fn new(tokenizer: Tokenizer<'a>) -> Self {
        Self {
            state: State::Enabled,
            tokenizer,
        }
    }

    fn parse_1(&mut self) -> Vec<MulExpression> {
        let mut expressions = Vec::new();

        while let Some(token) = self.tokenizer.eat() {
            if token == Token::Mul {
                if let Some(Token::LeftParen) = self.tokenizer.eat() {
                    if let Some(Token::Number(left)) = self.tokenizer.eat() {
                        if let Some(Token::Comma) = self.tokenizer.eat() {
                            if let Some(Token::Number(right)) = self.tokenizer.eat() {
                                if let Some(Token::RightParen) = self.tokenizer.eat() {
                                    expressions.push(MulExpression::new(left, right))
                                }
                            }
                        }
                    };
                };
            };

            continue;
        }

        expressions
    }

    fn parse_2(&mut self) -> Vec<MulExpression> {
        let mut expressions = Vec::new();

        while let Some(token) = self.tokenizer.eat() {
            match self.state {
                State::Enabled => match token {
                    Token::Mul => {
                        match self.tokenizer.eat() {
                            Some(Token::LeftParen) => match self.tokenizer.eat() {
                                Some(Token::Number(left)) => match self.tokenizer.eat() {
                                    Some(Token::Comma) => match self.tokenizer.eat() {
                                        Some(Token::Number(right)) => match self.tokenizer.eat() {
                                            Some(Token::RightParen) => {
                                                expressions.push(MulExpression::new(left, right))
                                            }
                                            Some(Token::Disable) => {
                                                self.state = State::Disabled;
                                                continue;
                                            }
                                            _ => continue,
                                        },
                                        Some(Token::Disable) => {
                                            self.state = State::Disabled;
                                            continue;
                                        }
                                        _ => continue,
                                    },
                                    Some(Token::Disable) => {
                                        self.state = State::Disabled;
                                        continue;
                                    }
                                    _ => continue,
                                },
                                Some(Token::Disable) => {
                                    self.state = State::Disabled;
                                    continue;
                                }
                                _ => continue,
                            },
                            Some(Token::Disable) => {
                                self.state = State::Disabled;
                                continue;
                            }
                            _ => continue,
                        };
                    }
                    Token::Disable => {
                        self.state = State::Disabled;
                        continue;
                    }
                    _ => continue,
                },
                State::Disabled => match token {
                    Token::Enable => self.state = State::Enabled,
                    _ => {
                        continue;
                    }
                },
            }

            continue;
        }

        expressions
    }
}

fn solve_puzzle_1(input: Input) {
    let input = input.lines.concat();
    let tokenizer = Tokenizer::from(&input);
    let mut parser = Parser::new(tokenizer);

    let expressions = parser.parse_1();

    let mut total_num = 0;

    for expression in expressions {
        total_num += expression.evaluate();
    }

    println!("Puzzle 1: Result of multiplications: {}", total_num);
}

fn solve_puzzle_2(input: Input) {
    let input = input.lines.concat();
    let tokenizer = Tokenizer::from(&input);
    let mut parser = Parser::new(tokenizer);

    let expressions = parser.parse_2();

    let mut total_num = 0;

    for expression in expressions {
        total_num += expression.evaluate();
    }

    println!("Puzzle 2: Result of enabled multiplications: {}", total_num);
}

fn get_input() -> Input {
    let file = File::open("./src/resources/day_3.txt").expect("could not open file for day 3");
    let buf_reader = BufReader::new(file);

    let lines = buf_reader
        .lines()
        .map(|line| line.expect("could not read line"))
        .collect();

    Input { lines }
}
