use itertools::Itertools;
use std::collections::HashSet;

fn parse_stars(input: &str) -> HashSet<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .next_tuple()
                .unwrap()
        })
        .collect()
}

fn fold(input: HashSet<(u32, u32)>, fold_x: u32, fold_y: u32) -> HashSet<(u32, u32)> {
    input
        .into_iter()
        .map(|(x, y)| {
            (
                if x > fold_x { 2 * fold_x - x } else { x },
                if y > fold_y { 2 * fold_y - y } else { y },
            )
        })
        .collect()
}

fn print_stars(stars: &HashSet<(u32, u32)>) {
    let width = stars.iter().map(|(x, _)| *x).max().unwrap();
    let height = stars.iter().map(|(_, y)| *y).max().unwrap();
    for y in 0..=height {
        for x in 0..=width {
            print!("{}", if stars.contains(&(x, y)) { "*" } else { " " });
        }
        println!("");
    }
}

pub fn solve(input: &str) -> usize {
    let mut input = input.split("\n\n");
    let mut stars = parse_stars(input.next().unwrap());
    let mut stars_after_first_fold = None;

    for line in input.next().unwrap().lines() {
        if let (Some(axis), Some(coord)) = scan_fmt!(line, "fold along {}={d}", String, u32) {
            stars = if axis == "x" {
                fold(stars, coord, u32::MAX)
            } else {
                fold(stars, u32::MAX, coord)
            };
            if stars_after_first_fold.is_none() {
                stars_after_first_fold = Some(stars.len())
            }
        }
    }
    print_stars(&stars);

    stars_after_first_fold.unwrap()
}
