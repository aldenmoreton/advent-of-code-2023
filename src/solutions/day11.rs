use itertools::Itertools;

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line|
            line
                .chars()
                .map(|character| match character {
                    '.' => false,
                    '#' => true,
                    _ => panic!("This character should not be in input")
                })
                .collect::<Vec<_>>()
        )
        .collect()
}

#[aoc(day11, part1)]
fn part_one(input: &[Vec<bool>]) -> usize {
    let col_len = input[0].len();
    let empty_rows: Vec<usize> = input
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            row
                .iter()
                .all(|galaxy| !*galaxy)
                .then_some(i)
        })
        .collect();

    let mut empty_cols = Vec::new();
    for col in 0..col_len {
        let mut empty = true;
        for row in input.iter() {
            if row[col] {
                empty = false;
                break
            }
        }
        if empty {
            empty_cols.push(col)
        }
    }

    let galaxies: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)|
            row
                .iter()
                .enumerate()
                .filter_map(|(j, galexy)|
                    galexy.then_some((i, j))
                )
                .collect::<Vec<_>>()
        )
        .collect();

    let sum = galaxies
        .iter()
        .combinations(2)
        .map(|galaxies| {
            let (x1, x2) = if galaxies[0].0 < galaxies[1].0 {
                (galaxies[0].0, galaxies[1].0)
            } else {
                (galaxies[1].0, galaxies[1].0)
            };
            let (y1, y2) = if galaxies[0].1 < galaxies[1].1 {
                (galaxies[0].1, galaxies[1].1)
            } else {
                (galaxies[1].1, galaxies[0].1)
            };


            let mut x_distance = x2 - x1;
            for space in empty_rows.iter() {
                if x1 < *space && *space < x2 {
                    x_distance += 1
                }
            }


            let mut y_distance = y2 - y1;
            for space in empty_cols.iter() {
                if y1 < *space && *space < y2 {
                    y_distance += 1
                }
            }
            x_distance + y_distance
        })
        .sum();

    sum
}

#[aoc(day11, part2)]
fn part_two(input: &[Vec<bool>]) -> usize {
    let col_len = input[0].len();
    let empty_rows: Vec<usize> = input
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            row
                .iter()
                .all(|galaxy| !*galaxy)
                .then_some(i)
        })
        .collect();

    let mut empty_cols = Vec::new();
    for col in 0..col_len {
        let mut empty = true;
        for row in input.iter() {
            if row[col] {
                empty = false;
                break
            }
        }
        if empty {
            empty_cols.push(col)
        }
    }

    let galaxies: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)|
            row
                .iter()
                .enumerate()
                .filter_map(|(j, galexy)|
                    galexy.then_some((i, j))
                )
                .collect::<Vec<_>>()
        )
        .collect();

    let sum = galaxies
        .iter()
        .combinations(2)
        .map(|galaxies| {
            let (x1, x2) = if galaxies[0].0 < galaxies[1].0 {
                (galaxies[0].0, galaxies[1].0)
            } else {
                (galaxies[1].0, galaxies[1].0)
            };
            let (y1, y2) = if galaxies[0].1 < galaxies[1].1 {
                (galaxies[0].1, galaxies[1].1)
            } else {
                (galaxies[1].1, galaxies[0].1)
            };

            let mut x_distance = x2 - x1;
            for space in empty_rows.iter() {
                if x1 < *space && *space < x2 {
                    x_distance += 999_999
                }
            }

            let mut y_distance = y2 - y1;
            for space in empty_cols.iter() {
                if y1 < *space && *space < y2 {
                    y_distance += 999_999
                }
            }
            x_distance + y_distance
        })
        .sum();

    sum
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn part1_1() {
        let input = indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};
        let result = part_one(&input_generator(input));
        assert_eq!(result, 374);
    }

    #[test]
    fn part2_1() {
        let input = indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};
        let result = part_two(&input_generator(input));
        assert_eq!(result, 82_000_210);
    }
}
