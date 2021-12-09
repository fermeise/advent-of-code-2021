#[allow(dead_code)]
pub fn solve_part1(input: &str) -> u32 {
    let digit_count = input.lines().fold(vec![], |mut acc, number| {
        if acc.is_empty() {
            acc = vec![(0, 0); number.len()];
        }
        for (c, count) in number.chars().zip(acc.iter_mut()) {
            if c == '1' {
                count.1 += 1;
            } else {
                count.0 += 1;
            }
        }
        acc
    });

    let mut gamma = 0;
    let mut epsilon = 0;
    for (zeroes, ones) in digit_count {
        let digit = if ones > zeroes { 1 } else { 0 };
        gamma = gamma * 2 + digit;
        epsilon = epsilon * 2 + 1 - digit;
    }

    gamma * epsilon
}

fn rating(mut report: Vec<&str>, swap: bool) -> u32 {
    let mut pos = 0;
    while report.len() > 1 {
        let ones = report
            .iter()
            .filter(|x| x.chars().nth(pos).unwrap() == '1')
            .count();
        let zeroes = report.len() - ones;
        let digit = if (zeroes > ones) != swap { '0' } else { '1' };
        report = report
            .into_iter()
            .filter(|x| x.chars().nth(pos).unwrap() == digit)
            .collect();
        pos += 1;
    }
    let mut rating = 0;
    for digit in report[0].chars() {
        rating *= 2;
        if digit == '1' {
            rating += 1;
        }
    }
    rating
}

pub fn solve(input: &str) -> u32 {
    let report: Vec<&str> = input.lines().collect();

    rating(report.clone(), false) * rating(report, true)
}
