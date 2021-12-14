use std::collections::BTreeMap;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// To run the program for 40 iterations with input from `input.txt`, run
///   cargo run --bin day_14 < input.txt 40
fn main() -> Result<()> {
    let input = read_input()?;
    println!("Input = {:?}", &input);
    let (substitution_map, template) = input;

    let num_rounds: u32 = std::env::args()
        .skip(1)
        .next()
        .map(|s| s.parse())
        .unwrap_or(Ok(10))?;

    let mut state = BTreeMap::new();
    format!("^{}$", template)
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .for_each(|window| {
            *state.entry((window[0], window[1])).or_insert(0) += 1;
        });
    for _ in 1..=num_rounds {
        state = polymerize(&substitution_map, state);
    }

    let mut counts: BTreeMap<char, u64> = BTreeMap::new();
    for ((a, b), count) in state {
        if a != '^' {
            *counts.entry(a).or_insert(0) += count;
        }
        if b != '$' {
            *counts.entry(b).or_insert(0) += count;
        }
    }
    let counts: BTreeMap<char, u64> = counts
        .into_iter()
        .map(|(ch, count)| (ch, count / 2))
        .collect();
    let (max_char, max_count) = counts.iter().max_by_key(|&(_, count)| count).unwrap();
    let (min_char, min_count) = counts.iter().min_by_key(|&(_, count)| count).unwrap();
    println!(
        "max = ({}, {}), min = ({}, {}), max - min = {}",
        max_char,
        max_count,
        min_char,
        min_count,
        max_count - min_count
    );

    Ok(())
}

type State = BTreeMap<(char, char), u64>;
fn polymerize(substitution_map: &BTreeMap<(char, char), char>, state: State) -> State {
    let mut new_state: State = BTreeMap::new();
    for ((a, b), count) in state {
        if let Some(&c) = substitution_map.get(&(a, b)) {
            *new_state.entry((a, c)).or_insert(0) += count;
            *new_state.entry((c, b)).or_insert(0) += count;
        } else {
            *new_state.entry((a, b)).or_insert(0) += count;
        }
    }
    new_state
}

fn read_input() -> Result<(BTreeMap<(char, char), char>, String)> {
    let mut template = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut template)?;

    let mut mappings: BTreeMap<(char, char), char> = BTreeMap::new();
    for line in stdin.lock().lines() {
        let line = line?;
        if let Some((pattern, ch)) = line.split_once(" -> ") {
            let mut pattern = pattern.chars();
            let (a, b, c) = (|| Some((pattern.next()?, pattern.next()?, ch.chars().next()?)))()
                .ok_or(format!("invalid line \"{}\"", line))?;
            mappings.insert((a, b), c);
        }
    }
    Ok((mappings, template.trim().into()))
}
