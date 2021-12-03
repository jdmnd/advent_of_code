use std::cmp::Ordering;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;
    part_one(&input);
    part_two(&input);

    Ok(())
}

fn part_one(input: &[Vec<bool>]) {
    println!("part_one");
    let width = input[0].len();
    let most_common = (0..width)
        .map(|bit_n| input.iter().map(|num| num[bit_n]).filter(|&b| b).count() > input.len() / 2)
        .collect::<Vec<_>>();
    let least_common = most_common.iter().map(|b| !b).collect::<Vec<_>>();
    println!(
        "power level = {}",
        bit_vec_to_u32(&most_common) * bit_vec_to_u32(&least_common)
    );
}

fn bit_vec_to_u32(vec: &[bool]) -> u32 {
    assert!(vec.len() <= 32);
    vec.iter().fold(0, |acc, &bit| acc << 1 | bit as u32)
}

fn part_two(input: &[Vec<bool>]) {
    println!("part_two");

    let oxygen = choose_for_criteria(input, true);
    println!("oxygen {:?} / {}", oxygen, bit_vec_to_u32(oxygen));
    let co2 = choose_for_criteria(input, false);
    println!("co2 {:?} / {}", co2, bit_vec_to_u32(co2));
    let life_support_rating = bit_vec_to_u32(oxygen) * bit_vec_to_u32(co2);
    println!("life support rating = {}", life_support_rating);
}

fn choose_for_criteria(input: &[Vec<bool>], criteria: bool) -> &Vec<bool> {
    let width = input[0].len();
    let mut numbers = input.iter().collect::<Vec<_>>();
    for bit_n in 0..width {
        let count_criteria = numbers
            .iter()
            .map(|num| num[bit_n])
            .filter(|&b| b == criteria)
            .count();
        let to_retain = match (count_criteria * 2).cmp(&numbers.len()) {
            Ordering::Greater => true,
            Ordering::Less => false,
            Ordering::Equal => criteria,
        };
        numbers.retain(|num| num[bit_n] == to_retain);
        if numbers.len() == 1 {
            break;
        }
        if numbers.len() == 0 {
            panic!("Oops, no numbers left");
        }
    }
    if numbers.len() > 1 {
        panic!("Still too many numbers left ({})", numbers.len());
    }
    numbers[0]
}

fn read_input() -> Result<Vec<Vec<bool>>> {
    let mut nums = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        nums.push(
            line.chars()
                .map(|ch| match ch {
                    '1' => Ok(true),
                    '0' => Ok(false),
                    c => Err(format!("Unexpected character '{}'", c)),
                })
                .collect::<std::result::Result<_, _>>()?,
        );
    }
    Ok(nums)
}
