// use std::cmp::Ordering;

// type Tower = Vec<Box<dyn Support>>;
// type TowerIndex = usize;

// struct Coord {
//     x: usize,
//     y: usize,
//     z: usize,
// }

// enum Brick {
//     Horizontal(HorizontalBrick),
//     Vertical(VerticalBrick),
// }

// struct HorizontalBrick(Coord, Coord);
// struct VerticalBrick {
//     low: Coord,
//     high: Coord,
// }

// trait Support {
//     fn supports(&self, location: TowerIndex, tower: &Tower) -> Vec<TowerIndex>;
//     fn supported_by(&self, location: TowerIndex, tower: &Tower) -> Vec<TowerIndex>;
//     fn min_z(&self) -> usize;
//     fn has_x_y(&self, x: usize, y: usize) -> bool;
//     fn single_support(&self) -> bool;
// }

// impl Support for VerticalBrick {
//     fn supports(&self, location: TowerIndex, tower: &Tower) -> Vec<TowerIndex> {
//         let mut supported_bricks = Vec::new();
//         for (i, brick) in tower[location + 1..].iter().enumerate() {
//             if brick.has_x_y(self.high.x, self.high.y) {
//                 supported_bricks.push(i + location + 1)
//             }
//         }
//         supported_bricks
//     }

//     fn supported_by(&self, location: TowerIndex, tower: &Tower) -> Vec<TowerIndex> {
//         let mut supports_bricks = Vec::new();
//         for (i, brick) in tower[..location].iter().enumerate().rev() {
//             if brick.has_x_y(self.high.x, self.high.y) {
//                 supports_bricks.push(i);
//                 break;
//             }
//         }
//         supports_bricks
//     }

//     fn min_z(&self) -> usize {
//         self.low.z
//     }

//     fn has_x_y(&self, x: usize, y: usize) -> bool {
//         self.high.x = x && self.high.y == y
//     }

//     fn single_support(&self) -> bool {
//         true
//     }
// }

// impl Support for HorizontalBrick {
//     fn supports(&self, location: TowerIndex, tower: &Tower) -> Vec<TowerIndex> {
//         todo!()
//     }

//     fn supported_by(&self, location: TowerIndex, tower: &Tower) -> Vec<TowerIndex> {
//         todo!()
//     }

//     fn min_z(&self) -> usize {
//         self.0.z
//     }

//     fn has_x_y(&self, x: usize, y: usize) -> bool {}

//     fn single_support(&self) -> bool {
//         false
//     }
// }

// fn input_generator(input: &str) -> Vec<Brick> {
//     input
//         .lines()
//         .map(|line| {
//             let (end_1, end_2) = line.split_once('~').unwrap();
//             let mut end_1 = end_1.split(',');
//             let coord_1 = Coord {
//                 x: end_1.next().unwrap().parse().unwrap(),
//                 y: end_1.next().unwrap().parse().unwrap(),
//                 z: end_1.next().unwrap().parse().unwrap(),
//             };
//             let mut end_2 = end_2.split(',');
//             let coord_2 = Coord {
//                 x: end_2.next().unwrap().parse().unwrap(),
//                 y: end_2.next().unwrap().parse().unwrap(),
//                 z: end_2.next().unwrap().parse().unwrap(),
//             };

//             match coord_1.z.cmp(&coord_2.z) {
//                 Ordering::Less => Brick::Vertical(VerticalBrick {
//                     low: coord_1,
//                     high: coord_2,
//                 }),
//                 Ordering::Greater => Brick::Vertical(VerticalBrick {
//                     low: coord_2,
//                     high: coord_1,
//                 }),
//                 Ordering::Equal => Brick::Horizontal(HorizontalBrick(coord_1, coord_2)),
//             }
//         })
//         .collect::<Vec<_>>()
// }
// // try this with Box<dyn Support>
// #[aoc(day22, part1)]
// fn part_one(input: &str) -> usize {
//     let tower: Tower = {
//         let mut tower = input_generator(input);
//         tower.sort_by(|a, b| {
//             let a_x = match a {
//                 Brick::Horizontal(HorizontalBrick(coords, _)) => coords.z,
//                 Brick::Vertical(VerticalBrick { high: coords, .. }) => coords.z,
//             };
//             let b_x = match b {
//                 Brick::Horizontal(HorizontalBrick(coords, _)) => coords.z,
//                 Brick::Vertical(VerticalBrick { high: coords, .. }) => coords.z,
//             };
//             a_x.cmp(&b_x)
//         });
//         tower
//             .into_iter()
//             .map(|brick| match brick {
//                 Brick::Vertical(brick) => Box::new(brick) as Box<dyn Support>,
//                 Brick::Horizontal(brick) => Box::new(brick) as Box<dyn Support>,
//             })
//             .collect()
//     };

//     0
// }

// #[aoc(day22, part2)]
// fn part_two(_input: &str) -> usize {
//     0
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use indoc::indoc;

//     #[test]
//     fn part1_1() {
//         let input = indoc! {"
//             1,0,1~1,2,1
//             0,0,2~2,0,2
//             0,2,3~2,2,3
//             0,0,4~0,2,4
//             2,0,5~2,2,5
//             0,1,6~2,1,6
//             1,1,8~1,1,9
//         "};
//         let result = part_one(&input_generator(input));
//         assert_eq!(result, 5);
//     }

//     #[test]
//     fn part2_1() {
//         let input = indoc! {""};
//         let result = part_two(&input_generator(input));
//         assert_eq!(result, 0);
//     }
// }
