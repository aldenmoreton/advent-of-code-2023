use std::collections::HashSet;
use rayon::prelude::*;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn add_direction(direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1)
    }
}

#[aoc_generator(day18, part1)]
fn input_generator(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let direction = match parts.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Shouldn't go this direction")
            };
            let distance: i32 = parts.next().unwrap().parse().unwrap();

            (direction, distance)
        })
        .collect()
}

#[aoc(day18, part1)]
fn part_one(input: &[(Direction, i32)]) -> usize {
    let (mut min_x, mut min_y) = (i32::MAX, i32::MAX);
    let (mut max_x, mut max_y) = (i32::MIN, i32::MIN);
    let mut map = HashSet::new();

    let mut curr_location = (0, 0);
    map.insert(curr_location);
    for (direction, steps) in input {
        let adder = add_direction(direction);
        for _ in 0..*steps {
            curr_location = (curr_location.0+adder.0, curr_location.1+adder.1);
            map.insert(curr_location);


            min_x = std::cmp::min(min_x, curr_location.0);
            min_y = std::cmp::min(min_y, curr_location.1);


            max_x = std::cmp::max(max_x, curr_location.0);
            max_y = std::cmp::max(max_y, curr_location.1);
        }
    }

    (min_x..=max_x)
        .flat_map(|i|
            (min_y..=max_y)
                .map(move |j| (i, j))
        )
        .par_bridge()
        .map(|(i, j)| {
            if map.contains(&(i.to_owned(), j)) {
                1
            } else {
                let mut cursor = (i-1, j);
                let mut inside_tracker = false;

                while cursor.0 >= min_x {
                    if map.contains(&cursor) {
                        let left_neighbor = map.get(&(cursor.0, cursor.1-1));
                        let right_neighbor = map.get(&(cursor.0, cursor.1+1));
                        let start_y = match (left_neighbor, right_neighbor) {
                            (Some(_), Some(_)) => {
                                inside_tracker = !inside_tracker;
                                cursor.0 -= 1;
                                continue
                            },
                            (Some((_, y)), None) => y,
                            (None, Some((_, y))) => y,
                            (None, None) => panic!("A wall should not be by itself")
                        };

                        cursor.0 -= 1;
                        let end_y = loop {
                            let left_neighbor = map.get(&(cursor.0, cursor.1-1));
                            let right_neighbor = map.get(&(cursor.0, cursor.1+1));
                            match (left_neighbor, right_neighbor) {
                                (Some(_), Some(_)) => panic!("Shouldn't find a horizontal wall"),
                                (Some((_, y)), None) => break y,
                                (None, Some((_, y))) => break y,
                                (None, None) => ()
                            };
                            cursor.0 -= 1;
                        };
                        if start_y != end_y {
                            inside_tracker = !inside_tracker
                        }
                    }
                    cursor.0 -= 1;
                }

                if inside_tracker {
                    1
                } else {
                    0
                }
            }
        })
        .sum()
}

#[aoc(day18, part1, Optimized)]
fn part_one_optimized(input: &[(Direction, i32)]) -> i32 {
    let (result, _, _) = input
        .iter()
        .fold((0, 0, 0), |(mut a, mut r, mut c), (direction, distance)| {
            let (rr, cc) = (r,c);
            match direction {
                Direction::Up => r -= distance,
                Direction::Right => c += distance,
                Direction::Down => r += distance,
                Direction::Left => c -= distance
            };
            a += (c + cc) * (r - rr) + distance;
            (a, r, c)
        });
    result / 2 + 1
}

#[aoc_generator(day18, part2)]
fn input_generator_two(input: &str) -> Vec<(Direction, i64)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let full_hex = parts.nth(2).unwrap();

            let distance = i64::from_str_radix(&full_hex[2..7], 16).unwrap();
            let direction_num = i64::from_str_radix(&full_hex[7..8], 16).unwrap();
            let direction = match direction_num {
                0 => Direction::Right,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Up,
                _ => panic!("Not a valid direction")
            };

            (direction, distance)
        })
        .collect()
}

#[aoc(day18, part2)]
fn part_two(input: &[(Direction, i64)]) -> i64 {
    let (result, _, _) = input
        .iter()
        .fold((0, 0, 0), |(mut a, mut r, mut c), (direction, distance)| {
            let (rr, cc) = (r,c);
            match direction {
                Direction::Up => r -= distance,
                Direction::Right => c += distance,
                Direction::Down => r += distance,
                Direction::Left => c -= distance
            };
            a += (c + cc) * (r - rr) + distance;
            (a, r, c)
        });
    result / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_1() {
        let input = indoc!{"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        "};
        let result = part_one(&input_generator(input));
        assert_eq!(result, 62);
    }

    #[test]
    fn part2_1() {
        let input = indoc!{"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        "};
        let result = part_two(&input_generator_two(input));
        assert_eq!(result, 952_408_144_115);
    }
}
