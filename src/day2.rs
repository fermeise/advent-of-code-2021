#[allow(dead_code)]
fn solve_part1(input: &str) -> i32 {
    let mut depth: i32 = 0;
    let mut dist: i32 = 0;
    for x in input.lines() {
        if let (Some(op), Some(value)) = scan_fmt!(x, "{} {d}", String, i32) {
            match op.as_ref() {
                "forward" => dist += value,
                "down" => depth += value,
                "up" => depth -= value,
                _ => panic!("Unknown command"),
            }
        } else {
            panic!()
        }
    }
    dist * depth
}

pub fn solve(input: &str) -> i32 {
    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    let mut dist: i32 = 0;
    for x in input.lines() {
        if let (Some(op), Some(value)) = scan_fmt!(x, "{} {d}", String, i32) {
            match op.as_ref() {
                "forward" => {
                    dist += value;
                    depth += aim * value;
                }
                "down" => aim += value,
                "up" => aim -= value,
                _ => panic!("Unknown command"),
            }
        } else {
            panic!()
        }
    }
    dist * depth
}
