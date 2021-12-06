use std::collections::HashSet;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;
    part_1(&input);
    part_2(&input);
    Ok(())
}

fn part_1(input: &[LineSegment]) {
    println!("part 1");
    let all_points = input
        .iter()
        .flat_map(|&line| points_on_line_part_1(line).into_iter())
        .collect::<Vec<_>>();
    println!("duplicates ({})", count_duplicates(&all_points));
}

fn part_2(input: &[LineSegment]) {
    println!("part 2");

    let all_points = input
        .iter()
        .flat_map(|&line| points_on_line_part_2(line).into_iter())
        .collect::<Vec<_>>();
    println!("duplicates ({})", count_duplicates(&all_points));
}

fn count_duplicates(all_points: &[Point]) -> usize {
    let mut unique_points = HashSet::new();
    let mut duplicates = HashSet::new();
    for point in all_points {
        if !unique_points.insert(point) {
            duplicates.insert(point);
        }
    }
    return duplicates.len();
}

type Point = (u32, u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LineSegment {
    from: Point,
    to: Point,
}

fn range(from: u32, to: u32) -> Box<dyn Iterator<Item = u32>> {
    if from < to {
        Box::from(from..=to)
    } else {
        Box::from((to..=from).rev())
    }
}

fn points_on_line_part_1(line: LineSegment) -> Vec<Point> {
    let LineSegment {
        from: (from_x, from_y),
        to: (to_x, to_y),
    } = line;
    if from_x == to_x {
        range(from_y, to_y).map(|y| (from_x, y)).collect()
    } else if from_y == to_y {
        range(from_x, to_x).map(|x| (x, from_y)).collect()
    } else {
        vec![]
    }
}

fn points_on_line_part_2(line: LineSegment) -> Vec<Point> {
    let LineSegment {
        from: (from_x, from_y),
        to: (to_x, to_y),
    } = line;
    if from_x == to_x {
        range(from_y, to_y).map(|y| (from_x, y)).collect()
    } else if from_y == to_y {
        range(from_x, to_x).map(|x| (x, from_y)).collect()
    } else {
        let range = range(from_x, to_x).zip(range(from_y, to_y)).collect();
        println!("diagonal {:?} = {:?}", &line, range);
        range
    }
}

fn read_input() -> Result<Vec<LineSegment>> {
    let mut line_segments = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let (from_str, to_str) = line.split_once(" -> ").ok_or("failed to split parts")?;
        let (from_x, from_y) = from_str
            .split_once(",")
            .ok_or("failed to split from part")?;
        let (to_x, to_y) = to_str.split_once(",").ok_or("failed to split to part")?;
        line_segments.push(LineSegment {
            from: (from_x.parse()?, from_y.parse()?),
            to: (to_x.parse()?, to_y.parse()?),
        });
    }
    Ok(line_segments)
}
