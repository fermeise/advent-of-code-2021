use std::collections::VecDeque;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

fn adjacent(y: usize, x: usize, map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();
    if y > 0 {
        adjacent.push((y - 1, x));
    }
    if x > 0 {
        adjacent.push((y, x - 1))
    }
    if y < map.len() - 1 {
        adjacent.push((y + 1, x))
    }
    if x < map[y].len() - 1 {
        adjacent.push((y, x + 1))
    }
    adjacent
}

fn find_min_risk(map: Vec<Vec<u32>>) -> u32 {
    let mut queue = VecDeque::new();
    let mut min_risk = vec![vec![u32::MAX; map[0].len()]; map.len()];
    queue.push_back((0, 0));
    min_risk[0][0] = 0;
    while let Some((y, x)) = queue.pop_front() {
        adjacent(y, x, &map).into_iter().for_each(|(w, v)| {
            if (min_risk[y][x] + map[w][v]) < min_risk[w][v] {
                min_risk[w][v] = min_risk[y][x] + map[w][v];
                queue.push_back((w, v));
            }
        });
    }
    min_risk[map.len() - 1][map[0].len() - 1]
}

#[allow(dead_code)]
pub fn solve_part1(input: &str) -> u32 {
    find_min_risk(parse(input))
}

pub fn solve(input: &str) -> u32 {
    let map = std::iter::repeat(parse(input))
        .take(5)
        .zip(0..5)
        .flat_map(|(tile, inc)| {
            tile.iter()
                .map(|line| {
                    std::iter::repeat(line)
                        .take(5)
                        .zip(inc..inc + 5)
                        .flat_map(|(line, inc)| {
                            line.iter()
                                .map(|x| (x + inc - 1) % 9 + 1)
                                .collect::<Vec<u32>>()
                        })
                        .collect()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .collect();
    find_min_risk(map)
}
