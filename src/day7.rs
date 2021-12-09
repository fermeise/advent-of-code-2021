fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn solve_part1(input: &str) -> i32 {
    let mut crabs = parse(input);
    crabs.sort();
    let pos = crabs[crabs.len() / 2];
    crabs.iter().map(|x| (x - pos).abs()).sum()
}

pub fn solve(input: &str) -> i32 {
    let crabs = parse(input);
    (*crabs.iter().min().unwrap()..=*crabs.iter().max().unwrap())
        .map(|pos| {
            crabs
                .iter()
                .map(|x| {
                    let d = (x - pos).abs();
                    (d * d + d) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}
