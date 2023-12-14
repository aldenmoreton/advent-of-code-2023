use core::fmt::Debug;

type Map = Vec<Vec<GroundType>>;

#[derive(Clone, PartialEq)]
enum GroundType {
    Round,
    Cube,
    Empty
}

impl Debug for GroundType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroundType::Round => write!(f, "O"),
            GroundType::Cube => write!(f, "#"),
            GroundType::Empty => write!(f, ".")
        }
    }
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Map {
    input
        .lines()
        .map(|line|
            line
                .chars()
                .map(|character|
                    match character {
                        'O' => GroundType::Round,
                        '#' => GroundType::Cube,
                        '.' => GroundType::Empty,
                        _ => panic!("Should not have this input char")
                    }
                )
                .collect()
        )
        .collect()
}

#[aoc(day14, part1)]
fn part_one(input: &Map) -> usize {
    let col_len = input[0].len();

    (0..col_len)
        .map(|j| {
            let mut col_sum = 0;
            let mut max_location = 0;
            for (i, row) in input.iter().enumerate() {
                let curr_spot = &row[j];
                if *curr_spot == GroundType::Round {
                    col_sum += col_len - max_location;
                    max_location += 1;
                } else if *curr_spot == GroundType::Cube {
                    max_location = i + 1
                }
            }
            col_sum
        })
        .sum()
}

fn tilt_north(mut map: Map) -> Map {
    let (row_len, col_len) = (map.len(), map[0].len());

    (0..col_len)
        .for_each(|j| {
            let mut max_location = 0;
            for i in 0..row_len {
                let curr_spot = &map[i][j];
                if *curr_spot == GroundType::Round {
                    map[i][j] = GroundType::Empty;
                    map[max_location][j] = GroundType::Round;
                    max_location += 1;
                } else if *curr_spot == GroundType::Cube {
                    max_location = i + 1
                }
            }
        });

    map
}


fn tilt_south(mut map: Map) -> Map {
    let (row_len, col_len) = (map.len(), map[0].len());

    (0..col_len)
        .for_each(|j| {
            let mut max_location = col_len - 1;
            for i in (0..row_len).rev() {
                let curr_spot = &map[i][j];
                if *curr_spot == GroundType::Round {
                    map[i][j] = GroundType::Empty;
                    map[max_location][j] = GroundType::Round;
                    max_location = max_location.checked_sub(1).unwrap_or_default()
                } else if *curr_spot == GroundType::Cube {
                    max_location = i.checked_sub(1).unwrap_or_default()
                }
            }
        });

    map
}


fn tilt_east(mut map: Map) -> Map {
    let (row_len, col_len) = (map.len(), map[0].len());

    (0..row_len)
        .for_each(|i| {
            let mut max_location = row_len - 1;
            for j in (0..col_len).rev() {
                let curr_spot = &map[i][j];
                if *curr_spot == GroundType::Round {
                    map[i][j] = GroundType::Empty;
                    map[i][max_location] = GroundType::Round;
                    max_location = max_location.checked_sub(1).unwrap_or_default()
                } else if *curr_spot == GroundType::Cube {
                    max_location = j.checked_sub(1).unwrap_or_default()
                }
            }
        });

    map
}

fn tilt_west(mut map: Map) -> Map {
    let (row_len, col_len) = (map.len(), map[0].len());

    (0..row_len)
        .for_each(|i| {
            let mut max_location = 0;
            for j in 0..col_len {
                let curr_spot = &map[i][j];
                if *curr_spot == GroundType::Round {
                    map[i][j] = GroundType::Empty;
                    map[i][max_location] = GroundType::Round;
                    max_location += 1
                } else if *curr_spot == GroundType::Cube {
                    max_location = j + 1
                }
            }
        });

    map
}

fn cycle(map: Map) -> Map {
    tilt_east(
        tilt_south(
            tilt_west(
                tilt_north(map)
            )
        )
    )
}

fn get_load_on_support(map: &Map) -> usize {
    let mut total_load = 0;
    for (vertical_pos, line) in map.iter().enumerate() {
        for (_, curr_spot) in line.iter().enumerate() {
            if *curr_spot == GroundType::Round {
                total_load += map.len() - vertical_pos;
            }
        }
    }

    total_load
}

#[aoc(day14, part2)]
fn part_two(input: &Map) -> usize {
    let mut map = input.clone();
    let mut seen_states: Vec<Map> = vec![input.clone()];

    loop {
        map = cycle(map.clone());
        if let Some(index) = seen_states.iter().position(|x| x == &map) {
            let cycle_length = seen_states.len() - index;
            let cycle_start = index;
            let final_map =
                &seen_states[cycle_start + (1_000_000_000 - cycle_start) % cycle_length];

            return get_load_on_support(final_map);
        }
        seen_states.push(map.clone());
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn part1_1() {
        let input = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "};
        let result = part_one(&input_generator(input));
        assert_eq!(result, 136);
    }

    #[test]
    fn part2_1() {
        let input = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "};
        let result = part_two(&input_generator(input));
        assert_eq!(result, 64);
    }
}
