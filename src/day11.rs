fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

fn adjacent(y: usize, x: usize, height: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();
    if y > 0 && x > 0 {
        adjacent.push((y - 1, x - 1));
    }
    if y > 0 {
        adjacent.push((y - 1, x));
    }
    if y > 0 && x < height[y].len() - 1 {
        adjacent.push((y - 1, x + 1));
    }
    if x > 0 {
        adjacent.push((y, x - 1))
    }
    if x < height[y].len() - 1 {
        adjacent.push((y, x + 1))
    }
    if y < height.len() - 1 && x > 0 {
        adjacent.push((y + 1, x - 1));
    }
    if y < height.len() - 1 {
        adjacent.push((y + 1, x));
    }
    if y < height.len() - 1 && x < height[y].len() - 1 {
        adjacent.push((y + 1, x + 1));
    }
    adjacent
}

fn step(levels: Vec<Vec<u32>>) -> (Vec<Vec<u32>>, u32) {
    let mut levels: Vec<Vec<u32>> = levels
        .into_iter()
        .map(|line| line.into_iter().map(|x| x + 1).collect())
        .collect();

    let height = levels.len();
    let width = levels[0].len();

    let mut flashed = vec![vec![false; width]; height];
    let mut flashes = 0;
    let mut done = false;

    while !done {
        done = true;
        for y in 0..height {
            for x in 0..width {
                if levels[y][x] > 9 && !flashed[y][x] {
                    flashed[y][x] = true;
                    flashes += 1;
                    adjacent(y, x, &levels)
                        .into_iter()
                        .for_each(|(w, v)| levels[w][v] += 1);
                    done = false;
                }
            }
        }
    }

    let levels = levels
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|x| if x > 9 { 0 } else { x })
                .collect()
        })
        .collect();
    (levels, flashes)
}

#[allow(dead_code)]
pub fn solve_part1(input: &str) -> u32 {
    let levels = parse(input);

    (0..100)
        .fold((levels, 0), |(levels, prev_flashes), _| {
            let (levels, new_flashes) = step(levels);
            (levels, prev_flashes + new_flashes)
        })
        .1
}

pub fn solve(input: &str) -> u32 {
    let mut levels = parse(input);

    for i in 0..1000 {
        let (new_levels, flashes) = step(levels);
        levels = new_levels;
        if flashes == 100 {
            return i + 1;
        }
    }
    return 0;
}
