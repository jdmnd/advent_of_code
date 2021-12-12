use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

enum ParseResult {
    Fine,
    SyntaxError(char),
    Incomplete(Vec<char>),
}

fn main() -> Result<()> {
    let input = read_input()?;
    part_1(&input);
    part_2(&input);

    Ok(())
}

fn part_1(input: &[Line]) {
    let total: u64 = input
        .iter()
        .map(|line| {
            if let ParseResult::SyntaxError(ch) = parse_line(line) {
                return match ch {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!("didn't expect character {}", ch),
                };
            }
            0
        })
        .sum();
    println!("total = {}", total);
}

fn part_2(input: &[Line]) {
    let mut scores: Vec<(String, i64)> = input
        .iter()
        .filter_map(|line| match parse_line(line) {
            ParseResult::Incomplete(completion) => Some(completion),
            _ => None,
        })
        .map(|completion| {
            let score = completion
                .iter()
                .map(|ch| match ch {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("unexpected completion char {}", ch),
                })
                .fold(0, |acc, score| acc * 5 + score);
            (completion.iter().collect::<String>(), score)
        })
        .collect();
    scores.sort_by_key(|&(_, score)| score);
    println!("scores = {:?}", scores);
    let middle_score = &scores[scores.len() / 2];
    println!("middle score = {:?}", middle_score);
}

fn parse_line(line: &[char]) -> ParseResult {
    let mut stack = vec![];
    for &ch in line {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ch => {
                if let Some(expected) = stack.pop() {
                    if ch != expected {
                        return ParseResult::SyntaxError(ch);
                    }
                }
            }
        }
    }
    if stack.is_empty() {
        ParseResult::Fine
    } else {
        stack.reverse();
        ParseResult::Incomplete(stack)
    }
}

type Line = Vec<char>;

fn read_input() -> Result<Vec<Line>> {
    let mut lines = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        lines.push(line.trim().chars().collect());
    }
    Ok(lines)
}
