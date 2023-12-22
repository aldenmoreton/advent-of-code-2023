use std::collections::HashSet;


type Coords = (usize, usize);
type Grid = Vec<Vec<bool>>;

#[aoc_generator(day21)]
fn input_generator(input: &str) -> (Coords, Grid) {
    let mut start_point = (0,0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(i, line)|
            line
                .char_indices()
                .map(|(j, character)| {
                    if character == 'S' {
                        start_point = (i, j)
                    }
                    character != '#'
                })
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    (start_point, grid)
}

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
}

fn next_pos(
    direction: &Direction,
    (x, y): (usize, usize),
    grid: &Grid
) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            x
                .checked_sub(1)
                .map(|next_x| (next_x, y))
        },
        Direction::Down => {
            let x_len = grid.len();
            let next_x = x + 1;
            (next_x < x_len)
                .then_some((next_x, y))
        },
        Direction::Left => {
            y
                .checked_sub(1)
                .map(|next_y| (x, next_y))
        },
        Direction::Right => {
            let y_len = grid[0].len();
            let next_y = y + 1;
            (next_y < y_len)
                .then_some((x, next_y))
        }
    }
}

fn step_counter(num_steps: usize, starting_point: Coords, grid: &Grid) -> usize {
    let mut curr_locations = HashSet::new();
    curr_locations.insert(starting_point);
    for _ in 0..num_steps {
        let mut next_locations = HashSet::new();
        for location in curr_locations {
            Direction::ALL_DIRECTIONS
                .iter()
                .filter_map(|direction|
                    next_pos(direction, location, grid)
                )
                .filter(|coords| grid[coords.0][coords.1])
                .for_each(|coords| { next_locations.insert(coords); })
        }
        curr_locations = next_locations;
    }
    curr_locations.len()
}

#[aoc(day21, part1)]
fn part_one((start_point, grid): &(Coords, Grid)) -> usize {
    step_counter(64, start_point.clone(), grid)
}

#[aoc(day21, part2)]
fn part_two(_input: &(Coords, Grid)) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_1() {
        let input = indoc! {"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
        "};
        let (starting_point, grid) = input_generator(input);
        let result = step_counter(6, starting_point, &grid);
        assert_eq!(result, 16);
    }

    // #[test]
    // fn part2_1() {
    //     let input = indoc! {""};
    //     let result = part_two(&input_generator(input));
    //     assert_eq!(result, 0);
    // }
}
