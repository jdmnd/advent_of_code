use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;
    println!("Input = {:?}", &input);
    part_one(&input);
    part_two(&input);

    Ok(())
}

fn part_one(input: &[u32]) {
    let all_but_first = {
        let mut it = input.iter();
        it.next();
        it
    };
    let num_increasing = input
        .iter()
        .zip(all_but_first)
        .filter(|(left, right)| left < right)
        .count();
    println!("num increasing = {:?}", num_increasing);
}

fn part_two(input: &[u32]) {
    let all_but_first = {
        let mut it = input.iter();
        it.next();
        it
    };
    let all_but_second = {
        let mut it = all_but_first.clone();
        it.next();
        it
    };
    let windows = input
        .iter()
        .zip(all_but_first)
        .zip(all_but_second)
        .map(|((a, b), c)| a + b + c)
        .collect::<Vec<_>>();
    println!("windows {:?}", windows);
    part_one(&windows);
}

fn read_input() -> Result<Vec<u32>> {
    let mut nums = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        nums.push(line.parse()?);
    }
    Ok(nums)
}
