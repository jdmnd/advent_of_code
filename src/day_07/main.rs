use std::io;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = read_input()?;
    input.sort();
    let median = input[input.len() / 2];
    println!("median = {}", median);
    let mut d = dist(median, &input);
    let step = if dist(median + 1, &input) < d { 1 } else { -1 } as i32;
    let mut current = median as i32;
    loop {
        println!("current = {}, d = {}", current, d);
        let cand = dist((current + step) as u32, &input);
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

fn dist(from: u32, positions: &Vec<u32>) -> u32 {
    positions
        .iter()
        .map(|&pos| {
            let n = (from as i32 - pos as i32).abs() as u32;
            (n * (n + 1)) / 2
        })
        .sum()
}

fn read_input() -> Result<Vec<u32>> {
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
