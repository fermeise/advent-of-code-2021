use std::collections::HashSet;

fn parse_draws(line: &str) -> Vec<u32> {
    line.split(',').map(|x| x.parse::<u32>().unwrap()).collect()
}

fn parse_board<'a, I: Iterator<Item = &'a str>>(lines: I) -> Vec<Vec<u32>> {
    lines
        .take(5)
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn parse_boards<'a, I: Iterator<Item = &'a str>>(mut lines: I) -> Vec<Vec<Vec<u32>>> {
    let mut boards = Vec::new();
    while lines.next().is_some() {
        boards.push(parse_board(lines.by_ref()));
    }
    boards
}

fn is_bingo(board: &Vec<Vec<u32>>, draws: &HashSet<u32>) -> bool {
    (0..5).any(|i| (0..5).all(|k| draws.contains(&board[i][k])))
        || (0..5).any(|k| (0..5).all(|i| draws.contains(&board[i][k])))
}

fn unmarked_sum(board: &Vec<Vec<u32>>, draws: &HashSet<u32>) -> u32 {
    board.iter().flatten().filter(|x| !draws.contains(x)).sum()
}

#[allow(dead_code)]
pub fn solve_part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let draws = parse_draws(lines.next().unwrap());
    let boards = parse_boards(lines);

    let mut drawn = HashSet::new();
    for draw in draws {
        drawn.insert(draw);
        for board in &boards {
            if is_bingo(board, &drawn) {
                return draw * unmarked_sum(&board, &drawn);
            }
        }
    }
    0
}

pub fn solve(input: &str) -> u32 {
    let mut lines = input.lines();
    let draws = parse_draws(lines.next().unwrap());
    let boards = parse_boards(lines);
    let mut board_won = vec![false; boards.len()];

    let mut last_score = 0;
    let mut drawn = HashSet::new();
    for draw in draws {
        drawn.insert(draw);
        for i in 0..boards.len() {
            let board = &boards[i];
            if !board_won[i] && is_bingo(board, &drawn) {
                board_won[i] = true;
                last_score = draw * unmarked_sum(&board, &drawn);
            }
        }
    }
    last_score
}
