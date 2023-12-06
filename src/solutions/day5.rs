use std::collections::VecDeque;
use itertools::Itertools;

use rayon::prelude::*;

#[aoc_generator(day5)]
fn input_generator(input: &str) -> (Vec<u64>, Vec<Vec<((u64, u64), u64)>>) {
    let mut lines = input.lines().peekable();


    let (_, seeds) = lines.next().unwrap().split_once(": ").unwrap();
    let source_values: Vec<u64> = seeds
        .split_whitespace()
        .map(|number| number.parse::<u64>().unwrap())
        .collect();


    lines.next(); lines.next();


    let mut value_mappings: Vec<Vec<((u64, u64), u64)>> = Vec::new();
    let mut curr_map = Vec::new();
    while let Some(line) = lines.next() {
        if line == "" || lines.peek().is_none() {
            lines.next();
            value_mappings.push(curr_map.clone());
            curr_map.clear();
            continue
        }
        let mut amounts = line.split_whitespace();
        curr_map
            .push((
                (
                    amounts.next().unwrap().parse::<u64>().unwrap(),
                    amounts.next().unwrap().parse::<u64>().unwrap()
                ),
                amounts.next().unwrap().parse::<u64>().unwrap()
            ))
    }
    (source_values, value_mappings)
}


#[aoc(day5, part1)]
fn part_one(input: &(Vec<u64>, Vec<Vec<((u64, u64), u64)>>)) -> u64 {
    let mut ids = input.0.clone();
    let mappings = &input.1;


    let mut min_id = u64::MAX;
    for id in ids.iter_mut() {
        // print!("{} -> ", id);
        for mapping in mappings.iter() {
            for ((value_start, key_start), map_len) in mapping {
                if key_start <= id && *id < key_start + map_len {
                    // print!("*");
                    let key_offset = *id - key_start;
                    *id = value_start + key_offset;
                    break
                }
            }
            // print!("{} -> ", id);
        }
        // println!("{}", id);
        if *id < min_id { min_id = *id }
    }


    min_id
}


#[aoc(day5, part1, Rayon)]
fn part_one_rayon(input: &(Vec<u64>, Vec<Vec<((u64, u64), u64)>>)) -> u64 {
    let ids = input.0.clone();
    let mappings = &input.1;


    ids
        .into_par_iter()
        .map(|mut id| {
            for mapping in mappings.iter() {
                for ((value_start, key_start), map_len) in mapping {
                    if *key_start <= id && id < key_start + map_len {
                        let key_offset = id - key_start;
                        id = value_start + key_offset;
                        break
                    }
                }
            }
            id
        })
        .min()
        .unwrap()
}


#[aoc(day5, part2)]
fn part_two(input: &(Vec<u64>, Vec<Vec<((u64, u64), u64)>>)) -> u64 {
    let id_ranges: Vec<(u64, u64)> = input.0
        .clone()
        .into_iter()
        .tuples()
        .collect();


    let mappings = &input.1;


    let mut min_id = u64::MAX;
    for (min_id_range, range_len) in id_ranges {
        for mut id in min_id_range..min_id_range+range_len {
            // print!("{} -> ", id);
            for mapping in mappings.iter() {
                for ((value_start, key_start), map_len) in mapping {
                    if *key_start <= id && id < key_start + map_len {
                        // print!("*");
                        let key_offset = id - key_start;
                        id = value_start + key_offset;
                        break
                    }
                }
                // print!("{} -> ", id);
            }
            // println!("{}", id);
            if id < min_id { min_id = id }
        }
    }


    min_id
}


#[aoc(day5, part2, Rayon)]
fn part_two_rayon(input: &(Vec<u64>, Vec<Vec<((u64, u64), u64)>>)) -> u64 {
    let id_ranges: Vec<(u64, u64)> = input.0
        .clone()
        .into_iter()
        .tuples()
        .collect();
    let mappings = &input.1;

    id_ranges
        .into_par_iter()
        .map(|(min_id_range, range_len)| {
            (min_id_range..min_id_range+range_len)
                .into_par_iter()
                .map(|mut id| {
                    for mapping in mappings.iter() {
                        for ((value_start, key_start), map_len) in mapping {
                            if *key_start <= id && id < key_start + map_len {
                                let key_offset = id - key_start;
                                id = value_start + key_offset;
                                break
                            }
                        }
                    }
                    id
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

// #[aoc(day5, part2, Optimized)]
// fn part_two_optimized(input: &(Vec<u64>, Vec<Vec<((u64, u64), u64)>>)) -> u64 {
//     let id_ranges: Vec<(u64, u64)> = input.0
//         .clone()
//         .into_iter()
//         .tuples()
//         .collect();
//     let mappings = &input.1;


//     let mut next_ranges = Vec::new();
//     for original_curr_range in curr_ranges {
//         // print!("{} -> ", source_value);
//         let mut range_splitting = VecDeque::new();
//         range_splitting.push_back(original_curr_range);
//         'curr_range: while !range_splitting.is_empty() {
//             let (curr_start, curr_len) = range_splitting.pop_front().unwrap();
//             let curr_end = curr_start + curr_len - 1;
//             for ((value_start, key_start), mapping_len) in mappings.iter() {
//                 let key_end = key_start + mapping_len - 1;
//                 if curr_start < *key_start {
//                     if curr_end < key_end {
//                         if curr_end < key_end { continue }
//                         // Case 1
//                         println!("Case 1: {}-{}", curr_start, curr_end);
//                         let left_len = key_start - curr_start;
//                         range_splitting.push_back((curr_start, left_len));
//                         next_ranges.push((*value_start, 1 + curr_end - key_start));
//                         continue 'curr_range
//                     } else if curr_end > key_end {
//                         // Case 2
//                         println!("Case 2: {}-{}", curr_start, curr_end);
//                         let left_len = key_start - curr_start;
//                         let right_len = curr_end - key_end;
//                         range_splitting.push_back((curr_start, left_len));
//                         range_splitting.push_back((key_end+1, right_len));
//                         next_ranges.push((*value_start, *mapping_len));
//                         continue 'curr_range
//                     } else {
//                         // Case 3
//                         println!("Case 3: {}-{}", curr_start, curr_end);
//                         let left_len = key_start - curr_start;
//                         range_splitting.push_back((curr_start, left_len));
//                         next_ranges.push((*value_start, *mapping_len));
//                         continue 'curr_range
//                     }
//                 } else if curr_start > *key_start {
//                     if curr_end < key_end {
//                         // Case 4
//                         println!("Case 4: {}-{}", curr_start, curr_end);
//                         next_ranges.push((*value_start+(curr_start-key_start), curr_len));
//                         continue 'curr_range
//                     } else if curr_end > key_end {
//                         if curr_end > key_end { continue }
//                         // Case 5
//                         println!("Case 5: {}-{}", curr_start, curr_end);
//                         let right_len = curr_end - key_end;
//                         range_splitting.push_back((key_end+1, right_len));
//                         next_ranges.push((*value_start + (curr_start - key_start), curr_len - right_len));
//                         continue 'curr_range
//                     } else {
//                         // Case 6
//                         println!("Case 6: {}-{}", curr_start, curr_end);
//                         next_ranges.push((*value_start + (curr_start - key_start), curr_len));
//                         continue 'curr_range
//                     }
//                 } else {
//                     if curr_end < key_end {
//                         // Case 7
//                         println!("Case 7: {}-{}", curr_start, curr_end);
//                         next_ranges.push((*value_start, 1 + key_end-curr_end));
//                         continue 'curr_range
//                     } else if curr_end > key_end {
//                         // Case 8
//                         println!("Case 8: {}-{}", curr_start, curr_end);
//                         let right_len = curr_end - key_end;
//                         range_splitting.push_back((key_end+1, right_len));
//                         next_ranges.push((*value_start, curr_len - right_len));
//                         continue 'curr_range
//                     } else {
//                         // Case 9
//                         println!("Case 9: {}-{}", curr_start, curr_end);
//                         next_ranges.push((*value_start, *mapping_len));
//                         continue 'curr_range
//                     }
//                 }
//             }
//             next_ranges.push((curr_start, curr_len))
//         }
//         // println!("{}", source_value);
//     }
//     // println!("-----------");
//     mappings.clear();
//     curr_ranges = next_ranges;
//     lines.next();
//     println!("--------");
//     continue

//         let mut amounts = line.split_whitespace();
//         mappings
//             .push((
//                 (
//                     amounts.next().unwrap().parse::<u64>().unwrap(),
//                     amounts.next().unwrap().parse::<u64>().unwrap()
//                 ),
//                 amounts.next().unwrap().parse::<u64>().unwrap()
//             ))

//     println!("{:?}", curr_ranges);
//     println!("{:?}", curr_ranges.len());
//     curr_ranges
//         .into_iter()
//         .map(|(first_index, _)| first_index)
//         .min()
//         .unwrap()
// }

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
