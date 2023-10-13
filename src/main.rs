use std::fs;
use aoc_2023::solution::Solution;
use aoc_2023::solutions;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which day to run solution for
    #[arg(short, long, default_value_t = 1)]
    day: u8,

    /// Filename of input
    #[arg(short, long)]
    file: String,
}


fn main() {
    let Args { day, file } = Args::parse();

    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(solutions::Day1),
    ];

    println!("Getting solution for day {day} with file {file}",);

    let file_contents = fs::read_to_string(&file).expect("Unable to read file");
    let solution = solutions.get(day as usize - 1).unwrap();
    solution.part1(&file_contents);
    solution.part2(&file_contents);
}
