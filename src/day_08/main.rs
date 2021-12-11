use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let digits_to_segments: BTreeMap<u8, BTreeSet<char>> = BTreeMap::from([
        (0, BTreeSet::from(['a', 'b', 'c', 'e', 'f', 'g'])),
        (1, BTreeSet::from(['c', 'f'])),
        (2, BTreeSet::from(['a', 'c', 'd', 'e', 'g'])),
        (3, BTreeSet::from(['a', 'c', 'd', 'f', 'g'])),
        (4, BTreeSet::from(['b', 'c', 'd', 'f'])),
        (5, BTreeSet::from(['a', 'b', 'd', 'f', 'g'])),
        (6, BTreeSet::from(['a', 'b', 'd', 'e', 'f', 'g'])),
        (7, BTreeSet::from(['a', 'c', 'f'])),
        (8, BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g'])),
        (9, BTreeSet::from(['a', 'b', 'c', 'd', 'f', 'g'])),
    ]);

    let segments_to_digits: BTreeMap<BTreeSet<char>, u8> = digits_to_segments
        .iter()
        .map(|(digit, segments)| (segments.clone(), *digit))
        .collect();

    let all_entries = read_input()?;
    let total: i64 = all_entries
        .iter()
        .map(|entry| solve_entry(&entry, &digits_to_segments, &segments_to_digits))
        .sum();
    println!("total = {}", total);

    Ok(())
}

/// apply the provided candidate mapping to each of the provided patterns
/// returns the digits that the patterns map to, if any
fn check_mapping(
    patterns: &[String],
    segments_to_digits: &BTreeMap<BTreeSet<char>, u8>,
    candidate_mapping: &BTreeMap<char, char>,
) -> Option<Vec<u8>> {
    patterns
        .iter()
        .map(|pattern| {
            let mapped_pattern = pattern
                .chars()
                .map(|ch| candidate_mapping[&ch])
                .collect::<BTreeSet<char>>();
            segments_to_digits.get(&mapped_pattern).map(|&digit| digit)
        })
        .collect()
}

/// recursively explore a set of known-possible mappings
/// returns a mapping that converts each of the patterns to a valid digit
fn check_all_possible_mappings(
    segments_to_digits: &BTreeMap<BTreeSet<char>, u8>,
    patterns: &[String],
    possible_mappings: &[(char, BTreeSet<char>)],
    candidate_mapping: BTreeMap<char, char>,
) -> Option<BTreeMap<char, char>> {
    if !possible_mappings.is_empty() {
        // recursively explore the possible mappings until we have built a complete
        // candidate mapping
        let (from, to_set) = &possible_mappings[0];
        for to in to_set {
            if candidate_mapping.values().any(|v| v == to) {
                continue;
            }
            let mut the_choices = candidate_mapping.clone();
            the_choices.insert(*from, *to);
            if let Some(mapping) = check_all_possible_mappings(
                segments_to_digits,
                patterns,
                &possible_mappings[1..],
                the_choices,
            ) {
                return Some(mapping);
            }
        }
        return None;
    } else {
        // we have a complete candidate, verify whether it works for all the patterns provided
        assert_eq!(possible_mappings.len(), 0);
        if let Some(_numbers) = check_mapping(patterns, segments_to_digits, &candidate_mapping) {
            return Some(candidate_mapping);
        }
        return None;
    }
}

fn solve_entry(
    entry: &Entry,
    digits_to_segments: &BTreeMap<u8, BTreeSet<char>>,
    segments_to_digits: &BTreeMap<BTreeSet<char>, u8>,
) -> i64 {
    // all possible candidate mappings, where a pattern character is the key, and
    // the set of segments it might map to is the value
    let mut possible_mappings: BTreeMap<char, BTreeSet<char>> = ('a'..='g')
        .map(|from| (from, BTreeSet::from_iter('a'..='g')))
        .collect();
    // remove impossible mappings by inferring digits based on length
    for pattern in &entry.signal_patterns {
        let digits_with_same_length: Vec<_> = digits_to_segments
            .iter()
            .filter(|(_, segments)| segments.len() == pattern.len())
            .map(|(digit, _)| digit)
            .collect();
        let potential_digit_mappings = digits_with_same_length
            .iter()
            .map(|digit| digits_to_segments[digit].clone())
            .reduce(|a: BTreeSet<char>, b| a.union(&b).map(|&c| c).collect())
            .unwrap();
        pattern.chars().for_each(|pattern_character| {
            possible_mappings
                .get_mut(&pattern_character)
                .unwrap()
                .retain(|ch| potential_digit_mappings.contains(ch));
        });
    }
    // for efficiency, we will consider mappings with fewer possibilities first
    let mut possible_mappings: Vec<_> = possible_mappings.into_iter().collect();
    possible_mappings.sort_by_key(|(_, to_set)| to_set.len());
    if let Some(mapping) = check_all_possible_mappings(
        &segments_to_digits,
        &entry.signal_patterns,
        &possible_mappings,
        BTreeMap::new(),
    ) {
        let output = check_mapping(&entry.output_value, &segments_to_digits, &mapping)
            .unwrap()
            .into_iter()
            .fold(0, |acc, d| acc * 10 + (d as i64));

        println!("mapped output = {:?}", output);
        return output;
    }
    panic!("didn't manage to solve for entry {:?}", &entry);
}

#[derive(Clone, Debug)]
struct Entry {
    signal_patterns: Vec<String>,
    output_value: Vec<String>,
}

fn read_input() -> Result<Vec<Entry>> {
    let mut entries = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let (first_part, second_part) = line
            .split_once(" | ")
            .ok_or(format!("No separator in \"{}\"", &line))?;
        entries.push(Entry {
            signal_patterns: first_part
                .split_ascii_whitespace()
                .map(String::from)
                .collect(),
            output_value: second_part
                .split_ascii_whitespace()
                .map(String::from)
                .collect(),
        });
    }
    Ok(entries)
}
