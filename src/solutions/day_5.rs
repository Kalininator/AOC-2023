use crate::util::string_to_numbers;
use sscanf::scanf;
use std::str::FromStr;

pub struct Day5;
impl crate::Solution for Day5 {
    fn part1(&self, input: &str) {
        part1(input);
    }
    fn part2(&self, input: &str) {
        part2(input);
    }
}

fn part1(input: &str) {
    let mut sections: Vec<&str> = input.split("\n\n").collect();
    let seeds_line = sections.remove(0);

    let seeds = scanf!(seeds_line, "seeds: {}", String).unwrap();
    let mut seeds = string_to_numbers(&seeds);

    let map_sets = sections
        .iter()
        .map(|s| MapSet::from_str(s).unwrap())
        .collect::<Vec<MapSet>>();

    for set in map_sets {
        seeds = set.apply(&seeds);
    }
    println!("lowest seed: {}", seeds.iter().min().unwrap());
}

fn part2(input: &str) {
    let mut sections: Vec<&str> = input.split("\n\n").collect();
    let seeds_line = sections.remove(0);

    let seeds = scanf!(seeds_line, "seeds: {}", String).unwrap();
    let mut actual_seeds: Vec<usize> = vec![];
    string_to_numbers(&seeds).chunks(2).for_each(|c| {
        // println!("c: {:?}", c);
        // println!("c[0]: {}", c[0]);
        // println!("c[1]: {}", c[1]);
        for i in c[0]..=(c[0] + c[1]) {
            // println!("adding seed: {}", i);
            actual_seeds.push(i);
        }
    });
    // println!("after laoding in all seeds: {actual_seeds:?}");

    let map_sets = sections
        .iter()
        .map(|s| MapSet::from_str(s).unwrap())
        .collect::<Vec<MapSet>>();

    for set in map_sets {
        actual_seeds = set.apply(&actual_seeds);
        // println!("{seeds:?}");
    }
    println!("lowest seed: {}", actual_seeds.iter().min().unwrap());
}

#[derive(sscanf::FromSscanf, Debug)]
#[sscanf(format = "{destination_range_start} {source_range_start} {range_length}")]
struct Map {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

#[derive(Debug)]
struct MapSet {
    maps: Vec<Map>,
}

impl FromStr for MapSet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let maps = s
            .lines()
            .skip(1)
            .map(|line| scanf!(line, "{}", Map).unwrap())
            .collect::<Vec<Map>>();
        Ok(MapSet { maps })
    }
}

impl MapSet {
    fn apply(&self, seeds: &[usize]) -> Vec<usize> {
        seeds
            .iter()
            .map(|seed| {
                for map in &self.maps {
                    if *seed >= map.source_range_start
                        && *seed < map.source_range_start + map.range_length
                    {
                        return map.destination_range_start + (*seed - map.source_range_start);
                    }
                }
                *seed
            })
            .collect::<Vec<usize>>()
    }
}
