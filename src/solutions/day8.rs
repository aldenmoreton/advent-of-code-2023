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
    while let Some(line) = lines.next() {
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
fn part_one(input: &(String, HashMap<String, (String, String)>)) -> usize {
    input.0
        .chars()
        .cycle()
        .enumerate()
        .fold_while(("AAA", 0), |curr_location, direction| {
            let next_location = if direction.1 == 'L' {
                &input.1.get(curr_location.0).unwrap().0
            } else if direction.1 == 'R' {
                &input.1.get(curr_location.0).unwrap().1
            } else {
                panic!("This isn't a direction")
            };
            if next_location == "ZZZ" {
                FoldWhile::Done(("", direction.0 + 1))
            } else {
                FoldWhile::Continue((next_location, 0))
            }
        })
        .into_inner().1
}

#[aoc(day8, part2)]
fn part_two(input: &(String, HashMap<String, (String, String)>)) -> u64 {
    let paths: Vec<_> = input.1
        .keys()
        .filter(|node| node.ends_with('A'))
        .collect();

    paths
        .into_par_iter()
        .map(|node| {
            let (_, count) = input.0
                .chars()
                .cycle()
                .fold_while((node, 0u64), |(src, count), dir| {
                    let (left, right) = input.1.get(src).unwrap();
                    let dst = match dir {
                        'L' => left,
                        'R' => right,
                        _ => unreachable!(),
                    };
                    if !dst.ends_with('Z') {
                        FoldWhile::Continue((&dst, count + 1))
                    } else {
                        FoldWhile::Done((&dst, count + 1))
                    }
                })
                .into_inner();
            count
        })
        .reduce(|| 1u64, |a, b| lcm(a, b))
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
