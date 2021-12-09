use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

fn adjacent(y: usize, x: usize, height: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();
    if y > 0 {
        adjacent.push((y - 1, x));
    }
    if x > 0 {
        adjacent.push((y, x - 1))
    }
    if y < height.len() - 1 {
        adjacent.push((y + 1, x))
    }
    if x < height[y].len() - 1 {
        adjacent.push((y, x + 1))
    }
    adjacent
}

fn is_low_point(y: usize, x: usize, height: &Vec<Vec<u32>>) -> bool {
    adjacent(y, x, height)
        .into_iter()
        .all(|(w, v)| height[w][v] > height[y][x])
}

#[allow(dead_code)]
pub fn solve_part1(input: &str) -> u32 {
    let height = parse(input);
    (0..height.len())
        .map(|y| {
            (0..height[y].len())
                .map(|x| {
                    if is_low_point(y, x, &height) {
                        height[y][x] + 1
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

pub fn solve(input: &str) -> usize {
    let height = parse(input);
    let basins: Vec<usize> = (0..height.len())
        .map(|y| {
            (0..height[y].len())
                .map(|x| {
                    if is_low_point(y, x, &height) {
                        let mut basin = HashSet::new();
                        let mut queue = VecDeque::new();
                        queue.push_back((y, x));
                        while let Some((y, x)) = queue.pop_front() {
                            if height[y][x] < 9 && !basin.contains(&(y, x)) {
                                let adj = adjacent(y, x, &height);
                                if adj.iter().all(|&(w, v)| {
                                    height[w][v] >= height[y][x] || basin.contains(&(w, v))
                                }) {
                                    basin.insert((y, x));
                                    adj.into_iter().for_each(|(w, v)| {
                                        queue.push_back((w, v));
                                    });
                                }
                            }
                        }
                        basin.len()
                    } else {
                        0
                    }
                })
                .collect::<Vec<usize>>()
        })
        .flatten()
        .sorted()
        .collect();
    let n = basins.len();
    basins[n - 3] * basins[n - 2] * basins[n - 1]
}
