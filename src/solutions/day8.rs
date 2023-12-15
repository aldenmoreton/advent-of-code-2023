use std::collections::HashMap;

use itertools::{
    FoldWhile,
    Itertools,
};

use rayon::prelude::*;
use num::integer::lcm;

#[aoc_generator(day8)]
fn input_generator(input: &str) -> (String, HashMap<String, (String, String)>) {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().to_owned();
    lines.next();

    let mut mappings = HashMap::new();
    for line in lines {
        let (key, values) = line.split_once(" = ").unwrap();
        let (left, right) = values.split_once(", ").unwrap();
        mappings.insert(
            key.to_owned(),
            (
                left.replace('(', ""),
                right.replace(')', "")
            )
        );
    }

    (directions, mappings)
}

#[aoc(day8, part1)]
fn part_one(
    (
        directions,
        mappings
    ): &(String, HashMap<String, (String, String)>)
) -> usize {
    directions
        .chars()
        .cycle()
        .fold_while(("AAA", 0), |(curr_location, count), direction| {
            let next_location = if direction == 'L' {
                &mappings.get(curr_location).unwrap().0
            } else if direction == 'R' {
                &mappings.get(curr_location).unwrap().1
            } else {
                unreachable!()
            };
            if next_location == "ZZZ" {
                FoldWhile::Done(("", count + 1))
            } else {
                FoldWhile::Continue((next_location, count + 1))
            }
        })
        .into_inner()
        .1
}

#[aoc(day8, part2)]
fn part_two(
    (
        directions,
        mappings): &(String, HashMap<String, (String, String)>
    )
) -> u64 {
    mappings
        .keys()
        .filter(|node| node.ends_with('A'))
        .collect_vec()
        .into_par_iter()
        .map(|starting_point| {
            directions
                .chars()
                .cycle()
                .fold_while((starting_point, 0u64), |(curr_location, count), direction| {
                    let next_location = if direction == 'L' {
                        &mappings.get(curr_location).unwrap().0
                    } else if direction == 'R' {
                        &mappings.get(curr_location).unwrap().1
                    } else {
                        unreachable!()
                    };
                    if !next_location.ends_with('Z') {
                        FoldWhile::Continue((next_location, count + 1))
                    } else {
                        FoldWhile::Done((next_location, count + 1))
                    }
                })
                .into_inner()
                .1
        })
        .reduce(|| 1u64, lcm)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn part1_1() {
        let input = indoc! {"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "};
        let result = part_one(&input_generator(input));
        assert_eq!(result, 2);
    }

    #[test]
    fn part1_2() {
        let input = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};
        let result = part_one(&input_generator(input));
        assert_eq!(result, 6);
    }

    #[test]
    fn part2_1() {
        let input = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};
        let result = part_two(&input_generator(input));
        assert_eq!(result, 6);
    }
}
