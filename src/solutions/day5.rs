use std::collections::VecDeque;

use itertools::Itertools;


#[aoc_generator(day5)]
fn input_generator(input: &str) -> String {
    input.into()
}

#[aoc(day5, part1)]
fn part_one(input: &str) -> u64 {
    let mut lines = input.lines().peekable();

    let (_, seeds) = lines.next().unwrap().split_once(": ").unwrap();
    let mut source_values: Vec<u64> = seeds
        .split_whitespace()
        .map(|number| number.parse::<u64>().unwrap())
        .collect();

    lines.next(); lines.next();

    let mut value_mappings: Vec<((u64, u64), u64)> = Vec::new();
    while let Some(line) = lines.next() {
        if line == "" || lines.peek().is_none() {
            // println!("{:?}", value_mappings);
            for source_value in source_values.iter_mut() {
                // print!("{} -> ", source_value);
                for ((dest_start, source_start), range_len) in value_mappings.iter() {
                    if source_start <= source_value && *source_value < source_start + range_len {
                        // print!("mapping to source starting at {} -> ", source_start);
                        let source_offset = *source_value - source_start;
                        *source_value = dest_start + source_offset;
                        break
                    }
                }
                // println!("{}", source_value);
            }
            // println!("-----------");
            value_mappings.clear();
            lines.next();
            continue
        }
        let mut amounts = line.split_whitespace();
        value_mappings
            .push((
                (
                    amounts.next().unwrap().parse::<u64>().unwrap(),
                    amounts.next().unwrap().parse::<u64>().unwrap()
                ),
                amounts.next().unwrap().parse::<u64>().unwrap()
            ))
    }

    source_values
        .into_iter()
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part_two(input: &str) -> u64 {
    let mut lines = input.lines().peekable();

    let (_, seeds) = lines.next().unwrap().split_once(": ").unwrap();
    let mut curr_ranges: Vec<(u64, u64)> = seeds
        .split_whitespace()
        .map(|number| number.parse::<u64>().unwrap())
        .tuples()
        .collect();

    lines.next(); lines.next();

    let mut value_mappings: Vec<((u64, u64), u64)> = Vec::new();
    while let Some(line) = lines.next() {
        if line == "" || lines.peek().is_none() {
            // println!("{:?}", value_mappings);
            let mut next_ranges = Vec::new();
            for original_curr_range in curr_ranges {
                // print!("{} -> ", source_value);
                let mut range_splitting = VecDeque::new();
                range_splitting.push_back(original_curr_range);
                while !range_splitting.is_empty() {
                    let (curr_start, curr_len) = range_splitting.pop_front().unwrap();
                    let curr_end = curr_start + curr_len - 1;
                    for ((value_start, key_start), mapping_len) in value_mappings.iter() {
                        let key_end = key_start + mapping_len - 1;
                        if curr_start <= *key_start {
                            if curr_end <= key_end {
                                let left_len = key_start - curr_start;
                                if left_len > 0 {
                                    range_splitting.push_back((curr_start, left_len));
                                }
                                next_ranges.push((*value_start, curr_len-left_len))
                            } else {
                                let left_len = key_start - curr_start;
                                if left_len > 0 {
                                    range_splitting.push_back((*key_start, left_len))
                                }
                                let right_len = key_end - curr_end;
                                if right_len > 0 {
                                    range_splitting.push_back((curr_end, right_len))
                                }
                                next_ranges.push((*value_start, *mapping_len))
                            }
                        } else if curr_start < key_end {
                            let right_len = curr_end - key_end;
                            if right_len > 0 {
                                range_splitting.push_back((key_end+1, right_len))
                            }
                            next_ranges.push((*value_start, *mapping_len))
                        } else {
                            next_ranges.push((curr_start, curr_len))
                        }
                    }
                }
                // println!("{}", source_value);
            }
            // println!("-----------");
            value_mappings.clear();
            curr_ranges = next_ranges;
            lines.next();
            continue
        }
        let mut amounts = line.split_whitespace();
        value_mappings
            .push((
                (
                    amounts.next().unwrap().parse::<u64>().unwrap(),
                    amounts.next().unwrap().parse::<u64>().unwrap()
                ),
                amounts.next().unwrap().parse::<u64>().unwrap()
            ))
    }
    println!("{:?}", curr_ranges);
    println!("{:?}", curr_ranges.len());
    curr_ranges
        .into_iter()
        .map(|(first_index, _)| first_index)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_1() {
        let input = indoc!{"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};
        let result = part_one(&input_generator(input));
        assert_eq!(result, 35);
    }

    #[test]
    fn part2_1() {
        let input = indoc!{"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};
        let result = part_two(&input_generator(input));
        assert_eq!(result, 46);
    }
}
