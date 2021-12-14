use std::collections::BTreeMap;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type State = BTreeMap<(char, char), u128>;
type Substitutions = BTreeMap<(char, char), char>;
type CharacterOccurrences = BTreeMap<char, u128>;

/// To run the program for 40 iterations with input from `input.txt`, run
///   cargo run --bin day_14 < input.txt 40
fn main() -> Result<()> {
    let input = read_input()?;
    //println!("Input = {:?}", &input);
    let (substitutions, template) = input;

    let num_rounds: u32 = std::env::args()
        .skip(1)
        .next()
        .map(|s| s.parse())
        .unwrap_or(Ok(10))?;

    let mut state = build_initial_state(&template);

    for _ in 1..=num_rounds {
        state = polymerize(&substitutions, state);
    }

    let counts = count_occurrences(&state);

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

fn polymerize(substitutions: &Substitutions, state: State) -> State {
    let mut new_state: State = BTreeMap::new();
    for ((a, b), count) in state {
        if let Some(&c) = substitutions.get(&(a, b)) {
            *new_state.entry((a, c)).or_insert(0) += count;
            *new_state.entry((c, b)).or_insert(0) += count;
        } else {
            *new_state.entry((a, b)).or_insert(0) += count;
        }
    }
    new_state
}

fn build_initial_state(template: &str) -> State {
    let mut state = BTreeMap::new();
    format!("^{}$", template)
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .for_each(|window| {
            *state.entry((window[0], window[1])).or_insert(0) += 1;
        });
    state
}

fn count_occurrences(state: &State) -> CharacterOccurrences {
    let mut counts = BTreeMap::new();
    for (&(a, b), count) in state {
        if a != '^' {
            *counts.entry(a).or_insert(0) += count;
        }
        if b != '$' {
            *counts.entry(b).or_insert(0) += count;
        }
    }
    counts
        .into_iter()
        .map(|(ch, count)| (ch, count / 2))
        .collect()
}

fn read_input() -> Result<(Substitutions, String)> {
    let mut template = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut template)?;

    let mut mappings: Substitutions = BTreeMap::new();
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
