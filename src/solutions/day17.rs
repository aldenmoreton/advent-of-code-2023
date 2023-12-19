use itertools::Itertools;
use pathfinding::prelude::{dijkstra, astar};

type Grid = Vec<Vec<u32>>;
type Coords = (usize, usize);
type DirectionDuration = (u32, Direction);
type State = (Coords, DirectionDuration);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    const ALL_DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right
    ];

    fn complement(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

#[aoc_generator(day17)]
fn input_generator(input: &str) -> Grid {
    input
        .lines()
        .map(|line|
            line
                .chars()
                .map(move |character|
                    character.to_digit(10).unwrap()
                )
                .collect()
        )
        .collect()
}

fn next_pos(
    direction: &Direction,
    (x, y): (&usize, &usize),
    grid: &Grid
) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            x
                .checked_sub(1)
                .map(|next_x| (next_x, *y))
        },
        Direction::Down => {
            let x_len = grid.len();
            let next_x = x + 1;
            (next_x < x_len)
                .then_some((next_x, *y))
        },
        Direction::Left => {
            y
                .checked_sub(1)
                .map(|next_y| (*x, next_y))
        },
        Direction::Right => {
            let y_len = grid[0].len();
            let next_y = y + 1;
            (next_y < y_len)
                .then_some((*x, next_y))
        }
    }
}

#[aoc(day17, part1, Dijkstra)]
fn part_one(grid: &Grid) -> u32 {
    let (x_len, y_len) = (grid.len(), grid[0].len());

    let shortest_path: (Vec<State>, u32) = dijkstra(
        &((0, 0), (0, Direction::Right)),
        |((curr_x, curr_y), (prev_duration, prev_direction))| {
            let complement = prev_direction.complement();
            Direction::ALL_DIRECTIONS
                .iter()
                // Remove unavailable directions
                .filter(|pos_direction| {
                    if **pos_direction == complement {
                        false
                    } else if *prev_duration == 3 {
                        *pos_direction != prev_direction
                    } else {
                        true
                    }
                })
                // Convert directions to coords and validate bounds
                .filter_map(|pos_direction|
                    next_pos(pos_direction, (curr_x, curr_y), grid)
                        .map(|coords| (coords, pos_direction))
                )
                // Add direction duration tracking
                .map(|(pos_coords, pos_direction)| {
                    if pos_direction == prev_direction {
                        ((pos_coords, (prev_duration+1, prev_direction.clone())), grid[pos_coords.0][pos_coords.1])
                    } else {
                        ((pos_coords, (1, pos_direction.clone())), grid[pos_coords.0][pos_coords.1])
                    }
                })
                .collect_vec()
        },
        |(position, _)| {
            *position == (x_len-1, y_len-1)
        }
    )
    .expect("Should have valid path");

    shortest_path.1
}

#[aoc(day17, part1, Astar)]
fn part_one_astar(grid: &Grid) -> u32 {
    let (x_len, y_len) = (grid.len(), grid[0].len());

    let shortest_path: (Vec<State>, u32) = astar(
        &((0, 0), (0, Direction::Right)),
        |((curr_x, curr_y), (prev_duration, prev_direction))| {
            let complement = prev_direction.complement();
            Direction::ALL_DIRECTIONS
                .iter()
                .filter(|pos_direction| {
                    if **pos_direction == complement {
                        false
                    } else if *prev_duration == 3 {
                        *pos_direction != prev_direction
                    } else {
                        true
                    }
                })
                .filter_map(|pos_direction|
                    next_pos(pos_direction, (curr_x, curr_y), grid)
                        .map(|coords| (coords, pos_direction))
                )
                .map(|(pos_coords, pos_direction)| {
                    if pos_direction == prev_direction {
                        ((pos_coords, (prev_duration+1, prev_direction.clone())), grid[pos_coords.0][pos_coords.1])
                    } else {
                        ((pos_coords, (1, pos_direction.clone())), grid[pos_coords.0][pos_coords.1])
                    }
                })
                .collect_vec()
        },
        |((curr_x, curr_y), _)| {
            let (target_x, target_y) = (x_len-1, y_len-1);
            ((target_x - curr_x) + (target_y - curr_y)) as u32
        },
        |(position, _)| {
            *position == (x_len-1, y_len-1)
        }
    )
    .expect("Should have valid path");

    shortest_path.1
}

#[aoc(day17, part2, Dijkstra)]
fn part_two(grid: &Grid) -> u32 {
    let (x_len, y_len) = (grid.len(), grid[0].len());

    let shortest_path: (Vec<State>, u32) = dijkstra(
        &((0, 0), (0, Direction::Right)),
        |((curr_x, curr_y), (prev_duration, prev_direction))| {
            let complement = prev_direction.complement();
            Direction::ALL_DIRECTIONS
                .iter()
                .filter(|pos_direction| {
                    if **pos_direction == complement {
                        false
                    } else if *prev_duration < 4 {
                        *pos_direction == prev_direction
                    } else if *prev_duration == 10 {
                        *pos_direction != prev_direction
                    } else {
                        true
                    }
                })
                .filter_map(|pos_direction|
                    next_pos(pos_direction, (curr_x, curr_y), grid)
                        .map(|coords| (coords, pos_direction))
                )
                .map(|(pos_coords, pos_direction)| {
                    if pos_direction == prev_direction {
                        ((pos_coords, (prev_duration+1, prev_direction.clone())), grid[pos_coords.0][pos_coords.1])
                    } else {
                        ((pos_coords, (1, pos_direction.clone())), grid[pos_coords.0][pos_coords.1])
                    }
                })
                .collect_vec()
        },
        |(position, (prev_duration, _))| {
            *position == (x_len-1, y_len-1) && *prev_duration >= 4
        }
    )
    .expect("Should have valid path");

    shortest_path.1
}

#[aoc(day17, part2, Astar)]
fn part_two_astar(grid: &Grid) -> u32 {
    let (x_len, y_len) = (grid.len(), grid[0].len());

    let shortest_path: (Vec<State>, u32) = astar(
        &((0, 0), (0, Direction::Right)),
        |((curr_x, curr_y), (prev_duration, prev_direction))| {
            let complement = prev_direction.complement();
            Direction::ALL_DIRECTIONS
                .iter()
                .filter(|pos_direction| {
                    if **pos_direction == complement {
                        false
                    } else if *prev_duration < 4 {
                        *pos_direction == prev_direction
                    } else if *prev_duration == 10 {
                        *pos_direction != prev_direction
                    } else {
                        true
                    }
                })
                .filter_map(|pos_direction|
                    next_pos(pos_direction, (curr_x, curr_y), grid)
                        .map(|coords| (coords, pos_direction))
                )
                .map(|(pos_coords, pos_direction)| {
                    if pos_direction == prev_direction {
                        ((pos_coords, (prev_duration+1, prev_direction.clone())), grid[pos_coords.0][pos_coords.1])
                    } else {
                        ((pos_coords, (1, pos_direction.clone())), grid[pos_coords.0][pos_coords.1])
                    }
                })
                .collect_vec()
        },
        |((curr_x, curr_y), _)| {
            let (target_x, target_y) = (x_len-1, y_len-1);
            ((target_x - curr_x) + (target_y - curr_y)) as u32
        },
        |(position, (prev_duration, _))| {
            *position == (x_len-1, y_len-1) && *prev_duration >= 4
        }
    )
    .expect("Should have valid path");

    shortest_path.1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_1() {
        let input = indoc! {"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        "};
        let result = part_one(&input_generator(input));
        assert_eq!(result, 102);
    }

    #[test]
    fn part2_1() {
        let input = indoc! {"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        "};
        let result = part_two(&input_generator(input));
        assert_eq!(result, 94);
    }

    #[test]
    fn part2_2() {
        let input = indoc! {"
            111111111111
            999999999991
            999999999991
            999999999991
            999999999991
        "};
        let result = part_two(&input_generator(input));
        assert_eq!(result, 71);
    }
}
