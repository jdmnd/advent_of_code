use array2d::Array2D;
use std::array;
use std::collections::BTreeSet;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let matrix = read_input()?;
    //println!("matrix = {:?}", &matrix);
    part_1(&matrix);
    part_2(&matrix);
    Ok(())
}

fn neighbours((y, x): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (y, x) = (y as i64, x as i64);
    array::IntoIter::new([(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)])
        .filter(|&(y, x)| y >= 0 && x >= 0)
        .map(|(y, x)| (y as usize, x as usize))
}

fn find_minima(matrix: &Array2D<u8>) -> Vec<(usize, usize)> {
    let mut minima = vec![];
    for y in 0..matrix.num_rows() as i32 {
        for x in 0..matrix.num_columns() as i32 {
            let pos = (y as usize, x as usize);
            let value = matrix[pos];
            let is_min = neighbours(pos).all(|(neighbour_y, neighbour_x)| {
                if let Some(&neighbour) = matrix.get(neighbour_y, neighbour_x) {
                    return value < neighbour;
                }
                return true;
            });
            if is_min {
                minima.push((y as usize, x as usize));
            }
        }
    }
    minima
}

fn print_marked_matrix(matrix: &Array2D<u8>, marked_points: &BTreeSet<(usize, usize)>) {
    for y in 0..matrix.num_rows() {
        for x in 0..matrix.num_columns() {
            let pos = (y as usize, x as usize);
            let value = matrix[pos];
            if marked_points.contains(&pos) {
                print!("\u{001b}[1m{}\u{001b}[0m", value);
            } else {
                print!("{}", value);
            }
        }
        println!()
    }
}

fn part_1(matrix: &Array2D<u8>) {
    let minima = find_minima(matrix);
    //print_marked_matrix(matrix, &minima.clone().into_iter().collect());
    let total_risk: i64 = minima
        .iter()
        .map(|&pos| matrix[pos])
        .map(|m| m as i64 + 1)
        .sum();
    println!("total risk = {}", total_risk);
}

fn grow(matrix: &Array2D<u8>, from: (usize, usize), basin: &mut BTreeSet<(usize, usize)>) {
    for neighbour_pos @ (neighbour_y, neighbour_x) in neighbours(from) {
        if let Some(&neighbour) = matrix.get(neighbour_y, neighbour_x) {
            if neighbour == 9 {
                continue;
            }
            if basin.insert(neighbour_pos) {
                grow(matrix, neighbour_pos, basin)
            }
        }
    }
}

fn part_2(matrix: &Array2D<u8>) {
    let minima = find_minima(matrix);
    let mut basins: Vec<_> = minima
        .into_iter()
        .map(|minimum| {
            let mut basin: BTreeSet<(usize, usize)> = BTreeSet::new();
            grow(matrix, minimum, &mut basin);
            //print_marked_matrix(matrix, &basin.clone().into_iter().collect());
            basin.len()
        })
        .collect();
    basins.sort();
    basins.reverse();
    let largest: Vec<_> = basins.iter().take(3).collect();
    println!(
        "largest basins: {:?} product = {}",
        &largest,
        largest.iter().map(|&&v| v as i64).product::<i64>()
    )
}

fn read_input() -> Result<Array2D<u8>> {
    let mut rows = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        rows.push(
            line.trim()
                .chars()
                .map(|ch| ch as u8 - '0' as u8)
                .collect::<Vec<_>>(),
        );
    }
    Ok(Array2D::from_rows(&rows))
}
