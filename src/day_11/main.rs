use array2d::Array2D;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn bold(value: &str) -> String {
    format!("\u{001b}[1m{}\u{001b}[0m", value)
}

fn main() -> Result<()> {
    let matrix = read_input()?;
    part_1(matrix.clone())?;
    part_2(matrix)?;

    Ok(())
}

fn part_1(mut matrix: Array2D<Cell>) -> Result<()> {
    println!("Input = {:?}", &matrix);
    let count_flashes: u32 = (1..=100)
        .map(|i| {
            println!("after step {}", i);
            step(&mut matrix)
        })
        .sum();
    println!("count_flashes = {}", count_flashes);
    Ok(())
}

fn part_2(mut matrix: Array2D<Cell>) -> Result<()> {
    for i in 1.. {
        println!("after step {}", i);
        let count_flashes = step(&mut matrix);
        if count_flashes == (matrix.num_rows() * matrix.num_columns()) as u32 {
            break;
        }
    }
    Ok(())
}

fn print_matrix(matrix: &Array2D<Cell>) {
    for y in 0..matrix.num_rows() {
        for x in 0..matrix.num_columns() {
            if let Cell::Energy(value) = matrix[(y, x)] {
                print!("{}", value);
            } else {
                print!("{}", bold("0"));
            }
        }
        println!()
    }
}

fn neighbours((y_base, x_base): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (y_base, x_base) = (y_base as i64, x_base as i64);
    (y_base - 1..=y_base + 1).flat_map(move |y| {
        (x_base - 1..=x_base + 1)
            .filter(move |&x| (y != y_base || x != x_base) && y >= 0 && x >= 0)
            .map(move |x| (y as usize, x as usize))
    })
}

fn step(matrix: &mut Array2D<Cell>) -> u32 {
    for y in 0..matrix.num_rows() {
        for x in 0..matrix.num_columns() {
            match matrix.get_mut(y, x).unwrap() {
                Cell::Energy(value) => *value += 1,
                _ => (),
            }
        }
    }
    let mut count_flashes = 0;
    while let Some(pos) = find_should_flash(&matrix) {
        count_flashes += 1;
        matrix[pos] = Cell::Flashed;
        for (neighbour_y, neighbour_x) in neighbours(pos) {
            if let Some(Cell::Energy(neighbour)) = matrix.get_mut(neighbour_y, neighbour_x) {
                *neighbour += 1;
            }
        }
    }
    print_matrix(&matrix);
    for y in 0..matrix.num_rows() {
        for x in 0..matrix.num_columns() {
            match &mut matrix[(y, x)] {
                cell @ Cell::Flashed => *cell = Cell::Energy(0),
                _ => (),
            }
        }
    }
    count_flashes
}

fn find_should_flash(matrix: &Array2D<Cell>) -> Option<(usize, usize)> {
    for y in 0..matrix.num_rows() {
        for x in 0..matrix.num_columns() {
            if let Some(&Cell::Energy(v)) = matrix.get(y, x) {
                if v > 9 {
                    return Some((y, x));
                }
            }
        }
    }
    None
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Energy(u8),
    Flashed,
}

fn read_input() -> Result<Array2D<Cell>> {
    let mut rows = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        rows.push(
            line.chars()
                .map(|ch| ch.to_digit(10).map(|digit| Cell::Energy(digit as u8)))
                .collect::<Option<_>>()
                .ok_or(format!("failed to parse line as digits '{}'", line))?,
        )
    }
    Ok(Array2D::from_rows(&rows))
}

#[cfg(test)]
mod test {
    use super::neighbours;

    #[test]
    fn test_neighbours() {
        let ns: Vec<_> = neighbours((1, 1)).collect();
        assert_eq!(ns.len(), 8);
        for (x, y) in ns {
            assert!((1 - x as i32).abs() == 1 || (1 - y as i32).abs() == 1);
        }
    }
    #[test]
    fn test_neighbours_at_zero() {
        let ns: Vec<_> = neighbours((0, 0)).collect();
        println!("neighbours = {ns:?}", ns = &ns);
        assert_eq!(ns.len(), 3);
        for (x, y) in ns {
            assert!(x == 1 || y == 1);
        }
    }
}
