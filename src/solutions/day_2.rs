use sscanf::scanf;
use std::cmp;
use std::str::FromStr;

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
        let game = Game::from_str(line).unwrap();
        let mut game_valid = true;
        for pick in game.picks {
            for cp in pick.cubes {
                let max = max_for_colour(&cp.colour);
                if cp.amount > max {
                    game_valid = false;
                }
            }
        }
        if game_valid {
            id_sum += game.game_number;
        }
    }
    println!("Sum of valid games: {id_sum}");
}

fn part2(input: &str) {
    let mut total_power = 0;
    for line in input.lines() {
        let game = Game::from_str(line).unwrap();
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for pick in game.picks {
            for cp in pick.cubes {
                match cp.colour {
                    Colour::Red => min_red = cmp::max(min_red, cp.amount),
                    Colour::Green => min_green = cmp::max(min_green, cp.amount),
                    Colour::Blue => min_blue = cmp::max(min_blue, cp.amount),
                }
            }
        }
        let power = min_red * min_green * min_blue;
        total_power += power;
    }
    println!("Total power: {total_power}");
}

struct Game {
    game_number: usize,
    picks: Vec<Pick>,
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let g = scanf!(s, "Game {}:{}", usize, String).unwrap();
        let picks =
            g.1.split(";")
                .map(|s| Pick {
                    cubes: s
                        .split(",")
                        .map(|s| scanf!(s, "{}", CubePick).unwrap())
                        .collect(),
                })
                .collect();
        Ok(Self {
            game_number: g.0,
            picks,
        })
    }
}

struct Pick {
    cubes: Vec<CubePick>,
}

#[derive(sscanf::FromSscanf)]
#[sscanf(format = " {amount} {colour}")]
struct CubePick {
    colour: Colour,
    amount: usize,
}

#[derive(sscanf::FromSscanf)]
enum Colour {
    #[sscanf(format = "red")]
    Red,
    #[sscanf(format = "green")]
    Green,
    #[sscanf(format = "blue")]
    Blue,
}

fn max_for_colour(colour: &Colour) -> usize {
    match colour {
        Colour::Red => 12,
        Colour::Green => 13,
        Colour::Blue => 14,
    }
}
