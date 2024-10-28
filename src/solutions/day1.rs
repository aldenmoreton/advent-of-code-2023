use itertools::Itertools;

#[aoc(day1, part1)]
fn part_one(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let numbers: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
        let first = numbers.first().unwrap().to_digit(10).unwrap();
        let last = numbers.last().unwrap().to_digit(10).unwrap();

        total += first * 10 + last
    }
    total
}

const NUMBERS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

#[aoc(day1, part2)]
fn part_two(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let mut indices = Vec::new();
        for number in NUMBERS.iter() {
            indices.push(line.match_indices(number));
        }
        let sorted_indices: Vec<_> = indices
            .into_iter()
            .flatten()
            .sorted_by(|(a, _), (b, _)| Ord::cmp(a, b))
            .collect();

        let first = match sorted_indices.first().unwrap() {
            (_, "one") => 1,
            (_, "two") => 2,
            (_, "three") => 3,
            (_, "four") => 4,
            (_, "five") => 5,
            (_, "six") => 6,
            (_, "seven") => 7,
            (_, "eight") => 8,
            (_, "nine") => 9,
            (_, num) => num.parse().unwrap(),
        };

        let last = match sorted_indices.last().unwrap() {
            (_, "one") => 1,
            (_, "two") => 2,
            (_, "three") => 3,
            (_, "four") => 4,
            (_, "five") => 5,
            (_, "six") => 6,
            (_, "seven") => 7,
            (_, "eight") => 8,
            (_, "nine") => 9,
            (_, num) => num.parse().unwrap(),
        };

        total += first * 10 + last;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input = "1abc2";
        let result = part_one(input);
        assert_eq!(result, 12);
    }

    #[test]
    fn part1_2() {
        let input = "pqr3stu8vwx";
        let result = part_one(input);
        assert_eq!(result, 38);
    }

    #[test]
    fn part1_3() {
        let input = "a1b2c3d4e5f";
        let result = part_one(input);
        assert_eq!(result, 15);
    }

    #[test]
    fn part1_4() {
        let input = "treb7uchet";
        let result = part_one(input);
        assert_eq!(result, 77);
    }

    #[test]
    fn part1_5() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let result = part_one(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn part2_1() {
        let input = "two1nine";
        let result = part_two(input);
        assert_eq!(result, 29);
    }

    #[test]
    fn part2_2() {
        let input = "eightwothree";
        let result = part_two(input);
        assert_eq!(result, 83);
    }

    #[test]
    fn part2_3() {
        let input = "abcone2threexyz";
        let result = part_two(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn part2_4() {
        let input = "xtwone3four";
        let result = part_two(input);
        assert_eq!(result, 24);
    }

    #[test]
    fn part2_5() {
        let input = "4nineeightseven2";
        let result = part_two(input);
        assert_eq!(result, 42);
    }

    #[test]
    fn part2_6() {
        let input = "zoneight234";
        let result = part_two(input);
        assert_eq!(result, 14);
    }

    #[test]
    fn part2_7() {
        let input = "7pqrstsixteen";
        let result = part_two(input);
        assert_eq!(result, 76);
    }

    #[test]
    fn part2_8() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let result = part_two(input);
        assert_eq!(result, 281);
    }

    #[test]
    fn part2_9() {
        let input = "asdfadasdf3";
        let result = part_two(input);
        assert_eq!(result, 33);
    }

    #[test]
    fn part2_10() {
        let input = "3asdfadasdf";
        let result = part_two(input);
        assert_eq!(result, 33);
    }

    #[test]
    fn part2_11() {
        let input = "eightwo";
        let result = part_two(input);
        assert_eq!(result, 82);
    }
}
