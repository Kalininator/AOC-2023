use aoc_2023::solution::Solution;
use aoc_2023::solutions;
use chrono::prelude::*;
use clap::Parser;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which day to run solution for. Defaults to current day of the month
    #[arg(short, long)]
    day: Option<u32>,

    /// Filename of input
    #[arg(short, long)]
    file: String,
}

fn get_day(input: Option<u32>) -> u32 {
    match input {
        Some(day) => day,
        None => {
            let local: DateTime<Local> = Local::now();
            local.day()
        }
    }
}

fn main() {
    let Args { day, file } = Args::parse();

    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(solutions::Day1),
        Box::new(solutions::Day2),
        Box::new(solutions::Day3),
        Box::new(solutions::Day4),
        Box::new(solutions::Day5),
        Box::new(solutions::Day6),
        Box::new(solutions::Day7),
        Box::new(solutions::Day8),
        Box::new(solutions::Day9),
        Box::new(solutions::Day10),
        Box::new(solutions::Day11),
        Box::new(solutions::Day11),
        Box::new(solutions::Day11),
        Box::new(solutions::Day11),
        Box::new(solutions::Day15),
    ];

    let day = get_day(day);

    println!("Getting solution for day {day} with file {file}",);

    let file_contents = fs::read_to_string(&file).expect("Unable to read file");
    let solution = solutions.get(day as usize - 1).unwrap_or_else(|| panic!("No solution for day {day}. Only have days 1-{}",
            solutions.len()));
    println!("Day {day} part 1");
    solution.part1(&file_contents);
    println!("Day {day} part 2");
    solution.part2(&file_contents);
}
