use itertools::Itertools;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line
                .split(' ')
                .map(|number| number.parse::<i32>().unwrap())
                .collect()
        )
        .collect()
}

#[aoc(day9, part1)]
fn part_one(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|history| {
            let mut derivatives = vec![history.to_owned()];
            while *derivatives.last().unwrap().last().unwrap() != 0 {
                let next_derivative = derivatives
                    .last()
                    .unwrap()
                    .iter()
                    .tuple_windows()
                    .map(|(l, r)| r-l)
                    .collect();
                derivatives.push(next_derivative)
            }
            derivatives
                .into_iter()
                .rev()
                .skip(1)
                .fold(0, |acc, row| {
                    acc + row.last().unwrap()
                })
        })
        .sum()
}

#[aoc(day9, part2)]
fn part_two(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|history| {
            let mut derivatives = vec![history.to_owned()];
            while *derivatives.last().unwrap().last().unwrap() != 0 {
                let next_derivative = derivatives
                    .last()
                    .unwrap()
                    .iter()
                    .tuple_windows()
                    .map(|(l, r)| r-l)
                    .collect();
                derivatives.push(next_derivative)
            }
            derivatives
                .into_iter()
                .rev()
                .skip(1)
                .fold(0, |acc, row| {
                    row.first().unwrap() - acc
                })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn part1_1() {
        let input = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};
        let result = part_one(&input_generator(input));
        assert_eq!(result, 114);
    }

    #[test]
    fn part2_1() {
        let input = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};
        let result = part_two(&input_generator(input));
        assert_eq!(result, 2);
    }
}
