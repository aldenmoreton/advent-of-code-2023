
#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<(f64, f64)> {
    let mut lines = input.lines();
    let (_, times) = lines.next().unwrap().split_once(':').unwrap();
    let (_, distances) = lines.next().unwrap().split_once(':').unwrap();


    let times: Vec<f64> = times
        .split_whitespace()
        .map(|time| time.parse().unwrap())
        .collect();


    let distances: Vec<f64> = distances
        .split_whitespace()
        .map(|distance| distance.parse().unwrap())
        .collect();


    times
        .into_iter()
        .zip(distances)
        .collect()
}


fn quadradic_range(b: f64, c: f64) -> u64 {
    let a = -1.;
    let disc = b * b - 4.0 * a * c;


    if disc > 0.0 {
        let root_a = (-b + disc.sqrt()) / (2.0 * a);
        let root_b = (-b - disc.sqrt()) / (2.0 * a);
        if root_a < root_b {
            let root_a = if root_a.fract() == 0.0 {
                root_a + 1.
            } else {
                root_a.ceil()
            } as u64;
            let root_b = if root_b.fract() == 0.0 {
                root_b - 1.
            } else {
                root_b.floor()
            } as u64;
            1 + root_b - root_a
        } else {
            root_a.floor() as u64 - root_b.ceil() as u64
        }
    } else  {
        1
    }
}


#[aoc(day6, part1)]
fn part_one(input: &[(f64, f64)]) -> u64 {
    let mut max_multiplied = 1;
    for (time, distance) in input {
        let curr_range = quadradic_range(*time, -1. * *distance);
        max_multiplied *= curr_range;
    }
    max_multiplied
}


#[aoc(day6, part2)]
fn part_two(input: &[(f64, f64)]) -> u64 {
    let input = input
        .iter()
        .fold(
            (String::new(), String::new()),
            |accumulator, (time, distance)| {
                (accumulator.0+&time.to_string(), accumulator.1+&distance.to_string())
            }
        );
    let time: f64 = input.0.parse().unwrap();
    let distance: f64 = input.1.parse().unwrap();

    quadradic_range(time, -distance)
}


#[cfg(test)]
mod tests {
    use indoc::indoc;


    use super::*;


    #[test]
    fn part1_1() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};
        let result = part_one(&input_generator(input));
        assert_eq!(result, 288);
    }


    #[test]
    fn part2_1() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};
        let result = part_two(&input_generator(input));
        assert_eq!(result, 71503);
    }
}