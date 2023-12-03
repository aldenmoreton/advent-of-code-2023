use std::collections::HashSet;

#[aoc(day3, part1)]
fn part_one(input: &str) -> u32 {
    let mut potential_parts = Vec::new();
    let mut symbols = HashSet::new();
    let mut new_number = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, curr_char) in line.chars().enumerate() {
            match curr_char {
                '.' => {
                    if new_number.len() > 0 {
                        let number_string: String = new_number.iter().collect();
                        let first_index = j - number_string.len();
                        potential_parts.push((number_string, (i, (first_index, j - 1))));
                        new_number.clear();
                    }
                },
                curr_char if curr_char.is_numeric() => {
                    new_number.push(curr_char);
                    if j == line.len() - 1 {
                        let number_string: String = new_number.iter().collect();
                        let first_index = j - number_string.len() + 1;
                        potential_parts.push((number_string, (i, (first_index, j))));
                        new_number.clear();
                    }
                },
                _ => {
                    symbols.insert((i, j));
                    if new_number.len() > 0 {
                        let number_string: String = new_number.iter().collect();
                        println!("{number_string}");
                        let first_index = j - number_string.len();
                        potential_parts.push((number_string, (i, (first_index, j - 1))));
                        new_number.clear();
                    }
                }
            }
        }
    }
    println!("Potentials: {:?}", potential_parts);
    println!("Len: {:?}", potential_parts.len());

    let mut part_total = 0;
    for (part_number, (number_i, (number_first_j, number_last_j))) in potential_parts {
        let first_i = if number_i > 0 {
            number_i - 1
        } else {
            0
        };
        let last_i = number_i + 1;
        'number_check: for i in first_i..=last_i {
            let first_j = if number_first_j > 0 {
                number_first_j - 1
            } else {
                0
            };
            let last_j = number_last_j + 1;
            println!("Checking number {part_number}");
            println!("Checking between ({},{})-({},{})", first_i, first_j, last_i, last_j);
            for j in first_j..=last_j {
                println!("Checking: {i},{j}");
                if symbols.contains(&(i, j)) {
                    println!("Add this number: {part_number}");
                    println!("Because of this: {},{}", i, j);
                    part_total += part_number.parse::<u32>().unwrap();
                    break 'number_check
                }
            }
        }
        println!("-----");
    }

    part_total
}

#[aoc(day3, part2)]
fn part_two(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let result = part_one(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn part1_2() {
        let input = "....$123";
        let result = part_one(input);
        assert_eq!(result, 123)
    }

    #[test]
    fn part1_3() {
        let input = "....123$";
        let result = part_one(input);
        assert_eq!(result, 123)
    }

    #[test]
    fn part1_4() {
        let input = "....123";
        let result = part_one(input);
        assert_eq!(result, 0)
    }

    #[test]
    fn part2_1() {
        let input = "";
        let result = part_two(input);
        assert_eq!(result, 0);
    }
}
