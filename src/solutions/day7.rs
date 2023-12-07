use itertools::Itertools;

#[derive(Ord, PartialEq, PartialOrd, Eq, Debug)]
enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    ty: HandType,
    cards: [u8; 5],
    bid: u32,
}

#[aoc_generator(day7, part1)]
fn input_generator_part_one(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in input.lines() {
        let (cards, bid) = line.split_once(' ').unwrap();
        let mut card_counts: [u8; 13] = [0; 13];
        let mut numeric_representation_of_cards = [0; 5];
        for (i, card) in cards.chars().enumerate() {
            match card {
                card if card.is_numeric() => {
                    let card_number: u8 = card.to_digit(10).unwrap() as u8;
                    numeric_representation_of_cards[i] = card_number - 2;
                    card_counts[(card_number - 2) as usize] += 1;
                },
                'A' => {
                    numeric_representation_of_cards[i] = 12;
                    card_counts[12] += 1;
                },
                'K' => {
                    numeric_representation_of_cards[i] = 11;
                    card_counts[11] += 1;
                },
                'Q' => {
                    numeric_representation_of_cards[i] = 10;
                    card_counts[10] += 1;
                },
                'J' => {
                    numeric_representation_of_cards[i] = 9;
                    card_counts[9] += 1;
                },
                'T' => {
                    numeric_representation_of_cards[i] = 8;
                    card_counts[8] += 1;
                },
                _ => unreachable!()
            }
        }
        let card_counts = card_counts
            .into_iter()
            .filter(|count| *count > 0)
            .collect_vec();
        let hand_type = match card_counts {
            count if count.len() == 1 => {
                HandType::FiveOfKind
            },
            count if count.len() == 2 => {
                if count[0] == 1 || count[0] == 4 {
                    HandType::FourOfKind
                } else {
                    HandType::FullHouse
                }
            },
            count if count.len() == 3 => {
                if count[0] == 3 || count[1] == 3 || count[2] == 3 {
                    HandType::ThreeOfKind
                } else {
                    HandType::TwoPair
                }
            },
            count if count.len() == 4 => {
                HandType::OnePair
            }
            _ => HandType::HighCard
        };
        hands.push(
            Hand {
                ty: hand_type,
                cards: numeric_representation_of_cards,
                bid: bid.parse().unwrap()
            },
        )
    }
    hands
}

#[aoc(day7, part1)]
fn part_one(input: &[Hand]) -> u32 {
    input
        .into_iter()
        .sorted_unstable()
        .enumerate()
        .fold(0, |value, (i, Hand { bid, .. })| {
            value + (i+1) as u32 * bid
        })
}

#[aoc_generator(day7, part2)]
fn input_generator_part_two(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in input.lines() {
        let (cards, bid) = line.split_once(' ').unwrap();
        let mut card_counts: [u8; 13] = [0; 13];
        let mut numeric_representation_of_cards = [0; 5];
        for (i, card) in cards.chars().enumerate() {
            match card {
                card if card.is_numeric() => {
                    let card_number: u8 = card.to_digit(10).unwrap() as u8;
                    numeric_representation_of_cards[i] = card_number - 1;
                    card_counts[(card_number - 1) as usize] += 1;
                },
                'A' => {
                    numeric_representation_of_cards[i] = 12;
                    card_counts[12] += 1;
                },
                'K' => {
                    numeric_representation_of_cards[i] = 11;
                    card_counts[11] += 1;
                },
                'Q' => {
                    numeric_representation_of_cards[i] = 10;
                    card_counts[10] += 1;
                },
                'J' => {
                    numeric_representation_of_cards[i] = 0;
                    card_counts[0] += 1;
                },
                'T' => {
                    numeric_representation_of_cards[i] = 9;
                    card_counts[9] += 1;
                },
                _ => unreachable!()
            }
        }
        if card_counts[0] > 0 {
            let index_of_max = card_counts
                .iter()
                .enumerate()
                .skip(1)
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(index, _)| index)
                .unwrap();
            card_counts[index_of_max] += card_counts[0];
            card_counts[0] = 0;
        }
        let card_counts = card_counts
            .into_iter()
            .filter(|count| *count > 0)
            .collect_vec();
        let hand_type = match card_counts {
            count if count.len() == 1 => {
                HandType::FiveOfKind
            },
            count if count.len() == 2 => {
                if count[0] == 1 || count[0] == 4 {
                    HandType::FourOfKind
                } else {
                    HandType::FullHouse
                }
            },
            count if count.len() == 3 => {
                if count[0] == 3 || count[1] == 3 || count[2] == 3 {
                    HandType::ThreeOfKind
                } else {
                    HandType::TwoPair
                }
            },
            count if count.len() == 4 => {
                HandType::OnePair
            }
            _ => HandType::HighCard
        };
        hands.push(
            Hand {
                ty: hand_type,
                cards: numeric_representation_of_cards,
                bid: bid.parse().unwrap()
            }
        )
    }
    hands
}

#[aoc(day7, part2)]
fn part_two(input: &[Hand]) -> u32 {
    input
        .into_iter()
        .sorted_unstable()
        .enumerate()
        .fold(0, |value, (i, Hand { bid, .. })| {
            value + (i+1) as u32 * bid
        })
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn part1_1() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};
        let result = part_one(&input_generator_part_one(input));
        assert_eq!(result, 6440);
    }

    #[test]
    fn part1_2() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
            QQQ2A 2
        "};
        let result = part_one(&input_generator_part_one(input));
        assert_eq!(result, 6933);
    }

    #[test]
    fn part2_1() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};
        let result = part_two(&input_generator_part_two(input));
        assert_eq!(result, 5905);
    }

    #[test]
    fn part2_2() {
        let input = indoc! {"
            2345A 1
            Q2KJJ 13
            Q2Q2Q 19
            T3T3J 17
            T3Q33 11
            2345J 3
            J345A 2
            32T3K 5
            T55J5 29
            KK677 7
            KTJJT 34
            QQQJA 31
            JJJJJ 37
            JAAAA 43
            AAAAJ 59
            AAAAA 61
            2AAAA 23
            2JJJJ 53
            JJJJ2 41
        "};
        let result = part_two(&input_generator_part_two(input));
        assert_eq!(result, 6839);
    }
}
