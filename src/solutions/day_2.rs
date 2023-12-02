use std::cmp;
use sscanf::scanf;

pub struct Day2;
impl crate::Solution for Day2 {
    fn part1(&self, input: &str) {
        part1(input);
    }
    fn part2(&self, input: &str) {
        part2(input);
    }
}

fn part1(input: &str) {
    let mut id_sum = 0;
    for line in input.lines() {
        let game = scanf!(line, "Game {}:{}", usize, String).unwrap();
        let game_number = game.0;
        let subsets = game.1.split(";");
        let mut game_valid = true;
        for (index, subset) in subsets.enumerate() {
            let picks = subset.split(",");
            for pick in picks {
                let pick = pick.trim();
                let pick = scanf!(pick, "{} {}", usize, String).unwrap();
                let amount = pick.0;
                let colour = pick.1;
                if colour == "red" && amount > 12 {
                    game_valid = false;
                }
                if colour == "green" && amount > 13 {
                    game_valid = false;
                }
                if colour == "blue" && amount > 14 {
                    game_valid = false;
                }
            }
        }
        if game_valid {
            id_sum += game_number;
        }
    }
    println!("Sum of valid games: {id_sum}");
}

fn part2(input: &str) {
    let mut total_power = 0;
    for line in input.lines() {
        let game = scanf!(line, "Game {}:{}", usize, String).unwrap();
        let game_number = game.0;
        let subsets = game.1.split(";");
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for (index, subset) in subsets.enumerate() {
            let picks = subset.split(",");
            for pick in picks {
                let pick = pick.trim();
                let pick = scanf!(pick, "{} {}", usize, String).unwrap();
                let amount = pick.0;
                let colour = pick.1;
                if colour == "red" {
                    min_red = cmp::max(min_red, amount);
                }
                if colour == "green" {
                    min_green = cmp::max(min_green, amount);
                }
                if colour == "blue" {
                    min_blue = cmp::max(min_blue, amount);
                }
            }
        }
        let power = min_red * min_green * min_blue;
        total_power += power;
    }
    println!("Total power: {total_power}");
}
