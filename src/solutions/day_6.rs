use crate::util::string_to_numbers;
use sscanf::scanf;
use std::str::FromStr;

pub struct Day6;
impl crate::Solution for Day6 {
    fn part1(&self, input: &str) {
        part1(input);
    }
    fn part2(&self, input: &str) {
        part2(input);
    }
}

fn part1(input: &str) {
    let line_1 = input.lines().nth(0).unwrap();
    let line_2 = input.lines().nth(1).unwrap();
    let times = string_to_numbers(&scanf!(line_1, "Time: {}", String).unwrap());
    let distances = string_to_numbers(&scanf!(line_2, "Distance: {}", String).unwrap());
    let mut result: usize = 1;
    for race_number in 0..times.len() {
        let time = times[race_number];
        let distance_to_beat = distances[race_number];
        let mut viable_acceleration_times: usize = 0;
        for accel_time in 1..(time - 1) {
            if distance_covered(accel_time, time) > distance_to_beat {
                // println!("accel_time: {}", accel_time);
                viable_acceleration_times += 1;
            }
        }
        println!("viable_acceleration_times: {}", viable_acceleration_times);
        result *= viable_acceleration_times;
    }
    println!("{}", result);
}

fn part2(input: &str) {
    let line_1 = remove_whitespace(input.lines().nth(0).unwrap());
    let line_2 = remove_whitespace(input.lines().nth(1).unwrap());
    println!("line_1: {}", line_1);
    println!("line_2: {}", line_2);
    let times = string_to_numbers(&scanf!(line_1, "Time:{}", String).unwrap());
    let distances = string_to_numbers(&scanf!(line_2, "Distance:{}", String).unwrap());
    let mut result: usize = 1;
    for race_number in 0..times.len() {
        let time = times[race_number];
        let distance_to_beat = distances[race_number];
        let mut viable_acceleration_times: usize = 0;
        for accel_time in 1..(time - 1) {
            // println!("Checking accel time {accel_time}/{}", time-1);
            if distance_covered(accel_time, time) > distance_to_beat {
                // println!("accel_time: {}", accel_time);
                viable_acceleration_times += 1;
            }
        }
        println!("viable_acceleration_times: {}", viable_acceleration_times);
        result *= viable_acceleration_times;
    }
    println!("{}", result);
}

fn distance_covered(time_accelerating: usize, time: usize) -> usize {
    time_accelerating * (time-time_accelerating)
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
