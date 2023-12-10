use regex::Regex;

pub fn string_to_numbers(s: &str) -> Vec<usize> {
    let regex = Regex::new(r"\d+").unwrap();
    regex
        .find_iter(s)
        .filter_map(|number| number.as_str().parse().ok())
        .collect()
}

pub fn string_to_numbers_i(s: &str) -> Vec<isize> {
    let regex = Regex::new(r"-*\d+").unwrap();
    regex
        .find_iter(s)
        .filter_map(|number| number.as_str().parse().ok())
        .collect()
}
