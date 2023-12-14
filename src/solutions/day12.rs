use std::collections::HashMap;

use itertools::{Itertools, repeat_n};
use rayon::prelude::*;

#[derive(PartialEq, Clone)]
enum Status {
    Operational,
    Broken,
    Unknown
}

#[aoc_generator(day12, part1)]
fn input_generator_one(input: &str) -> Vec<(Vec<Status>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (characters, groups) = line.split_once(' ').unwrap();

            let groups = groups
                .split(',')
                .map(|number| number.parse().unwrap())
                .collect();

            let sequence = characters
                .chars()
                .map(|character|
                    match character {
                        '.' => Status::Operational,
                        '#' => Status::Broken,
                        '?' => Status::Unknown,
                        _ => panic!("Should not have this character in input")
                    }
                )
                .collect();

            (sequence, groups)
        })
        .collect()
}

fn validate_sequence(sequence: Vec<Status>, damage_groups: &Vec<usize>) -> bool {
    let potential_groups = sequence
        .into_iter()
        .enumerate()
        .group_by(|(_, status)| (*status).clone())
        .into_iter()
        .filter_map(|(key, grouping)|
            if key == Status::Broken {
                Some(
                    grouping
                        .collect_vec()
                        .len()
                )
            } else {
                None
            }
        )
        .collect_vec();

    if potential_groups.len() == damage_groups.len() {
        for (proposed, actual) in potential_groups.into_iter().zip(damage_groups) {
            if proposed != *actual {
                return false
            }
        }
        true
    } else {
        false
    }
}

#[aoc(day12, part1)]
fn part_one(input: &[(Vec<Status>, Vec<usize>)]) -> usize {
    let options = vec![Status::Broken, Status::Operational];


    input
        .into_par_iter()
        .map(|(sequence, damage_groups)| {
            let unknown_count = sequence
                .iter()
                .filter(|ele| **ele == Status::Unknown)
                .count();

            repeat_n(&options, unknown_count)
                .multi_cartesian_product()
                .par_bridge()
                .filter(|posible_statuses| {
                    let mut posible_statuses = posible_statuses.into_iter();

                    let sequence: Vec<_> = sequence
                        .iter()
                        .map(|ele| if *ele == Status::Unknown {
                            (**posible_statuses.next().unwrap()).clone()
                        } else {
                            (*ele).clone()
                        })
                        .collect();

                    validate_sequence(sequence, damage_groups)
                })
                .count()
        })
        .sum()
}

#[aoc_generator(day12, part2)]
fn input_generator_two(input: &str) -> Vec<(Vec<Status>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (characters, groups) = line.split_once(' ').unwrap();
            let characters = (0..5).map(|_| characters).join("?");

            let groups: Vec<_> = groups
                .split(',')
                .map(|number| number.parse::<usize>().unwrap())
                .collect();

            let sequence = characters
                .chars()
                .map(|character|
                    match character {
                        '.' => Status::Operational,
                        '#' => Status::Broken,
                        '?' => Status::Unknown,
                        _ => panic!("Should not have this character in input")
                    }
                )
                .collect();

            let groups = repeat_n(groups, 5)
                .flatten()
                .collect();

            (sequence, groups)
        })
        .collect()
}

fn possible_ways(cache: &mut HashMap<(usize, usize, usize), usize>, s: &[Status], within: Option<usize>, remaining: &[usize]) -> usize {
    if s.is_empty() {
      return match (within, remaining.len()) {
        (None, 0) => 1,
        (Some(x), 1) if x == remaining[0] => 1,
        _ => 0
      };
    }
    if within.is_some() && remaining.is_empty() {
      return 0;
    }

    let key = (s.len(), within.unwrap_or(0), remaining.len());
    if let Some(&x) = cache.get(&key) {
      return x;
    }

    let ways = match (s[0].clone(), within) {
      (Status::Operational, Some(x)) if x != remaining[0] => 0,
      (Status::Operational, Some(_)) => possible_ways(cache, &s[1..], None, &remaining[1..]),
      (Status::Operational, None)    => possible_ways(cache, &s[1..], None, remaining),
      (Status::Broken, Some(_)) => possible_ways(cache, &s[1..], within.map(|x| x+1), remaining),
      (Status::Broken, None)    => possible_ways(cache, &s[1..], Some(1), remaining),
      (Status::Unknown, Some(x)) => {
        let mut ans = possible_ways(cache, &s[1..], within.map(|x| x+1), remaining);
        if x == remaining[0] {
          ans += possible_ways(cache, &s[1..], None, &remaining[1..])
        }
        ans
      }
      (Status::Unknown, None) =>
        possible_ways(cache, &s[1..], Some(1), remaining) +
        possible_ways(cache, &s[1..], None, remaining),
    };
    cache.insert(key, ways);
    ways
}

#[aoc(day12, part2)]
fn part_two(input: &[(Vec<Status>, Vec<usize>)]) -> usize {
    let mut cache =  HashMap::new();

    input
        .into_iter()
        .map(|(raw_sequence, damage_groups)| {
            let sum = possible_ways(&mut cache, raw_sequence, None, damage_groups);
            cache.clear();
            sum
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
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};
        let result = part_one(&input_generator_one(input));
        assert_eq!(result, 21);
    }


    #[test]
    fn part2_1() {
        let input = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};
        let result = part_two(&input_generator_two(input));
        assert_eq!(result, 525152);
    }

    #[test]
    fn part2_2() {
        let input = indoc! {"
            ???.### 1,1,3
        "};
        let result = part_two(&input_generator_two(input));
        assert_eq!(result, 1);
    }

    #[test]
    fn part2_3() {
        let input = indoc! {"
            .??..??...?##. 1,1,3
        "};
        let result = part_two(&input_generator_two(input));
        assert_eq!(result, 16384);
    }

    #[test]
    fn part2_4() {
        let input = indoc! {"
            ?#?#?#?#?#?#?#? 1,3,1,6
        "};
        let result = part_two(&input_generator_two(input));
        assert_eq!(result, 1);
    }

    #[test]
    fn part2_5() {
        let input = indoc! {"
            ????.#...#... 4,1,1
        "};
        let result = part_two(&input_generator_two(input));
        assert_eq!(result, 16);
    }

    #[test]
    fn part2_6() {
        let input = indoc! {"
            ????.######..#####. 1,6,5
        "};
        let result = part_two(&input_generator_two(input));
        assert_eq!(result, 2500);
    }

    #[test]
    fn part2_7() {
        let input = indoc! {"
            ?###???????? 3,2,1
        "};
        let result = part_two(&input_generator_two(input));
        assert_eq!(result, 506250);
    }
}
