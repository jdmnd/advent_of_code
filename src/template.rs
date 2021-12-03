use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;
    println!("Input = {:?}", &input);

    Ok(())
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
