use regex::Regex;

pub struct Day3;
impl crate::Solution for Day3 {
    fn part1(&self, input: &str) {
        // println!("{input}");
        part1(input);
    }
    fn part2(&self, input: &str) {}
}

fn part1(input: &str) {
    let regex_numbers = Regex::new(r"\d+").unwrap();
    let regex_symbols = Regex::new(r"[^0-9.\s]").unwrap();

    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    for (index, line) in input.lines().enumerate() {
        for result in regex_numbers.captures_iter(line) {
            let position = result.get(0).unwrap().start();
            let value = result.get(0).unwrap().as_str();
            numbers.push(Number {
                line: index as isize,
                position: position as isize,
                value: value.parse::<isize>().unwrap(),
                length: value.len() as isize,
            });
        }

        for result in regex_symbols.captures_iter(line) {
            let position = result.get(0).unwrap().start();
            symbols.push(Symbol {
                line: index as isize,
                position: position as isize,
                // value: result.get(0).unwrap().as_str().to_string(),
            });
        }
    }

    let mut part_numbers: Vec<Number> = vec![];
    for number in numbers.clone() {
        let mut is_part_number = false;
        for symbol in symbols.clone() {
            if symbol.line >= number.line - 1
                && symbol.line <= number.line + 1
                && symbol.position >= number.position - 1
                && symbol.position <= number.position + number.length
            {
                is_part_number = true;
            }
        }
        if is_part_number {
            part_numbers.push(number.clone());
        }
    }
    println!("{}", part_numbers.iter().map(|n| n.value).sum::<isize>());
}

#[derive(Debug, Clone)]
struct Number {
    line: isize,
    position: isize,
    value: isize,
    length: isize,
}

#[derive(Debug, Clone)]
struct Symbol {
    line: isize,
    position: isize,
    // value: String,
}
