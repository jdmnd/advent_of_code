use array2d::Array2D;
use std::array;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;
    let mut matrix = input.image;
    print_matrix(&matrix);
    for _ in 0..50 {
        matrix = grow_by(2, matrix);
        matrix = step(&input.lookup, matrix);
        matrix = shrink_by(1, matrix);
    }
    print_matrix(&matrix);
    Ok(())
}

fn step(lookup: &Vec<bool>, matrix: Array2D<bool>) -> Array2D<bool> {
    let mut new_matrix = Array2D::filled_with(false, matrix.num_rows(), matrix.num_columns());
    for y in 0..matrix.num_rows() {
        for x in 0..matrix.num_columns() {
            new_matrix[(y, x)] = lookup[compute_index(&matrix, y, x)];
        }
    }
    new_matrix
}

fn grow_by(n: usize, matrix: Array2D<bool>) -> Array2D<bool> {
    let width = matrix.num_columns() + n * 2;
    let height = matrix.num_rows() + n * 2;
    let mut new_matrix = Array2D::filled_with(matrix[(0, 0)], height, width);
    for y in 0..matrix.num_rows() {
        for x in 0..matrix.num_columns() {
            new_matrix[(y + n, x + n)] = matrix[(y, x)];
        }
    }
    new_matrix
}

fn shrink_by(n: usize, matrix: Array2D<bool>) -> Array2D<bool> {
    Array2D::from_rows(
        &matrix
            .rows_iter()
            .skip(n)
            .take(matrix.num_rows() - n * 2)
            .map(|row| {
                row.skip(n)
                    .take(matrix.num_columns() - n * 2)
                    .map(|&b| b)
                    .collect()
            })
            .collect::<Vec<_>>(),
    )
}

fn compute_index(matrix: &Array2D<bool>, y: usize, x: usize) -> usize {
    let get = |y, x| {
        if y < 0 || x < 0 {
            false
        } else {
            *matrix.get(y as usize, x as usize).unwrap_or(&false)
        }
    };
    let y = y as i32;
    let x = x as i32;
    let get_row = |y| array::IntoIter::new([get(y, x - 1), get(y, x), get(y, x + 1)]);
    get_row(y - 1)
        .chain(get_row(y))
        .chain(get_row(y + 1))
        .fold(0, |acc, b| acc << 1 | b as usize)
}

fn print_matrix(matrix: &Array2D<bool>) {
    for y in 0..matrix.num_rows() {
        for x in 0..matrix.num_columns() {
            if matrix[(y, x)] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
    println!(
        "num lit = {}",
        matrix.elements_row_major_iter().filter(|&&x| x).count()
    );
}

#[derive(Clone, Debug)]
struct Input {
    lookup: Vec<bool>,
    image: Array2D<bool>,
}

fn read_input() -> Result<Input> {
    let stdin = io::stdin();

    let mut first_line = String::new();
    stdin.read_line(&mut first_line)?;

    let from_char = |ch| match ch {
        '.' => false,
        '#' => true,
        c => panic!("expected . or #; found {}", c),
    };

    let lookup: Vec<bool> = first_line.trim().chars().map(from_char).collect();

    let mut rows = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        rows.push(line.chars().map(from_char).collect::<Vec<_>>())
    }
    Ok(Input {
        lookup,
        image: Array2D::from_rows(&rows),
    })
}
