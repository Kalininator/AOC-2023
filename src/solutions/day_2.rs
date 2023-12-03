use sscanf::scanf;
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
    let id_sum = input
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .filter(|game| game.is_valid())
        .map(|game| game.game_number)
        .sum::<usize>();
    println!("Sum of valid games: {id_sum}");
}

fn part2(input: &str) {
    let mut total_power = 0;
    for line in input.lines() {
        let game = Game::from_str(line).unwrap();
        total_power += game.cube_power();
    }
    println!("Total power: {total_power}");
}

struct Game {
    game_number: usize,
    picks: Vec<Pick>,
}

impl Game {
    fn is_valid(&self) -> bool {
        for pick in &self.picks {
            for cube in &pick.cubes {
                let max = max_for_colour(&cube.colour);
                if cube.amount > max {
                    return false;
                }
            }
        }
        true
    }

    fn amount_of_colour_needed(&self, colour: Colour) -> usize {
        self.picks
            .iter()
            .map(|p| {
                p.cubes
                    .iter()
                    .filter(|c| c.colour == colour)
                    .map(|c| c.amount)
            })
            .flatten()
            .max()
            .unwrap()
    }

    fn cube_power(&self) -> usize {
        self.amount_of_colour_needed(Colour::Red)
            * self.amount_of_colour_needed(Colour::Green)
            * self.amount_of_colour_needed(Colour::Blue)
    }
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let g = scanf!(s, "Game {}:{}", usize, String).unwrap();
        let picks = g.1.split(";").map(|s| Pick::from_str(s).unwrap()).collect();
        Ok(Self {
            game_number: g.0,
            picks,
        })
    }
}

struct Pick {
    cubes: Vec<CubePick>,
}
impl FromStr for Pick {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s
            .split(",")
            .map(|s| scanf!(s, "{}", CubePick).unwrap())
            .collect();
        Ok(Self { cubes })
    }
}

#[derive(sscanf::FromSscanf)]
#[sscanf(format = " {amount} {colour}")]
struct CubePick {
    colour: Colour,
    amount: usize,
}

#[derive(PartialEq, sscanf::FromSscanf)]
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
