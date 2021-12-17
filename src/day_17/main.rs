use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

fn main() -> Result<()> {
    let target = read_input()?;
    println!("target = {:?}", &target);

    let mut max = (i32::MIN, i32::MIN);
    let mut count = 0;
    for v_x in 0..=target.x_max {
        for v_y in target.y_min..=target.x_max {
            if trajectory_intersects(&target, v_x, v_y) {
                count += 1;
                println!("found intersecting trajectory {} {}", v_x, v_y);
                if v_y > max.1 {
                    println!("best yet!");
                    max = (v_x, v_y);
                }
            }
        }
    }

    fn trajectory_maximum(v_y: i32) -> i32 {
        v_y * (v_y + 1) / 2
    }
    let maximum = trajectory_maximum(max.1);

    println!("best initial velocity that intersects: {:?}", max);
    println!("maximum of best trajectory: {:?}", maximum);
    println!("total number of intersecting trajectories: {:?}", count);
    Ok(())
}

fn trajectory_intersects(target: &TargetArea, mut v_x: i32, mut v_y: i32) -> bool {
    let mut x = 0;
    let mut y = 0;
    loop {
        x += v_x;
        y += v_y;
        v_x -= v_x.signum();
        v_y -= 1;
        if x >= target.x_min && x <= target.x_max && y >= target.y_min && y <= target.y_max {
            return true;
        }
        if x > target.x_max || y < target.y_min {
            return false;
        }
    }
}

fn read_input() -> Result<TargetArea> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let pattern = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)")?;
    let captures = pattern.captures(&input).ok_or("failed to match input")?;
    Ok(TargetArea {
        x_min: captures[1].parse()?,
        x_max: captures[2].parse()?,
        y_min: captures[3].parse()?,
        y_max: captures[4].parse()?,
    })
}
