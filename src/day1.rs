use itertools::Itertools;

#[allow(dead_code)]
fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}
