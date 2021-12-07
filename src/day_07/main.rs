use std::io;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = read_input()?;
    input.sort();
    let median = input[input.len() / 2];
    let mean = input.iter().sum::<i32>() / input.len() as i32;
    println!("median = {}", median);
    println!("mean = {}", mean);
    let mut current = mean;
    let mut d = dist(current, &input);
    let step = if dist(current + 1, &input) < d { 1 } else { -1 };
    loop {
        println!("current = {}, d = {}", current, d);
        let cand = dist(current + step, &input);
        if cand < d {
            d = cand;
            current += step;
        } else {
            break;
        }
    }
    println!("minimum is {} with dist {}", current, d);
    Ok(())
}

fn dist(from: i32, positions: &Vec<i32>) -> i32 {
    positions
        .iter()
        .map(|&pos| {
            let n = (from - pos).abs();
            (n * (n + 1)) / 2
        })
        .sum()
}

fn read_input() -> Result<Vec<i32>> {
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
