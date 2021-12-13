use std::collections::BTreeSet;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;
    println!("Input = {:?}", &input);
    part_1(&input);
    part_2(&input);

    Ok(())
}

fn fold_coords(coords: BTreeSet<(i32, i32)>, fold: Fold) -> BTreeSet<(i32, i32)> {
    coords
        .into_iter()
        .map(|(x, y)| match fold {
            Fold::Horizontal(fold_y) => {
                if y > fold_y {
                    (x, fold_y - (fold_y - y).abs())
                } else {
                    (x, y)
                }
            }
            Fold::Vertical(fold_x) => {
                if x > fold_x {
                    (fold_x - (fold_x - x).abs(), y)
                } else {
                    (x, y)
                }
            }
        })
        .collect()
}

fn part_1(input: &Input) {
    let fold = input.folds[0];
    let mut coords = input.dot_coords.iter().map(|&c| c).collect();
    coords = fold_coords(coords, fold);
    println!("After fold = ({}) {:?}", coords.len(), coords);
}

fn part_2(input: &Input) {
    let mut coords = input.dot_coords.iter().map(|&c| c).collect();
    for &fold in &input.folds {
        coords = fold_coords(coords, fold);
    }
    println!("After all folds = ({}) {:?}", coords.len(), coords);
    let &max_x = coords.iter().map(|(x, _)| x).max().unwrap();
    let &max_y = coords.iter().map(|(_, y)| y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if coords.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[derive(Clone, Copy, Debug)]
enum Fold {
    Horizontal(i32),
    Vertical(i32),
}

#[derive(Debug)]
struct Input {
    dot_coords: Vec<(i32, i32)>,
    folds: Vec<Fold>,
}

fn read_input() -> Result<Input> {
    let mut dot_coords = vec![];
    let mut folds = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        if line.starts_with("fold along") {
            let fold = match line
                .split_once('=')
                .ok_or(format!("no split character in {:?}", line))?
            {
                ("fold along x", pos) => Fold::Vertical(pos.parse()?),
                ("fold along y", pos) => Fold::Horizontal(pos.parse()?),
                fold => Err(format!("invalid fold {:?}", fold))?,
            };
            folds.push(fold);
        } else {
            let (x, y) = line
                .split_once(',')
                .ok_or(format!("no split character in {:?}", line))?;
            dot_coords.push((x.parse()?, y.parse()?));
        }
    }
    Ok(Input { dot_coords, folds })
}
