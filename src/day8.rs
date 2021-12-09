use itertools::Itertools;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split('|')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
                .count()
        })
        .sum()
}

fn substitute(digit: &str, map: &HashMap<&char, &char>) -> String {
    digit
        .chars()
        .map(|segment| *map[&segment])
        .sorted()
        .collect()
}

pub fn solve(input: &str) -> usize {
    let segments = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let ref_digits = vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];

    let permutations: HashMap<_, _> = segments
        .iter()
        .permutations(7)
        .map(|p| {
            let map: HashMap<_, _> = p.clone().into_iter().zip(segments.iter()).collect();
            let inv_map: HashMap<_, _> = segments.iter().zip(p.into_iter()).collect();
            let digits: Vec<String> = ref_digits
                .iter()
                .map(|digit| substitute(digit, &inv_map))
                .sorted()
                .collect();
            (digits.join(" "), map)
        })
        .collect();

    input
        .lines()
        .map(|line| {
            let digits: Vec<String> = line
                .split('|')
                .nth(0)
                .unwrap()
                .split_whitespace()
                .map(|digit| digit.chars().sorted().collect())
                .sorted()
                .collect();
            let number: Vec<_> = line.split('|').nth(1).unwrap().split_whitespace().collect();
            let permutation = &permutations[&digits.join(" ")];
            let correct_number = number
                .iter()
                .map(|digit| {
                    let correct_digit = substitute(digit, permutation);
                    (0..10)
                        .find(|value| ref_digits[*value] == correct_digit)
                        .unwrap()
                })
                .fold(0, |n, d| n * 10 + d);
            correct_number
        })
        .sum()
}
