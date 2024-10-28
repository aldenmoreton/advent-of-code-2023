use std::collections::HashSet;

fn next_visits(
    current: (usize, usize),
    grid: &Vec<Vec<char>>,
    visited: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    match grid[current.0][current.1] {
        '^' => vec![(current.0 - 1, current.1)],
        'v' => vec![(current.0 + 1, current.1)],
        '<' => vec![(current.0, current.1 - 1)],
        '>' => vec![(current.0, current.1 + 1)],
        '.' => vec![
            (current.0 - 1, current.1),
            (current.0 + 1, current.1),
            (current.0, current.1 - 1),
            (current.0, current.1 + 1),
        ],
        '#' => panic!("We should never be on a #"),
        _ => unreachable!(),
    }
    .into_iter()
    .filter(|location| {
        let existing_non_wall = grid
            .get(location.0)
            .and_then(|y| y.get(location.1))
            .map(|tile| *tile != '#')
            .unwrap_or(false);

        existing_non_wall && !visited.contains(location)
    })
    .collect()
}

fn next_visits_part_two(
    current: (usize, usize),
    grid: &Vec<Vec<char>>,
    visited: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    match grid[current.0][current.1] {
        '^' | 'v' | '<' | '>' | '.' => vec![
            (current.0 - 1, current.1),
            (current.0 + 1, current.1),
            (current.0, current.1 - 1),
            (current.0, current.1 + 1),
        ],
        '#' => panic!("We should never be on a #"),
        _ => unreachable!(),
    }
    .into_iter()
    .filter(|location| {
        let existing_non_wall = grid
            .get(location.0)
            .and_then(|y| y.get(location.1))
            .map(|tile| *tile != '#')
            .unwrap_or(false);

        existing_non_wall && !visited.contains(location)
    })
    .collect()
}

#[aoc(day23, part1)]
fn part_one(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let bottom_row = grid.len() - 1;

    let mut diverging_paths = Vec::new();
    diverging_paths.push((
        (0, grid[0].iter().position(|c| *c == '.').unwrap()),
        HashSet::new(),
    ));

    let mut max_path = 0;
    let mut paths_checked = 0;

    while let Some((mut current, mut visited)) = diverging_paths.pop() {
        paths_checked += 1;
        loop {
            visited.insert(current);

            if current.0 == bottom_row {
                max_path = max_path.max(visited.len() - 1);
                break;
            }

            let mut next_visits = next_visits(current, &grid, &visited);

            if let Some(next) = next_visits.pop() {
                current = next;
                diverging_paths.extend(next_visits.into_iter().map(|n| (n, visited.clone())));
            } else {
                break;
            };
        }
    }

    println!("Paths checked: {paths_checked}");

    max_path
}

#[aoc(day23, part2)]
fn part_two(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let bottom_row = grid.len() - 1;

    let mut diverging_paths = Vec::new();
    diverging_paths.push((
        (0, grid[0].iter().position(|c| *c == '.').unwrap()),
        HashSet::new(),
    ));

    let mut max_path = 0;
    let mut paths_checked = 0;

    while let Some((mut current, mut visited)) = diverging_paths.pop() {
        paths_checked += 1;
        loop {
            visited.insert(current);

            if current.0 == bottom_row {
                max_path = max_path.max(visited.len() - 1);
                break;
            }

            let mut next_visits = next_visits_part_two(current, &grid, &visited);

            if let Some(next) = next_visits.pop() {
                current = next;
                diverging_paths.extend(next_visits.into_iter().map(|n| (n, visited.clone())));
            } else {
                break;
            };
        }
    }

    println!("Paths checked: {paths_checked}");

    max_path
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_1() {
        let input = indoc! {r#"
            #.#####################
			#.......#########...###
			#######.#########.#.###
			###.....#.>.>.###.#.###
			###v#####.#v#.###.#.###
			###.>...#.#.#.....#...#
			###v###.#.#.#########.#
			###...#.#.#.......#...#
			#####.#.#.#######.#.###
			#.....#.#.#.......#...#
			#.#####.#.#.#########v#
			#.#...#...#...###...>.#
			#.#.#v#######v###.###v#
			#...#.>.#...>.>.#.###.#
			#####v#.#.###v#.#.###.#
			#.....#...#...#.#.#...#
			#.#########.###.#.#.###
			#...###...#...#...#.###
			###.###.#.###v#####v###
			#...#...#.#.>.>.#.>.###
			#.###.###.#.###.#.#v###
			#.....###...###...#...#
			#####################.#"#};
        let result = part_one(&input);
        assert_eq!(result, 94);
    }

    #[test]
    fn part2_1() {
        let input = indoc! {r#"
            #.#####################
			#.......#########...###
			#######.#########.#.###
			###.....#.>.>.###.#.###
			###v#####.#v#.###.#.###
			###.>...#.#.#.....#...#
			###v###.#.#.#########.#
			###...#.#.#.......#...#
			#####.#.#.#######.#.###
			#.....#.#.#.......#...#
			#.#####.#.#.#########v#
			#.#...#...#...###...>.#
			#.#.#v#######v###.###v#
			#...#.>.#...>.>.#.###.#
			#####v#.#.###v#.#.###.#
			#.....#...#...#.#.#...#
			#.#########.###.#.#.###
			#...###...#...#...#.###
			###.###.#.###v#####v###
			#...#...#.#.>.>.#.>.###
			#.###.###.#.###.#.#v###
			#.....###...###...#...#
			#####################.#"#};
        let result = part_two(&input);
        assert_eq!(result, 154);
    }
}
