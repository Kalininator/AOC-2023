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
        let mut interpolated_value = 0;
        for row in (1..history.len()).rev() {
            println!("{:?}", history[row]);
            interpolated_value += history[row - 1].last().unwrap();
        }
        println!("{:?}", history[0]);
        sum += interpolated_value;
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
            if new_entry.is_empty() {
                break 'inner;
            }
        }
        let mut interpolated_value = 0;
        for row in (1..history.len()).rev() {
            println!("{:?}", history[row]);
            interpolated_value = history[row - 1].first().unwrap() - interpolated_value;
        }
        println!("{:?}", history[0]);
        sum += interpolated_value;
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
