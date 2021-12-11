mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

extern crate itertools;

#[macro_use]
extern crate scan_fmt;

use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let day: u8 = args.next().unwrap().parse().unwrap();
    let input_file = args.next().unwrap();

    let mut input = String::new();
    File::open(input_file)?.read_to_string(&mut input)?;

    match day {
        1 => println!("{:?}", day1::solve(&input)),
        2 => println!("{:?}", day2::solve(&input)),
        3 => println!("{:?}", day3::solve(&input)),
        4 => println!("{:?}", day4::solve(&input)),
        5 => println!("{:?}", day5::solve(&input)),
        6 => println!("{:?}", day6::solve(&input)),
        7 => println!("{:?}", day7::solve(&input)),
        8 => println!("{:?}", day8::solve(&input)),
        9 => println!("{:?}", day9::solve(&input)),
        10 => println!("{:?}", day10::solve(&input)),
        11 => println!("{:?}", day11::solve(&input)),
        _ => eprintln!("invalid day: {}", day),
    }
    Ok(())
}
