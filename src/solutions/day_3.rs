use regex::Regex;

pub struct Day3;
impl crate::Solution for Day3 {
    fn part1(&self, input: &str) {
        part1(input);
    }
    fn part2(&self, input: &str) {
        part2(input);
    }
}

fn part1(input: &str) {
    let numbers = get_numbers(input);
    let symbols = get_symbols(input);

    let mut part_numbers: Vec<Number> = vec![];
    for number in numbers.clone() {
        let mut is_part_number = false;
        for symbol in symbols.clone() {
            if number.is_adjacent_to_symbol(&symbol) {
                is_part_number = true;
            }
        }
        if is_part_number {
            part_numbers.push(number.clone());
        }
    }
    println!("{}", part_numbers.iter().map(|n| n.value).sum::<isize>());
}

fn part2(input: &str) {
    let symbols = get_symbols(input);
    let symbols = symbols
        .into_iter()
        .filter(|s| s.value == "*")
        .collect::<Vec<Symbol>>();
    let numbers = get_numbers(input);
    let mut gear_ratios = vec![];
    for s in symbols {
        let adjacent_numbers = numbers
            .clone()
            .into_iter()
            .filter(|n| n.is_adjacent_to_symbol(&s))
            .collect::<Vec<Number>>();
        if adjacent_numbers.len() == 2 {
            gear_ratios.push(adjacent_numbers[0].value * adjacent_numbers[1].value);
        }
    }
    println!("{}", gear_ratios.iter().sum::<isize>());
}

fn get_numbers(input: &str) -> Vec<Number> {
    let regex = Regex::new(r"\d+").unwrap();
    let mut numbers: Vec<Number> = vec![];
    for (index, line) in input.lines().enumerate() {
        for result in regex.captures_iter(line) {
            let position = result.get(0).unwrap().start();
            let value = result.get(0).unwrap().as_str();
            numbers.push(Number {
                line: index as isize,
                position: position as isize,
                value: value.parse::<isize>().unwrap(),
                length: value.len() as isize,
            });
        }
    }
    numbers
}

fn get_symbols(input: &str) -> Vec<Symbol> {
    let regex = Regex::new(r"[^0-9.\s]").unwrap();
    let mut symbols: Vec<Symbol> = vec![];
    for (index, line) in input.lines().enumerate() {
        for result in regex.captures_iter(line) {
            let position = result.get(0).unwrap().start();
            symbols.push(Symbol {
                line: index as isize,
                position: position as isize,
                value: result.get(0).unwrap().as_str().to_string(),
            });
        }
    }
    symbols
}

#[derive(Debug, Clone)]
struct Number {
    line: isize,
    position: isize,
    value: isize,
    length: isize,
}

impl Number {
    fn is_adjacent_to_symbol(&self, symbol: &Symbol) -> bool {
        symbol.line >= self.line - 1
            && symbol.line <= self.line + 1
            && symbol.position >= self.position - 1
            && symbol.position <= self.position + self.length
    }
}

#[derive(Debug, Clone)]
struct Symbol {
    line: isize,
    position: isize,
    value: String,
}
