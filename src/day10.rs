use itertools::Itertools;

#[allow(dead_code)]
pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    c => {
                        if let Some(top) = stack.pop() {
                            if c != top {
                                return match c {
                                    ')' => 3,
                                    ']' => 57,
                                    '}' => 1197,
                                    '>' => 25137,
                                    _ => panic!(),
                                };
                            }
                        }
                    }
                }
            }
            0
        })
        .sum()
}

pub fn solve(input: &str) -> u64 {
    let scores: Vec<u64> = input
        .lines()
        .filter_map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    c => {
                        if let Some(top) = stack.pop() {
                            if c != top {
                                return None;
                            }
                        }
                    }
                }
            }
            let score = stack
                .into_iter()
                .rev()
                .map(|c| match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                })
                .fold(0, |acc, score| acc * 5 + score);
            Some(score)
        })
        .sorted()
        .collect();
    scores[scores.len() / 2]
}
