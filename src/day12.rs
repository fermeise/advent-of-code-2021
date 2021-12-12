use std::collections::HashMap;

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut adj = HashMap::new();
    for line in input.lines() {
        if let [from, to] = line.split('-').collect::<Vec<&str>>().as_slice() {
            adj.entry(*from).or_insert(vec![]).push(*to);
            adj.entry(*to).or_insert(vec![]).push(*from);
        }
    }
    adj
}

pub fn backtrack<'a>(
    path: &mut Vec<&'a str>,
    adj: &'a HashMap<&str, Vec<&'a str>>,
    allow_small_twice: bool,
) -> u32 {
    if path.len() > 1 && *path.last().unwrap() == "start" {
        return 0;
    }
    if *path.last().unwrap() == "end" {
        return 1;
    }
    let mut path_count = 0;
    for next in adj[path.last().unwrap() as &str].iter() {
        let small_twice = next.chars().all(|c| c.is_lowercase()) && path.contains(next);
        if !small_twice || allow_small_twice {
            path.push(next);
            path_count += backtrack(path, adj, allow_small_twice && !small_twice);
            path.pop();
        }
    }
    return path_count;
}

#[allow(dead_code)]
pub fn solve_part1(input: &str) -> u32 {
    let adj = parse(input);
    backtrack(&mut vec!["start"], &adj, false)
}

pub fn solve(input: &str) -> u32 {
    let adj = parse(input);
    backtrack(&mut vec!["start"], &adj, true)
}
