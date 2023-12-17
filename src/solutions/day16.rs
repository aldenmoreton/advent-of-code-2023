use std::collections::{HashSet, VecDeque};
use rayon::prelude::*;

type Grid = Vec<Vec<Tile>>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

enum Tile {
    LeftUp,
    RightUp,
    VerticalSplitter,
    HorizontalSplitter,
    Empty
}

#[derive(Debug)]
struct Movement {
    x: usize,
    y: usize,
    direction: Direction
}

enum MoveResult {
    NextMove(Movement),
    Split(Movement, Movement),
    OutOfBounds
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Grid {
    input
        .lines()
        .map(|line|
            line
                .chars()
                .map(|character|
                    match character {
                        '.' => Tile::Empty,
                        '/' => Tile::LeftUp,
                        '\\' => Tile::RightUp,
                        '|' => Tile::VerticalSplitter,
                        '-' => Tile::HorizontalSplitter,
                        _ => panic!("This is not a valid tile")
                    }
                )
                .collect()
        )
        .collect()
}

fn next_tile(movement: &Movement, grid: &Grid) -> Option<(usize, usize)> {
    match &movement.direction {
        Direction::Up => {
            movement.x
                .checked_sub(1)
                .map(|next_x| (next_x, movement.y))
        },
        Direction::Down => {
            let x_len = grid.len();
            let next_x = movement.x + 1;
            (next_x < x_len)
                .then_some((next_x, movement.y))
        },
        Direction::Left => {
            movement.y
                .checked_sub(1)
                .map(|next_y| (movement.x, next_y))
        },
        Direction::Right => {
            let y_len = grid[0].len();
            let next_y = movement.y + 1;
            (next_y < y_len)
                .then_some((movement.x, next_y))
        }
    }
}

fn move_tile(movement: Movement, grid: &Grid) -> MoveResult {
    let curr_tile = &grid[movement.x][movement.y];
    match (&movement.direction, curr_tile) {
        (_, Tile::Empty) |
        (Direction::Up, Tile::VerticalSplitter) |
        (Direction::Down, Tile::VerticalSplitter) |
        (Direction::Left, Tile::HorizontalSplitter) |
        (Direction::Right, Tile::HorizontalSplitter) => {
            let pos_next_tile = next_tile(&movement, grid);
            let (next_x, next_y) = {
                if let Some(tile) = pos_next_tile {
                    tile
                } else {
                    return MoveResult::OutOfBounds
                }
            };

            MoveResult::NextMove(Movement { x: next_x, y: next_y, direction: movement.direction })
        },
        (Direction::Right, Tile::LeftUp) |
        (Direction::Left, Tile::RightUp)=> {
            let pos_next_tile = next_tile(&Movement { x: movement.x, y: movement.y, direction: Direction::Up }, grid);
            let (next_x, next_y) = {
                if let Some(tile) = pos_next_tile {
                    tile
                } else {
                    return MoveResult::OutOfBounds
                }
            };

            MoveResult::NextMove(Movement { x: next_x, y: next_y, direction: Direction::Up })
        },
        (Direction::Left, Tile::LeftUp) |
        (Direction::Right, Tile::RightUp) => {
            let pos_next_tile = next_tile(&Movement { x: movement.x, y: movement.y, direction: Direction::Down }, grid);
            let (next_x, next_y) = {
                if let Some(tile) = pos_next_tile {
                    tile
                } else {
                    return MoveResult::OutOfBounds
                }
            };

            MoveResult::NextMove(Movement { x: next_x, y: next_y, direction: Direction::Down })
        },
        (Direction::Up, Tile::RightUp) |
        (Direction::Down, Tile::LeftUp) => {
            let pos_next_tile = next_tile(&Movement { x: movement.x, y: movement.y, direction: Direction::Left }, grid);
            let (next_x, next_y) = {
                if let Some(tile) = pos_next_tile {
                    tile
                } else {
                    return MoveResult::OutOfBounds
                }
            };

            MoveResult::NextMove(Movement { x: next_x, y: next_y, direction: Direction::Left })
        }
        (Direction::Up, Tile::LeftUp) |
        (Direction::Down, Tile::RightUp) => {
            let pos_next_tile = next_tile(&Movement { x: movement.x, y: movement.y, direction: Direction::Right }, grid);
            let (next_x, next_y) = {
                if let Some(tile) = pos_next_tile {
                    tile
                } else {
                    return MoveResult::OutOfBounds
                }
            };

            MoveResult::NextMove(Movement { x: next_x, y: next_y, direction: Direction::Right })
        },
        (Direction::Up, Tile::HorizontalSplitter) |
        (Direction::Down, Tile::HorizontalSplitter) => {
            let pos_next_tile_1 = next_tile(&Movement { x: movement.x, y: movement.y, direction: Direction::Left }, grid);
            let pos_next_tile_2 = next_tile(&Movement { x: movement.x, y: movement.y, direction: Direction::Right }, grid);

            match (pos_next_tile_1, pos_next_tile_2) {
                (None, None) => MoveResult::OutOfBounds,
                (Some((next_x, next_y)), None) => MoveResult::NextMove(Movement { x: next_x, y: next_y, direction: Direction::Left }),
                (None, Some((next_x, next_y))) => MoveResult::NextMove(Movement { x: next_x, y: next_y, direction: Direction::Right }),
                (Some((next_x_1, next_y_1)), Some((next_x_2, next_y_2))) =>
                    MoveResult::Split(
                        Movement { x: next_x_1, y: next_y_1, direction: Direction::Left },
                        Movement { x: next_x_2, y: next_y_2, direction: Direction::Right }
                    ),
            }
        },
        (Direction::Left, Tile::VerticalSplitter) |
        (Direction::Right, Tile::VerticalSplitter) => {
            let pos_next_tile_1 = next_tile(&Movement { x: movement.x, y: movement.y, direction: Direction::Up }, grid);
            let pos_next_tile_2 = next_tile(&Movement { x: movement.x, y: movement.y, direction: Direction::Down }, grid);

            match (pos_next_tile_1, pos_next_tile_2) {
                (None, None) => MoveResult::OutOfBounds,
                (Some((next_x, next_y)), None) => MoveResult::NextMove(Movement { x: next_x, y: next_y, direction: Direction::Up }),
                (None, Some((next_x, next_y))) => MoveResult::NextMove(Movement { x: next_x, y: next_y, direction: Direction::Down }),
                (Some((next_x_1, next_y_1)), Some((next_x_2, next_y_2))) =>
                    MoveResult::Split(
                        Movement { x: next_x_1, y: next_y_1, direction: Direction::Up },
                        Movement { x: next_x_2, y: next_y_2, direction: Direction::Down }
                    ),
            }
        }
    }
}

#[aoc(day16, part1)]
fn part_one(grid: &Grid) -> usize {
    let mut tiles_visited: HashSet<(usize, usize)> = HashSet::new();
    let mut dashes_done = HashSet::new();

    let mut tile_paths = VecDeque::new();
    tile_paths.push_back(Movement{x: 0, y: 0, direction: Direction::Right});
    while let Some(mut curr_move) = tile_paths.pop_front() {
        loop {
            tiles_visited.insert((curr_move.x, curr_move.y));
            if !dashes_done.insert((curr_move.x, curr_move.y, curr_move.direction.clone())) {
                break
            }

            let next_action = move_tile(curr_move, grid);
            curr_move = match next_action {
                MoveResult::OutOfBounds => break,
                MoveResult::NextMove(movement) => movement,
                MoveResult::Split(curr_movement, next_movement) => {
                    tile_paths.push_back(next_movement);
                    curr_movement
                }
            };
        }
    }

    tiles_visited.len()
}

fn check_path(starting_spot: Movement, grid: &Grid) -> usize {
    let mut tiles_visited: HashSet<(usize, usize)> = HashSet::new();
    let mut dashes_done = HashSet::new();

    let mut tile_paths = VecDeque::new();
    tile_paths.push_back(starting_spot);

    while let Some(mut curr_move) = tile_paths.pop_front() {
        loop {
            tiles_visited.insert((curr_move.x, curr_move.y));
            if !dashes_done.insert((curr_move.x, curr_move.y, curr_move.direction.clone())) {
                break
            }

            let next_action = move_tile(curr_move, grid);
            curr_move = match next_action {
                MoveResult::OutOfBounds => break,
                MoveResult::NextMove(movement) => movement,
                MoveResult::Split(curr_movement, next_movement) => {
                    tile_paths.push_back(next_movement);
                    curr_movement
                }
            };
        }
    }

    tiles_visited.len()
}

#[aoc(day16, part2)]
fn part_two(grid: &Grid) -> usize {
    let (len_x, len_y) = (grid.len(), grid[0].len());

    let max_1 = (0..len_x)
        .into_par_iter()
        .map(|i| {
            std::cmp::max(
                check_path(
                    Movement { x: i, y: 0, direction: Direction::Right },
                    grid
                ),
                check_path(
                    Movement { x: i, y: len_y-1, direction: Direction::Left }, grid)
            )
        })
        .max()
        .unwrap();

    let max_2 = (0..len_y)
        .into_par_iter()
        .map(|i| {
            std::cmp::max(
                check_path(
                    Movement { x: 0, y: i, direction: Direction::Down },
                    grid
                ),
                check_path(
                    Movement { x: len_x-1, y: i, direction: Direction::Up }, grid)
            )
        })
        .max()
        .unwrap();

    std::cmp::max(max_1, max_2)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn part1_1() {
        let input = indoc! {r#"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        "#};
        let result = part_one(&input_generator(input));
        assert_eq!(result, 46);
    }

    #[test]
    fn part2_1() {
        let input = indoc! {r#"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        "#};
        let result = part_two(&input_generator(input));
        assert_eq!(result, 51);
    }
}
