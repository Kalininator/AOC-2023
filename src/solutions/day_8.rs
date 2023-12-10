use std::collections::HashMap;

use sscanf::scanf;

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
        let instruction = instructions.chars().nth(instuction_index % instructions.len()).unwrap();
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
    0
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
}
