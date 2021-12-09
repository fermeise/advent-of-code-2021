use std::collections::HashMap;

fn parse_fishes(input: &str) -> HashMap<u64, u64> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        })
}

fn simulate(fishes: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut result = HashMap::new();
    for (&fish, &count) in fishes.iter() {
        if fish > 0 {
            *result.entry(fish - 1).or_insert(0) += count;
        } else {
            *result.entry(6).or_insert(0) += count;
            *result.entry(8).or_insert(0) += count;
        }
    }
    result
}

#[allow(dead_code)]
pub fn solve_part1(input: &str) -> u64 {
    let mut fishes = parse_fishes(input);
    for _ in 0..80 {
        fishes = simulate(fishes)
    }
    fishes.values().sum()
}

pub fn solve(input: &str) -> u64 {
    let mut fishes = parse_fishes(input);
    for _ in 0..256 {
        fishes = simulate(fishes)
    }
    fishes.values().sum()
}
