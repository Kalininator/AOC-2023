// use sscanf::scanf;
pub struct Day11;
impl crate::Solution for Day11 {
    fn part1(&self, input: &str) {
        println!("{}", part1(input));
    }
    fn part2(&self, input: &str) {
        println!("{}", part2(input));
    }
}

fn part1(_input: &str) -> usize {
    0
}

fn part2(_input: &str) -> usize {
    0
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_part1() {
//         assert_eq!(
//             part1(
//                 r#".....
// .S-7.
// .|.|.
// .L-J.
// ....."#
//             ),
//             4
//         );
//     }
//
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
// }
