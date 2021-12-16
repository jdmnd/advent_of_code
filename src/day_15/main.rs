use array2d::Array2D;
use std::array;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.distance_remaining)
            .cmp(&(self.cost + self.distance_remaining))
            .then(other.distance_remaining.cmp(&self.distance_remaining))
            .then(other.cost.cmp(&self.cost))
            .then(other.location.cmp(&self.location))
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Path {
    //path: Vec<(usize, usize)>,
    location: (usize, usize),
    cost: i64,
    distance_remaining: i64,
}

fn main() -> Result<()> {
    let grid = read_input()?;

    let destination = (grid.num_rows() - 1, grid.num_columns() - 1);

    if let Some(path) = shortest_path(&grid, destination) {
        println!("part 1 - shortest path = {:?}", path);
    }

    let grid = tile(&grid);
    let destination = (grid.num_rows() - 1, grid.num_columns() - 1);

    if let Some(path) = shortest_path(&grid, destination) {
        println!("part 2 - shortest path = {:?}", path);
    }

    Ok(())
}

fn add_wrap(a: u8, b: u8) -> u8 {
    1 + ((a + b - 1) % 9)
}

fn tile(grid: &Array2D<u8>) -> Array2D<u8> {
    let mut first_row_of_tiles: Vec<Vec<u8>> = vec![];
    for row in grid.rows_iter() {
        let row: Vec<u8> = row.cloned().collect();
        let mut tiled_row = vec![];
        for i in 0..5 {
            for &n in &row {
                tiled_row.push(add_wrap(n, i))
            }
        }
        first_row_of_tiles.push(tiled_row);
    }

    let mut all_tiles: Vec<Vec<u8>> = vec![];
    for i in 0..5 {
        for row in &first_row_of_tiles {
            all_tiles.push(row.iter().map(|&n| add_wrap(n, i)).collect());
        }
    }

    Array2D::from_rows(&all_tiles)
}

fn shortest_path(grid: &Array2D<u8>, destination: (usize, usize)) -> Option<Path> {
    let mut priority_queue = BinaryHeap::new();
    let mut best_position_costs = BTreeMap::new();
    priority_queue.push(Path {
        //path: vec![(0, 0)],
        location: (0, 0),
        cost: 0,
        distance_remaining: distance((0, 0), destination),
    });
    best_position_costs.insert((0, 0), 0);

    while let Some(current_path) = priority_queue.pop() {
        if current_path.distance_remaining == 0 {
            return Some(current_path);
        }
        let location = current_path.location;
        for (y, x) in neighbours(location) {
            if let Some(&value) = grid.get(y, x) {
                let cost = current_path.cost + value as i64;
                if best_position_costs
                    .get(&(y, x))
                    .map(|&best_cost| cost < best_cost)
                    .unwrap_or(true)
                {
                    best_position_costs.insert((y, x), cost);
                    let distance_remaining = distance((y, x), destination);
                    let path = Path {
                        location: (y, x),
                        cost,
                        distance_remaining,
                    };
                    priority_queue.push(path);
                }
            }
        }
    }
    None
}

fn distance((from_y, from_x): (usize, usize), (to_y, to_x): (usize, usize)) -> i64 {
    (to_y as i64 - from_y as i64).abs() + (to_x as i64 - from_x as i64).abs()
}

fn neighbours((y, x): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (y, x) = (y as i64, x as i64);
    array::IntoIter::new([(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)])
        .filter(|&(y, x)| y >= 0 && x >= 0)
        .map(|(y, x)| (y as usize, x as usize))
}

fn read_input() -> Result<Array2D<u8>> {
    let mut rows = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        rows.push(line.chars().map(|ch| ch as u8 - '0' as u8).collect());
    }
    Ok(Array2D::from_rows(&rows))
}
