use sscanf::scanf;
use std::collections::HashMap;

pub struct Day8;
impl crate::Solution for Day8 {
    fn part1(&self, input: &str) {
        println!("{}", part1(input));
    }
    fn part2(&self, input: &str) {
        println!("{}", part2(input));
    }
}

fn part1(input: &str) -> usize {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let instructions = sections[0];
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for line in sections[1].lines() {
        let (node, left, right) = scanf!(line, "{String} = ({String}, {String})").unwrap();
        nodes.insert(node, (left, right));
    }

    let mut current_node = "AAA".to_string();
    let mut instuction_index: usize = 0;
    loop {
        let instruction = instructions
            .chars()
            .nth(instuction_index % instructions.len())
            .unwrap();
        let node = nodes.get(&current_node).unwrap();
        if instruction == 'L' {
            current_node = node.0.clone();
        } else if instruction == 'R' {
            current_node = node.1.clone();
        }
        if current_node == "ZZZ" {
            return instuction_index + 1;
        }
        instuction_index += 1;
    }
}

fn part2(input: &str) -> usize {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let instructions = sections[0];
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for line in sections[1].lines() {
        let (node, left, right) = scanf!(line, "{String} = ({String}, {String})").unwrap();
        nodes.insert(node, (left, right));
    }

    let mut starting_nodes: Vec<String> = vec![];
    for node in nodes.keys() {
        if node.ends_with('A') {
            starting_nodes.push(node.clone());
        }
    }

    let mut to_end_counts = vec![];
    for start in starting_nodes.iter() {
        let mut current_node_name = start.clone();
        let mut count = 0;
        let mut instruction_iter = instructions.chars().cycle();

        while !current_node_name.ends_with('Z') {
            let instruction = instruction_iter.next().unwrap();
            let node = nodes.get(&current_node_name).unwrap();
            if instruction == 'L' {
                current_node_name = node.0.clone();
            } else if instruction == 'R' {
                current_node_name = node.1.clone();
            }
            count += 1;
        }

        to_end_counts.push(count);
    }
    least_common_multiple(&to_end_counts)
}

fn least_common_multiple(nums: &[usize]) -> usize {
    let mut result = 1;
    for &num in nums {
        result = num * result / gcd(num, result);
    }
    result
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#
            ),
            2
        );
        assert_eq!(
            part1(
                r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#
            ),
            6
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#
            ),
            6
        )
    }
}
