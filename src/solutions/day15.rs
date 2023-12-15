type OurMap = Vec<Vec<(String, usize)>>;

fn our_hash(input: &str) -> usize {
    let mut hash = 0;
    for character in input.chars() {
        hash += character as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}

#[aoc(day15, part1)]
fn part_one(input: &str) -> usize {
    input
        .split(',')
        .map(our_hash)
        .sum()
}

fn delete(key: &str, map: &mut OurMap) -> bool {
    let index = our_hash(key);
    let position = map[index].iter().position(|(curr_key, _)| *curr_key == key);

    if let Some(position) = position {
        map[index].remove(position);
        true
    } else {
        false
    }
}

fn upsert(input: &str, map: &mut OurMap) -> bool {
    let (key, value) = input.split_once('=').unwrap();
    let index = our_hash(key);
    let position = map[index].iter().position(|(curr_key, _)| *curr_key == key);

    if let Some(position) = position {
        map[index][position] = (key.into(), value.parse().unwrap());
        true
    } else {
        map[index].push((key.into(), value.parse().unwrap()));
        false
    }
}


#[aoc(day15, part2)]
fn part_two(input: &str) -> usize {
    let mut map = vec![Vec::<(String, usize)>::new(); 256];
    input
        .split(',')
        .for_each(|element|
            if element.contains('-') {
                delete(&element.replace('-', ""), &mut map);
            } else {
                upsert(element, &mut map);
            }
        );

    map
        .iter()
        .enumerate()
        .map(|(box_num, ele)|
            ele
                .iter()
                .enumerate()
                .map(|(slot_num, (_, focal_len))|
                    (box_num+1) * (slot_num+1) * focal_len
                )
                .sum::<usize>()
        )
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input = "HASH";
        let result = part_one(input);
        assert_eq!(result, 52);
    }

    #[test]
    fn part1_2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = part_one(input);
        assert_eq!(result, 1320);
    }

    #[test]
    fn part2_1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = part_two(input);
        assert_eq!(result, 145);
    }
}
