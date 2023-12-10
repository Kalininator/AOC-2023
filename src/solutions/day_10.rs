use sscanf::scanf;
pub struct Day10;
impl crate::Solution for Day10 {
    fn part1(&self, input: &str) {
        println!("{}", part1(input));
    }
    fn part2(&self, input: &str) {
        println!("{}", part2(input));
    }
}

fn part1(input: &str) -> usize {
    let grid = parse_grid(input);
    let mut previous = find_start(&grid);
    let start = previous.clone();
    let mut current = find_next_from_start(&grid, previous);
    let mut moves = 1;
    while current != start {
        let temp = next_position(previous, current, grid[current.0][current.1]);
        previous = current;
        current = temp;
        moves += 1;
    }
    moves / 2
}

fn part2(input: &str) -> usize {
    let grid = parse_grid(input);

    let mut main_pipe_locations: Vec<(usize, usize)> = vec![];

    let mut previous = find_start(&grid);
    let start = previous.clone();
    main_pipe_locations.push(previous);
    let mut current = find_next_from_start(&grid, previous);
    while current != start {
        main_pipe_locations.push(current);
        let temp = next_position(previous, current, grid[current.0][current.1]);
        previous = current;
        current = temp;
    }

    let mut all_hiding_spots: Vec<(usize, usize)> = vec![];
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            if !main_pipe_locations.contains(&(row, column))
            {
                all_hiding_spots.push((row, column));
            }
        }
    }

    let mut inside_boundary: Vec<(usize, usize)> = vec![];
    for t in all_hiding_spots {
        let mut crossings = 0;
        for column in 0..t.1 {
            let tile = grid[t.0][column];
            if main_pipe_locations.contains(&(t.0, column))
                && (tile == Tile::Vertical || tile == Tile::BendNE || tile == Tile::BendNW)
            {
                crossings += 1;
            }
        }
        if crossings % 2 == 1 {
            inside_boundary.push(t);
        }
    }
    println!("{:?}", inside_boundary);
    inside_boundary.len()
}

fn parse_grid(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    let str = c.to_string();
                    let tile = scanf!(str, "{}", Tile).unwrap();
                    tile
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>()
}

fn find_start(grid: &Vec<Vec<Tile>>) -> (usize, usize) {
    for (row, row_vec) in grid.iter().enumerate() {
        for (col, tile) in row_vec.iter().enumerate() {
            if *tile == Tile::Start {
                return (row, col);
            }
        }
    }
    unreachable!()
}

fn find_next_from_start(grid: &Vec<Vec<Tile>>, start: (usize, usize)) -> (usize, usize) {
    let above = grid[start.0 - 1][start.1];
    if above == Tile::Vertical || above == Tile::BendSE || above == Tile::BendSW {
        return (start.0 - 1, start.1);
    }
    let below = grid[start.0 + 1][start.1];
    if below == Tile::Vertical || below == Tile::BendNE || below == Tile::BendNW {
        return (start.0 + 1, start.1);
    }
    let left = grid[start.0][start.1 - 1];
    if left == Tile::Horizontal || left == Tile::BendNE || left == Tile::BendSE {
        return (start.0, start.1 - 1);
    }
    unreachable!()
}

#[derive(PartialEq, sscanf::FromSscanf, Debug, Copy, Clone)]
enum Tile {
    #[sscanf(format = "|")]
    Vertical,
    #[sscanf(format = "-")]
    Horizontal,
    #[sscanf(format = "L")]
    BendNE,
    #[sscanf(format = "J")]
    BendNW,
    #[sscanf(format = "7")]
    BendSW,
    #[sscanf(format = "F")]
    BendSE,
    #[sscanf(format = ".")]
    Ground,
    #[sscanf(format = "S")]
    Start,
}

fn next_position(
    previous_position: (usize, usize),
    current_position: (usize, usize),
    tile: Tile,
) -> (usize, usize) {
    if previous_position.0 < current_position.0 {
        // Coming from North
        if tile == Tile::Vertical {
            return (current_position.0 + 1, current_position.1);
        } else if tile == Tile::BendNW {
            return (current_position.0, current_position.1 - 1);
        } else if tile == Tile::BendNE {
            return (current_position.0, current_position.1 + 1);
        }
    } else if previous_position.0 > current_position.0 {
        // Coming from South
        if tile == Tile::Vertical {
            return (current_position.0 - 1, current_position.1);
        } else if tile == Tile::BendSW {
            return (current_position.0, current_position.1 - 1);
        } else if tile == Tile::BendSE {
            return (current_position.0, current_position.1 + 1);
        }
    } else if previous_position.1 < current_position.1 {
        // Coming from West
        if tile == Tile::Horizontal {
            return (current_position.0, current_position.1 + 1);
        } else if tile == Tile::BendSW {
            return (current_position.0 + 1, current_position.1);
        } else if tile == Tile::BendNW {
            return (current_position.0 - 1, current_position.1);
        }
    } else if previous_position.1 > current_position.1 {
        // Coming from East
        if tile == Tile::Horizontal {
            return (current_position.0, current_position.1 - 1);
        } else if tile == Tile::BendSE {
            return (current_position.0 + 1, current_position.1);
        } else if tile == Tile::BendNE {
            return (current_position.0 - 1, current_position.1);
        }
    }
    println!("Previous: {:?}", previous_position);
    println!("Current: {:?}", current_position);
    println!("Tile: {:?}", tile);
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                r#".....
.S-7.
.|.|.
.L-J.
....."#
            ),
            8
        );
    }
}
