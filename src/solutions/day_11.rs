// use sscanf::scanf;

use itertools::Itertools;
pub struct Day11;
impl crate::Solution for Day11 {
    fn part1(&self, input: &str) {
        println!("{}", part1(input));
    }
    fn part2(&self, input: &str) {
        println!("{}", part2(input));
    }
}

fn part1(input: &str) -> usize {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let hash_locations =
        chars
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc: Vec<(usize, usize)>, (y, line)| {
                acc.extend(
                    line.iter()
                        .enumerate()
                        .filter(|(_, c)| **c == '#')
                        .map(|(x, _)| (y, x)),
                );
                acc
            });
    println!("{:?}", hash_locations);
    let rows_without_hashes =
        chars
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc: Vec<usize>, (y, line)| {
                if line.iter().filter(|c| **c == '#').count() == 0 {
                    acc.push(y);
                }
                acc
            });
    println!("{:?}", rows_without_hashes);
    let mut columns_without_hashes = Vec::new();
    for column in 0..chars[0].len() {
        let mut has_hash = false;
        for row in 0..chars.len() {
            if chars[row][column] == '#' {
                has_hash = true;
            }
        }
        if !has_hash {
            columns_without_hashes.push(column);
        }
    }
    println!("{:?}", columns_without_hashes);

    let mut total_distances = 0;
    for combination in hash_locations.into_iter().combinations(2) {
        println!("A: {:?}, B: {:?}", combination[0], combination[1]);
        let horizontal_distance =
            columns_between_hashes(combination[0], combination[1], &columns_without_hashes, 2);
        let vertical_distance =
            rows_between_hashes(combination[0], combination[1], &rows_without_hashes, 2);
        println!("H: {}, V: {}", horizontal_distance, vertical_distance);
        println!("Total: {}", horizontal_distance + vertical_distance);
        total_distances += horizontal_distance + vertical_distance;
    }
    total_distances
}

fn part2(input: &str) -> usize {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let hash_locations =
        chars
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc: Vec<(usize, usize)>, (y, line)| {
                acc.extend(
                    line.iter()
                        .enumerate()
                        .filter(|(_, c)| **c == '#')
                        .map(|(x, _)| (y, x)),
                );
                acc
            });
    println!("{:?}", hash_locations);
    let rows_without_hashes =
        chars
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc: Vec<usize>, (y, line)| {
                if line.iter().filter(|c| **c == '#').count() == 0 {
                    acc.push(y);
                }
                acc
            });
    println!("{:?}", rows_without_hashes);
    let mut columns_without_hashes = Vec::new();
    for column in 0..chars[0].len() {
        let mut has_hash = false;
        for row in 0..chars.len() {
            if chars[row][column] == '#' {
                has_hash = true;
            }
        }
        if !has_hash {
            columns_without_hashes.push(column);
        }
    }
    println!("{:?}", columns_without_hashes);

    let mut total_distances = 0;
    for combination in hash_locations.into_iter().combinations(2) {
        println!("A: {:?}, B: {:?}", combination[0], combination[1]);
        let horizontal_distance =
            columns_between_hashes(combination[0], combination[1], &columns_without_hashes, 1000000);
        let vertical_distance =
            rows_between_hashes(combination[0], combination[1], &rows_without_hashes, 1000000);
        println!("H: {}, V: {}", horizontal_distance, vertical_distance);
        println!("Total: {}", horizontal_distance + vertical_distance);
        total_distances += horizontal_distance + vertical_distance;
    }
    total_distances
}

fn columns_between_hashes(
    a: (usize, usize),
    b: (usize, usize),
    columns_with_hashes: &[usize],
    actual_distance: usize,
) -> usize {
    if a.1 > b.1 {
        columns_between_hashes(b, a, columns_with_hashes, actual_distance)
    } else {
        let amount = b.1 - a.1;
        let mut new_amount = amount;
        for col in a.1..b.1 {
            if columns_with_hashes.contains(&col) {
                new_amount += actual_distance - 1;
            }
        }
        new_amount
    }
}

fn rows_between_hashes(a: (usize, usize), b: (usize, usize), rows_with_hashes: &[usize], actual_distance: usize) -> usize {
    if a.0 > b.0 {
        rows_between_hashes(b, a, rows_with_hashes, actual_distance)
    } else {
        let amount = b.0 - a.0;
        let mut new_amount = amount;
        for row in a.0..b.0 {
            if rows_with_hashes.contains(&row) {
                new_amount += actual_distance - 1;
            }
        }
        new_amount
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#
            ),
            374
        );
    }

    //     #[test]
    //     fn test_part2() {
    //         assert_eq!(
    //             part2(
    //                 r#"...........
    // .S-------7.
    // .|F-----7|.
    // .||.....||.
    // .||.....||.
    // .|L-7.F-J|.
    // .|..|.|..|.
    // .L--J.L--J.
    // ..........."#
    //             ),
    //             4
    //         )
    //     }
}
