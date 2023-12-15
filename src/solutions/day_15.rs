use itertools::Itertools;
use sscanf::scanf;
use std::collections::HashMap;

pub struct Day15;
impl crate::Solution for Day15 {
    fn part1(&self, input: &str) {
        println!("{}", part1(input));
    }
    fn part2(&self, input: &str) {
        println!("{}", part2(input));
    }
}

fn part1(input: &str) -> usize {
    let mut input = input.to_string();
    input.retain(|c| !c.is_whitespace());
    input.split(',').map(hash_string).sum()
}

fn part2(input: &str) -> usize {
    let mut input = input.to_string();
    input.retain(|c| !c.is_whitespace());
    let operations = input
        .split(',')
        .map(|s| scanf!(s, "{}", Operation).unwrap())
        .collect::<Vec<Operation>>();
    let mut boxes: HashMap<usize, Vec<(String, usize)>> = HashMap::new();
    for o in operations {
        match o {
            Operation::Equals(s, n) => {
                let box_id = hash_string(&s);
                if let Some(existing_box) = boxes.get_mut(&box_id) {
                    if let Some(i) = existing_box.iter().position(|e| e.0 == s) {
                        existing_box[i].1 = n;
                    } else {
                        existing_box.push((s.clone(), n));
                    }
                } else {
                    boxes.insert(box_id, vec![(s.clone(), n)]);
                }
            }
            Operation::Dash(s) => {
                let box_id = hash_string(&s);
                if let Some(existing_box) = boxes.get_mut(&box_id) {
                    if let Some(i) = existing_box.iter().position(|e| e.0 == s) {
                        existing_box.remove(i);
                    }
                }
            }
        }
    }
    let boxes_vec = boxes
        .iter()
        .map(|(k, v)| {
            (
                *k,
                v.iter()
                    .map(|(s, n)| (s.clone(), *n))
                    .collect::<Vec<(String, usize)>>(),
            )
        })
        .sorted_by_key(|(k, _)| *k)
        .collect::<Vec<(usize, Vec<(String, usize)>)>>();

    let mut lens_focus_sum = 0;
    for (id, lenses) in boxes_vec.iter() {
        lenses.iter().enumerate().for_each(|(lens_index, lens)| {
            lens_focus_sum += (id + 1) * (1 + lens_index) * lens.1
        });
    }
    lens_focus_sum
}

fn hash_string(s: &str) -> usize {
    let mut current_hash = 0;
    for c in s.chars() {
        current_hash += c as usize;
        current_hash *= 17;
        current_hash %= 256;
    }
    current_hash
}

#[derive(PartialEq, sscanf::FromSscanf, Debug)]
enum Operation {
    #[sscanf(format = "{}={}")]
    Equals(String, usize),
    #[sscanf(format = "{}-")]
    Dash(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash_string() {
        assert_eq!(hash_string("rn=1"), 30);
        assert_eq!(hash_string("HASH"), 52);
    }
}
