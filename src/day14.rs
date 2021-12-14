use itertools::Itertools;
use std::collections::HashMap;

fn parse(input: &str) -> (String, HashMap<(char, char), char>) {
    let mut input = input.split("\n\n");
    let template = input.next().unwrap().to_string();
    let rules = input
        .next()
        .unwrap()
        .lines()
        .filter_map(|line| {
            let (lhs, rhs) = scan_fmt!(line, "{} -> {}", String, char);
            let lhs = lhs.unwrap();
            let mut chars = lhs.chars();
            Some(((chars.next().unwrap(), chars.next().unwrap()), rhs.unwrap()))
        })
        .collect();
    (template, rules)
}

fn apply_rules(
    pairs: HashMap<(char, char), u64>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
    let mut result = HashMap::new();
    pairs.into_iter().for_each(|((a, b), n)| {
        let c = rules[&(a, b)];
        *result.entry((a, c)).or_insert(0) += n;
        *result.entry((c, b)).or_insert(0) += n;
    });
    result
}

pub fn solve_n_steps(input: &str, steps: u32) -> u64 {
    let (template, rules) = parse(input);

    let last_char = template.chars().last().unwrap();
    let mut pairs = HashMap::new();
    template.chars().tuple_windows().for_each(|(a, b)| {
        *pairs.entry((a, b)).or_insert(0) += 1;
    });

    for _ in 0..steps {
        pairs = apply_rules(pairs, &rules);
    }

    let mut chars: HashMap<char, u64> = HashMap::new();
    pairs.into_iter().for_each(|((a, _), n)| {
        *chars.entry(a).or_insert(0) += n;
    });
    *chars.entry(last_char).or_insert(0) += 1;

    chars.values().max().unwrap() - chars.values().min().unwrap()
}

pub fn solve(input: &str) -> (u64, u64) {
    (solve_n_steps(input, 10), solve_n_steps(input, 40))
}
