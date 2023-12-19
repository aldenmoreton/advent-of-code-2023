use std::collections::HashMap;
use std::cmp::Ordering;
use std::ops::RangeInclusive;

type Workflows = HashMap<String, Vec<Operation>>;
type Parts = Vec<Vec<usize>>;

enum Operation {
    Inequality(usize, Ordering, usize, Location),
    Branch(Location)
}

enum Location {
    Rule(String),
    Accept,
    Reject
}

#[aoc_generator(day19)]
fn input_generator(input: &str) -> (Workflows, Parts) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|workflow| {
            let (key, values) = workflow.split_once('{').unwrap();

            let values = values.replace('}', "");
            let values = values
                .split(',')
                .map(|value|
                    if let Some((ineqality, result)) = value.split_once(':') {
                        let result = match result {
                            "A" => Location::Accept,
                            "R" => Location::Reject,
                            _ => Location::Rule(result.into())
                        };
                        if let Some((variable, number)) = ineqality.split_once('<') {
                            let variable = match variable {
                                "x" => 0,
                                "m" => 1,
                                "a" => 2,
                                "s" => 3,
                                _ => panic!("Not a valid category")
                            };
                            let number: usize = number.parse().unwrap();
                            Operation::Inequality(variable, Ordering::Less, number, result)
                        } else {
                            let (variable, number) = ineqality.split_once('>').unwrap();
                            let variable = match variable {
                                "x" => 0,
                                "m" => 1,
                                "a" => 2,
                                "s" => 3,
                                _ => panic!("Not a valid category")
                            };
                            let number: usize = number.parse().unwrap();
                            Operation::Inequality(variable, Ordering::Greater, number, result)
                        }
                    } else {
                        let result = match value {
                            "A" => Location::Accept,
                            "R" => Location::Reject,
                            _ => Location::Rule(value.into())
                        };
                        Operation::Branch(result)
                    }
                )
                .collect::<Vec<_>>();

            (key.to_string(), values)
        })
        .collect::<HashMap<_, _>>();

    let parts = parts
        .lines()
        .map(|line|
            line
                .split(',')
                .map(|category|
                    category
                        .chars()
                        .filter(|character| character.is_numeric())
                        .fold(String::new(), |mut acc, character| { acc.push(character); acc })
                        .parse::<usize>().unwrap()
                )
                .collect()
        )
        .collect::<Vec<Vec<usize>>>();

    (workflows, parts)
}

#[aoc(day19, part1)]
fn part_one((workflows, parts): &(Workflows, Parts)) -> usize {
    parts
        .into_iter()
        .filter(|part| {
            let mut curr_rule = workflows.get("in".into()).unwrap();
            'rules: loop {
                'rule: for rule in curr_rule {
                    match rule {
                        Operation::Inequality(index, ordering, value, next_location) => {
                            if part[*index].cmp(value) == *ordering {
                                match next_location {
                                    Location::Rule(next_rule) => { curr_rule = workflows.get(next_rule).unwrap(); break 'rule },
                                    Location::Reject => break 'rules false,
                                    Location::Accept => break 'rules true
                                }
                            }
                        },
                        Operation::Branch(next_location) => {
                            match next_location {
                                Location::Rule(next_rule) => { curr_rule = workflows.get(next_rule).unwrap(); break 'rule },
                                Location::Reject => break 'rules false,
                                Location::Accept => break 'rules true
                            }
                        }
                    }
                }
            }
        })
        .fold(0, |acc, part| acc + part.into_iter().sum::<usize>())
}

fn find_ranges(rule: &str, mut ranges: [RangeInclusive<usize>; 4], workflows: &Workflows) -> Vec<[RangeInclusive<usize>; 4]> {
    let rule = workflows.get(rule).unwrap();

    let mut valid_ranges = Vec::new();
    for operation in rule {
        match operation {
            Operation::Branch(location) => {
                match location {
                    Location::Accept => valid_ranges.push(ranges.clone()),
                    Location::Reject => (),
                    Location::Rule(new_rule) => valid_ranges.append(&mut find_ranges(new_rule, ranges.clone(), workflows))
                }
                return valid_ranges
            }
            Operation::Inequality(index, order, value, location) => {
                let critical_value = match order {
                    Ordering::Less => ranges[*index].end(),
                    Ordering::Greater => ranges[*index].start(),
                    Ordering::Equal => panic!("We should never be equal")
                };
                if critical_value.cmp(value) == *order {
                    match location {
                        Location::Accept => valid_ranges.push(ranges.clone()),
                        Location::Reject => (),
                        Location::Rule(new_rule) => valid_ranges.append(&mut find_ranges(new_rule, ranges.clone(), workflows))
                    }
                    return valid_ranges
                } else if ranges[*index].contains(value) {
                    let mut split_ranges = ranges.clone();
                    let lower_value = ranges[*index].start();
                    let upper_value = ranges[*index].end();

                    (split_ranges[*index], ranges[*index]) = match order {
                        Ordering::Less => ((*lower_value..=*value-1), (*value..=*upper_value)),
                        Ordering::Greater => ((*value+1..=*upper_value), (*lower_value..=*value)),
                        Ordering::Equal => panic!("We should never have this")
                    };

                    match location {
                        Location::Accept => valid_ranges.push(split_ranges),
                        Location::Reject => (),
                        Location::Rule(new_rule) => valid_ranges.append(&mut find_ranges(new_rule, split_ranges, workflows))
                    }
                }
            }
        }
    }

    valid_ranges
}

#[aoc(day19, part2)]
fn part_two((workflows, _): &(Workflows, Parts)) -> usize {
    let ranges_list = find_ranges("in", [(1..=4_000), (1..=4_000), (1..=4_000), (1..=4_000)], workflows);
    println!("{}", ranges_list.len());

    ranges_list
        .into_iter()
        .map(|ranges|
            ranges
                .into_iter()
                .map(|range| {
                    let (min, max) = range.into_inner();
                    1 + max - min
                })
                .product::<usize>()
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_1() {
        let input = indoc!{"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        "};
        let result = part_one(&input_generator(input));
        assert_eq!(result, 19114);
    }

    #[test]
    fn part2_1() {
        let input = indoc!{"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        "};
        let result = part_two(&input_generator(input));
        assert_eq!(result, 167_409_079_868_000);
    }
}
