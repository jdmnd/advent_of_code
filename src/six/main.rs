use std::collections::HashMap;
use std::io;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let fishies = read_input()?;
    part_1(&fishies);
    part_2(&fishies);

    Ok(())
}

fn part_1(fishies: &[u8]) {
    println!("part 1");
    let mut fishies: Vec<_> = fishies.into_iter().map(|&f| f).collect();
    for _ in 0..80 {
        fishies = fishies.iter().fold(vec![], |mut acc, &fishie| {
            let (new_age, spawn) = update_age(fishie);
            acc.push(new_age);
            if let Some(new_fishie) = spawn {
                acc.push(new_fishie);
            }
            acc
        })
    }
    println!("{} fishies", fishies.len());
}

fn part_2(fishies: &[u8]) {
    println!("part 2");
    let mut age_to_count: HashMap<u8, u64> = HashMap::new();
    for &fishie in fishies {
        *age_to_count.entry(fishie).or_insert(0) += 1;
    }
    for _i in 0..256 {
        let &count_spawn = age_to_count.get(&0).unwrap_or(&0);
        age_to_count = age_to_count
            .into_iter()
            .filter_map(|(age, count)| (age != 0).then(|| (age - 1, count)))
            .collect();
        *age_to_count.entry(6).or_insert(0) += count_spawn;
        age_to_count.insert(8, count_spawn);
    }
    let total: u64 = age_to_count.iter().map(|(_, &count)| count as u64).sum();
    println!("{} fishies", total);
}

fn update_age(initial_age: u8) -> (u8, Option<u8>) {
    if initial_age == 0 {
        (6, Some(8))
    } else {
        (initial_age - 1, None)
    }
}

fn read_input() -> Result<Vec<u8>> {
    let stdin = io::stdin();
    let mut first_line = String::new();
    stdin.read_line(&mut first_line)?;

    let nums = first_line
        .trim()
        .split(",")
        .map(|s| s.parse())
        .collect::<std::result::Result<_, _>>()?;

    Ok(nums)
}
