use std::fmt::Debug;

#[derive(PartialEq, Clone)]
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

impl Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::Vertical => write!(f, "|"),
            Pipe::Horizontal => write!(f, "-"),
            Pipe::UpRight => write!(f, "L"),
            Pipe::UpLeft => write!(f, "J"),
            Pipe::DownLeft => write!(f, "7"),
            Pipe::DownRight => write!(f, "F"),
            Pipe::Ground => write!(f, "."),
            Pipe::Start => write!(f, "S")
        }
    }
}

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

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
            (curr_x > 0).then_some((curr_x - 1, curr_y))
        },
        Direction::Left => {
            (curr_y > 0).then_some((curr_x, curr_y - 1))
        },
        Direction::Down => {
            let next_x = curr_x + 1;
            (next_x < x_len).then_some((next_x, curr_y))
        },
        Direction::Right => {
            let next_y = curr_y + 1;
            (next_y < y_len).then_some((curr_x, next_y))
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
    let cardnal_directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
    ];
    for mut curr_direction in cardnal_directions {
        let (mut curr_x, mut curr_y) = start;
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
            if !next_options.contains(curr_pipe) {
                count = 0;
                break
            }
            count += 1;
            if curr_pipe == &Pipe::Start { break }
            curr_direction = get_next_direction(&curr_direction, curr_pipe);
        }
        if count > 0 {
            return count / 2
        }
    }

    panic!("Should have found a solution")
}

#[aoc(day10, part2)]
fn part_two((start, map): &((usize, usize), Vec<Vec<Pipe>>)) -> i32 {
    let (x_len, y_len) = (map.len(), map[0].len());
    let cardnal_directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
    ];

    let mut pipe_map = (0..x_len)
        .map(|_| (0..y_len)
            .map(|_| Pipe::Ground)
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    for mut curr_direction in cardnal_directions {
        pipe_map
            .iter_mut()
            .for_each(|row| {
                row
                    .iter_mut()
                    .for_each(|column| { *column = Pipe::Ground });
            });
        let (mut curr_x, mut curr_y) = start;
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
            if !next_options.contains(curr_pipe) {
                count = 0;
                break
            }
            count += 1;
            pipe_map[curr_x][curr_y] = curr_pipe.clone();
            if curr_pipe == &Pipe::Start {
                let next_dir = if curr_x > 0 && pipe_map[curr_x - 1][curr_y] != Pipe::Ground && curr_direction != Direction::Down {
                    Direction::Up
                } else if curr_y > 0 && pipe_map[curr_x][curr_y - 1] != Pipe::Ground && curr_direction != Direction::Right {
                    Direction::Left
                } else if curr_x < x_len && pipe_map[curr_x + 1][curr_y] != Pipe::Ground && curr_direction != Direction::Up {
                    Direction::Down
                } else {
                    Direction::Right
                };
                pipe_map[curr_x][curr_y] = match (curr_direction, next_dir) {
                    (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => Pipe::Horizontal,
                    (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => Pipe::Vertical,
                    (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => Pipe::UpRight,
                    (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => Pipe::UpLeft,
                    (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => Pipe::DownRight,
                    (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down) => Pipe::DownLeft,
                    _ => panic!("Start position should not be this")
                };
                break
            }
            curr_direction = get_next_direction(&curr_direction, curr_pipe);
        }
        if count > 0 {
            break
        }
    }
    pipe_map
        .iter()
        .for_each(|row| println!("{row:?}"));

    let next_options = get_next_options(&Direction::Up);
    let mut area = 0;
    for (i, row) in pipe_map.iter().enumerate() {
        for (j, pipe) in row.iter().enumerate() {
            let mut hit_count = 0;
            let (mut curr_i, mut curr_j) = (i, j);
            let mut curr_pipe = &pipe.clone();
            if *curr_pipe != Pipe::Ground {
                continue
            }
            'outer: loop {
                if *curr_pipe != Pipe::Ground {
                    if *curr_pipe == Pipe::Horizontal {
                        hit_count += 1
                    } else {
                        let enter_pipe = (*curr_pipe).clone();
                        (curr_i, curr_j) = if let Some((x, y)) = get_next_index(&Direction::Up, curr_i, curr_j, x_len, y_len) {
                            (x, y)
                        } else {
                            break 'outer
                        };
                        curr_pipe = &pipe_map[curr_i][curr_j];
                        while *curr_pipe == Pipe::Vertical {
                            (curr_i, curr_j) = if let Some((x, y)) = get_next_index(&Direction::Up, curr_i, curr_j, x_len, y_len) {
                                (x, y)
                            } else {
                                break 'outer
                            };
                            curr_pipe = &pipe_map[curr_i][curr_j];
                            if !next_options.contains(curr_pipe) && *curr_pipe != Pipe::Ground {
                                hit_count += 1
                            }
                        }
                        hit_count += match enter_pipe {
                            Pipe::UpRight => match curr_pipe {
                                Pipe::DownLeft => 1,
                                Pipe::DownRight => 2,
                                _ => panic!("You shouldn't be able to hit this pipe")
                            },
                            Pipe::UpLeft => match curr_pipe {
                                Pipe::DownLeft => 2,
                                Pipe::DownRight => 1,
                                _ => panic!("You shouldn't be able to hit this pipe")
                            },
                            _ => panic!("You shouldn't be able to hit this pipe")
                        };
                    }
                }

                (curr_i, curr_j) = if let Some((x, y)) = get_next_index(&Direction::Up, curr_i, curr_j, x_len, y_len) {
                    (x, y)
                } else {
                    break
                };
                curr_pipe = &pipe_map[curr_i][curr_j];
            }
            if hit_count % 2 == 1 {
                area += 1
            }
        }
    }

    area
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

    #[test]
    fn part2_1() {
        let input = indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "};
        let result = part_two(&input_generator(input));
        assert_eq!(result, 4)
    }

    #[test]
    fn part2_2() {
        let input = indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "};
        let result = part_two(&input_generator(input));
        assert_eq!(result, 8)
    }

    #[test]
    fn part2_3() {
        let input = indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "};
        let result = part_two(&input_generator(input));
        assert_eq!(result, 10)
    }
}
