use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;
    println!("Input = {:?}", &input);

    part_one(&input);
    part_two(&input);
    Ok(())
}

fn part_one(input: &[Instruction]) {
    println!("- part one");
    let pos = input.iter().fold((0, 0), |(x, y), inst| match inst {
        Instruction::Forward(n) => (x + n, y),
        Instruction::Up(n) => (x, y - n),
        Instruction::Down(n) => (x, y + n),
    });
    println!("position = {:?}", pos);
    let (x, y) = pos;
    println!("product = {:?}", x * y);
}

#[derive(Debug)]
struct State {
    aim: i32,
    x: i32,
    y: i32,
}

impl State {
    fn new() -> Self {
        State { aim: 0, x: 0, y: 0 }
    }

    fn increase_aim(mut self, amount: i32) -> Self {
        self.aim += amount;
        self
    }

    fn move_forward(mut self, amount: i32) -> Self {
        self.x += amount;
        self.y += self.aim * amount;
        self
    }
}

fn part_two(input: &[Instruction]) {
    println!("- part two");
    let pos = input.iter().fold(State::new(), |state, inst| match inst {
        Instruction::Forward(n) => state.move_forward(*n as i32),
        Instruction::Up(n) => state.increase_aim(-(*n as i32)),
        Instruction::Down(n) => state.increase_aim(*n as i32),
    });
    println!("position = {:?}", pos);
    let State { x, y, .. } = pos;
    println!("product = {:?}", x * y);
}

#[derive(Debug)]
enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn read_input() -> Result<Vec<Instruction>> {
    let stdin = io::stdin();
    let mut instructions = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        let instruction = match line.split_once(' ').unwrap() {
            ("forward", dist) => Instruction::Forward(dist.parse()?),
            ("up", dist) => Instruction::Up(dist.parse()?),
            ("down", dist) => Instruction::Down(dist.parse()?),
            _ => Err("unexpected instruction")?,
        };
        instructions.push(instruction);
    }
    Ok(instructions)
}
