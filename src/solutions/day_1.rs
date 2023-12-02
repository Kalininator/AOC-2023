pub struct Day1;
impl crate::Solution for Day1 {
    fn part1(&self, input: &str) {
        let lines: Vec<&str> = input.lines().collect();
        let mut sum = 0;
        for line in lines {
            let number = extract_two_digit_number(line);
            sum += number;
        }
        println!("{sum}");
    }
    fn part2(&self, input: &str) {
        let lines: Vec<&str> = input.lines().collect();
        let mut sum: u32 = 0;
        for line in lines {
            let number = extract_two_digit_number_enhanced(line);
            sum += number;
        }
        println!("{sum}");
    }
}

fn extract_two_digit_number(input: &str) -> u32 {
    let first_digit: char = input.chars().find(|c| c.is_digit(10)).unwrap();
    let second_digit: char = input.chars().rev().find(|c| c.is_digit(10)).unwrap();
    return format!("{}{}", first_digit, second_digit)
        .parse::<u32>()
        .unwrap_or(0);
}

fn extract_two_digit_number_enhanced(input: &str) -> u32 {
    let mut first_digit: char = ' ';
    let mut found_letters = vec![];
    for char in input.chars() {
        if char.is_digit(10) {
            first_digit = char;
            break;
        } else {
            found_letters.push(char);
            if let Some(number) = chars_to_digit(&found_letters, false) {
                first_digit = std::char::from_digit(number as u32, 10).unwrap();
                break;
            }
        }
    }

    let mut second_digit: char = ' ';
    found_letters = vec![];

    for char in input.chars().rev() {
        if char.is_digit(10) {
            second_digit = char;
            break;
        } else {
            found_letters.push(char);
            if let Some(number) = chars_to_digit(&found_letters, true) {
                second_digit = std::char::from_digit(number as u32, 10).unwrap();
                break;
            }
        }
    }
    return format!("{}{}", first_digit, second_digit)
        .parse::<u32>()
        .unwrap();
}

fn chars_to_digit(input: &Vec<char>, reverse: bool) -> Option<u32> {
    let numbers_as_strings = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let string = if reverse {
        input.iter().rev().collect::<String>()
    } else {
        input.iter().collect::<String>()
    };
    for (index, number) in numbers_as_strings.iter().enumerate() {
        if string.contains(number) {
            return Some(index as u32);
        }
    }
    None
}
