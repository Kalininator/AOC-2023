use regex::Regex;
use sscanf::scanf;
use std::str::FromStr;

pub struct Day4;
impl crate::Solution for Day4 {
    fn part1(&self, input: &str) {
        part1(input);
    }
    fn part2(&self, input: &str) {
        part2(input);
    }
}

fn part1(input: &str) {
    let mut total_score: u32 = 0;
    for line in input.lines() {
        let c = scanf!(line, "{}: {} | {}", String, String, String).unwrap();
        let winning_numbers = string_to_numbers(&c.1);
        let my_numbers = string_to_numbers(&c.2);
        let num_winners = my_numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .count();
        if num_winners > 0 {
            let base: u32 = 2;
            total_score += base.pow(num_winners as u32 - 1);
        }
    }
    println!("{}", total_score);
}

fn part2(input: &str) {}

fn string_to_numbers(s: &str) -> Vec<usize> {
    let regex = Regex::new(r"\d+").unwrap();
    regex
        .find_iter(s)
        .filter_map(|number| number.as_str().parse().ok())
        .collect()
}
