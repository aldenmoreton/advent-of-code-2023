use rayon::prelude::*;

#[derive(PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
    Ground,
    Start
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal


#[aoc_generator(day10)]
fn input_generator(input: &str) -> ((usize, usize), Vec<Vec<Pipe>>) {
    let mut start = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(i, line)|
            line
                .char_indices()
                .map(|(j, character)| match character {
                    '|' => Pipe::Vertical,
                    '-' => Pipe::Horizontal,
                    'L' => Pipe::UpRight,
                    'J' => Pipe::UpLeft,
                    '7' => Pipe::DownLeft,
                    'F' => Pipe::DownRight,
                    '.' => Pipe::Ground,
                    'S' => {
                        start = (i, j);
                        Pipe::Start
                    },
                    _ => unreachable!()
                })
                .collect::<Vec<_>>()
        )
        .collect();
    (start, map)
}

// const DIRECTIONS: [(i32, i32); 4] = [
//     (-1, -1),
//     (-1,  1),
//     ( 1, -1),
//     ( 1,  1)
// ];

fn get_next_options(curr_direction: &Direction) -> Vec<Pipe> {
    let mut next_options = vec![Pipe::Start];
    match curr_direction {
        Direction::Up => {
            next_options.push(Pipe::Vertical);
            next_options.push(Pipe::DownLeft);
            next_options.push(Pipe::DownRight);
        },
        Direction::Down => {
            next_options.push(Pipe::Vertical);
            next_options.push(Pipe::UpLeft);
            next_options.push(Pipe::UpRight);
        },
        Direction::Left => {
            next_options.push(Pipe::Horizontal);
            next_options.push(Pipe::UpRight);
            next_options.push(Pipe::DownRight);
        },
        Direction::Right => {
            next_options.push(Pipe::Horizontal);
            next_options.push(Pipe::UpLeft);
            next_options.push(Pipe::DownLeft);
        }
    }
    next_options
}

fn get_next_index(curr_direction: &Direction, curr_x: usize, curr_y: usize, x_len: usize, y_len: usize) -> Option<(usize, usize)> {
    match curr_direction {
        Direction::Up => {
            if curr_x > 0 {
                Some((curr_x - 1, curr_y))
            } else {
                None
            }
        },
        Direction::Left => {
            if curr_y > 0 {
                Some((curr_x, curr_y - 1))
            } else {
                None
            }
        },
        Direction::Down => {
            let next_x = curr_x + 1;
            if next_x < x_len {
                Some((next_x, curr_y))
            } else {
                None
            }
        },
        Direction::Right => {
            let next_y = curr_y + 1;
            if next_y < y_len {
                Some((curr_x, next_y))
            } else {
                None
            }
        }
    }
}

fn get_next_direction(prev_direction: &Direction, curr_pipe: &Pipe) -> Direction {
    match curr_pipe {
        Pipe::Vertical | Pipe::Horizontal => (*prev_direction).clone(),
        Pipe::UpLeft => match prev_direction {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
            _ => panic!("You shouldn't be here")
        },
        Pipe::UpRight => match prev_direction {
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Right,
            _ => panic!("You shouldn't be here")
        },
        Pipe::DownLeft => match prev_direction {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Left,
            _ => panic!("You shouldn't be here")
        },
        Pipe::DownRight => match prev_direction {
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Right,
            _ => panic!("You shouldn't be here")
        }
        Pipe::Ground => panic!("Nowhere to go"),
        Pipe::Start => panic!("You have arrived!")
    }
}

#[aoc(day10, part1)]
fn part_one((start, map): &((usize, usize), Vec<Vec<Pipe>>)) -> i32 {
    let (x_len, y_len) = (map.len(), map[0].len());
    let length = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
    ]
        .into_par_iter()
        .map(|mut curr_direction| {
            let (mut curr_x, mut curr_y) = start.clone();
            let mut count = 0;
            loop {
                (curr_x, curr_y) = if let Some((x, y)) = get_next_index(&curr_direction, curr_x, curr_y, x_len, y_len) {
                    (x, y)
                } else {
                    count = 0;
                    break
                };
                let next_options = get_next_options(&curr_direction);
                let curr_pipe = &map[curr_x][curr_y];
                if !next_options.contains(&curr_pipe) {
                    count = 0;
                    break
                }
                count += 1;
                if curr_pipe == &Pipe::Start { break }
                curr_direction = get_next_direction(&curr_direction, curr_pipe);
            }
            count
        })
        .max()
        .unwrap();

    length / 2
}

#[aoc(day10, part2)]
fn part_two((_start, _map): &((usize, usize), Vec<Vec<Pipe>>)) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn part1_1() {
        let input = indoc! {"
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "};
        let result = part_one(&input_generator(input));
        assert_eq!(result, 4);
    }

    #[test]
    fn part1_2() {
        let input = indoc! {"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "};
        let result = part_one(&input_generator(input));
        assert_eq!(result, 8);
    }
}
