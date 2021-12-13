use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let mut stars = HashSet::new();
    let mut fold = false;
    let mut stars_after_first_fold = None;

    for line in input.lines() {
        if !fold {
            if line.is_empty() {
                fold = true;
            } else if let [x, y] = line
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
                .as_slice()
            {
                stars.insert((*x, *y));
            }
        } else if let (Some(axis), Some(coord)) = scan_fmt!(line, "fold along {}={d}", String, u32)
        {
            match axis.as_ref() {
                "x" => {
                    stars = stars
                        .into_iter()
                        .map(|(x, y)| ((if x > coord { 2 * coord - x } else { x }, y)))
                        .collect();
                }
                "y" => {
                    stars = stars
                        .into_iter()
                        .map(|(x, y)| (x, (if y > coord { 2 * coord - y } else { y })))
                        .collect();
                }
                _ => {}
            }
            if stars_after_first_fold.is_none() {
                stars_after_first_fold = Some(stars.len())
            }
        }
    }

    let width = stars.iter().map(|(x, _)| *x).max().unwrap();
    let height = stars.iter().map(|(_, y)| *y).max().unwrap();
    for y in 0..=height {
        for x in 0..=width {
            print!("{}", if stars.contains(&(x, y)) { "*" } else { " " });
        }
        println!("");
    }
    stars_after_first_fold.unwrap()
}
