use crate::util::string_to_numbers_i;

pub struct Day9;
impl crate::Solution for Day9 {
    fn part1(&self, input: &str) {
        println!("{}", part1(input));
    }
    fn part2(&self, input: &str) {
        println!("{}", part2(input));
    }
}

fn part1(input: &str) -> isize {
    let mut sum = 0;
    for line in input.lines() {
        let mut history: Vec<Vec<isize>> = vec![string_to_numbers_i(line)];
        'inner: loop {
            let new_entry = history
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<isize>>();
            history.push(new_entry.clone());
            if new_entry.iter().all(|n| *n == 0) {
                break 'inner;
            }
        }
        let mut foo = 0;
        for row in (1..history.len()).rev() {
            println!("{:?}", history[row]);
            foo = history[row - 1].last().unwrap() + foo;
        }
        println!("{:?}", history[0]);
        sum += foo;
    }
    sum
}

fn part2(input: &str) -> isize {
    let mut sum = 0;
    for line in input.lines() {
        let mut history: Vec<Vec<isize>> = vec![string_to_numbers_i(line)];
        'inner: loop {
            let new_entry = history
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<isize>>();
            history.push(new_entry.clone());
            if new_entry.len() == 0 {
                break 'inner;
            }
        }
        let mut foo = 0;
        for row in (1..history.len()).rev() {
            println!("{:?}", history[row]);
            foo = history[row - 1].first().unwrap() - foo;
        }
        println!("{:?}", history[0]);
        sum += foo;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#
            ),
            114
        );
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#
            ),
            2
        );
    }
}
