use crate::util::string_to_numbers;
use sscanf::scanf;

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
        let num_winners = count_card_winning_numbers(line);
        if num_winners > 0 {
            let base: u32 = 2;
            total_score += base.pow(num_winners as u32 - 1);
        }
    }
    println!("{}", total_score);
}

fn part2(input: &str) {
    let card_winning_count = input
        .lines()
        .map(count_card_winning_numbers)
        .collect::<Vec<usize>>();
    let mut card_qty = vec![1; card_winning_count.len()];
    for card in 0..card_qty.len() {
        let winners = card_winning_count[card];
        if winners > 0 {
            for i in (card + 1)..(card + winners + 1) {
                card_qty[i] += card_qty[card];
            }
        }
    }
    println!("{}", card_qty.iter().sum::<usize>());
}

fn count_card_winning_numbers(line: &str) -> usize {
    let c = scanf!(line, "{}: {} | {}", String, String, String).unwrap();
    let winning_numbers = string_to_numbers(&c.1);
    let my_numbers = string_to_numbers(&c.2);
    my_numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count()
}
