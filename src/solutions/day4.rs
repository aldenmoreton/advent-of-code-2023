use std::collections::VecDeque;


fn line_wins(input: &str) -> usize {
    let mut curr_wins = 0;
    let (_, win_ours) = input.split_once(": ").unwrap();
    let (winners, ours) = win_ours.split_once(" | ").unwrap();
    let winners: Vec<_> = winners.split_whitespace().collect();
    for number in ours.split_whitespace() {
        if winners.contains(&number) {
            curr_wins += 1;
        }
    }


    curr_wins
}


#[aoc(day4, part1)]
fn part_one(input: &str) -> usize {
    let mut total_points = 0;
    for line in input.lines() {
        let line_points = line_wins(line);
        total_points += if line_points < 3 {
            line_points
        } else {
            2_usize.pow((line_points - 1) as u32)
        };
    }
    total_points
}


#[aoc(day4, part2)]
fn part_two(input: &str) -> usize {
    let mut total_cards = 0;
    let mut card_copies = VecDeque::from([0]);
    for line in input.lines() {
        let curr_wins = line_wins(line);
        let curr_cards = card_copies.pop_front().unwrap_or_default() + 1;
        for i in 0..curr_wins {
            if let Some(num_copies) = card_copies.get_mut(i) {
                *num_copies += curr_cards
            } else {
                card_copies.push_back(curr_cards)
            }
        }


        total_cards += curr_cards;
    }
    total_cards
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn part1_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = part_one(input);
        assert_eq!(result, 8);
    }


    #[test]
    fn part1_2() {
        let input = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let result = part_one(input);
        assert_eq!(result, 2);
    }


    #[test]
    fn part1_3() {
        let input = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let result = part_one(input);
        assert_eq!(result, 2);
    }


    #[test]
    fn part1_4() {
        let input = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let result = part_one(input);
        assert_eq!(result, 1);
    }


    #[test]
    fn part1_5() {
        let input = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let result = part_one(input);
        assert_eq!(result, 0);
    }


    #[test]
    fn part1_6() {
        let input = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part_one(input);
        assert_eq!(result, 0);
    }


    #[test]
    fn part1_7() {
        let input =
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part_one(input);
        assert_eq!(result, 13);
    }


    #[test]
    fn part2_1() {
        let input =
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part_two(input);
        assert_eq!(result, 30);
    }
}