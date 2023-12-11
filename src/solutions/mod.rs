mod day1;
mod day2;
mod day3;
mod day4;
/// [If You Give A Seed A Fertilizer](https://adventofcode.com/2023/day/5)
/// ## Summary
/// 1. Day 5 was completed with a brute force approach.
/// This is great for part 1, but in the future I would
/// like to optimize part 2 by sending each *range* of numbers
/// through the checking loop vs each individual number.
/// 2. Parallel iteration using rayon cuts the time by a factor of 6
/// for part 2
/// ## Reflection
/// 1. This was my first day that I was unable to complete within
/// the 24 hour time span. I am already at a disadvantage, as I am not
/// usually up, moving, and working on these problems until about 10 hours
/// after the puzzle is released. Adding my stubborness to push to create
/// an optimal solution for part 2 meant that I had to set it aside and
/// come back to it in the following days. Sometimes getting an inefficient
/// correct answer is better than no answer at all.
/// 2. Rayon is the truth. It's ability to parallelize sequential processes
/// is amazing.
pub mod day5;
/// [Wait For It](https://adventofcode.com/2023/day/6)
/// ## Summary
/// Day 6 can be completed in O(n) time using quadradic functions.
/// The problem described can be set up as the following inequality:
/// ```txt
/// (t-x) * x > y
/// where x = time on button, y = time to beat
/// ```
/// ## Reflection
/// This was a much easier problem compared to day 5. It helped to
/// think this one through before starting it. As I graphed it out in
/// my head I realized that this would be a porabola where the descrete
/// points above the previous race record would be the winning options.
pub mod day6;
/// [Camel Cards](https://adventofcode.com/2023/day/7)
/// ## Summary
/// This day was a parsing puzzle. Once Card hands were parsed into
/// their correct form, they could be sorted with the derived implementations
/// of order.
/// ## Reflection
/// This was one of the problems where using structs absolutely made sense.
/// The ability to derive all of the ordering and comparision needed made this
/// one of the quickiest days to solve.
mod day7;
/// [Haunted Wasteland](https://adventofcode.com/2023/day/8)
/// ## Summary
/// Great mapping problem for part one extended into the same mapping
/// problem with the addition of some LCM math for part 2.
/// ## Reflection
/// This problem is a shining example of the power of parrellel iterators.
/// Part 2 is able to have almost the same runtime characteristics as part 1
/// because it is the part 1 problem done in parrellel and then given a small
/// calculation to fold all values.
mod day8;
/// [Mirage Maintenance](https://adventofcode.com/2023/day/9)
/// ## Summary
/// This day could be solved using iterators and folding.
/// ## Reflection
/// This was a very easy day, but was great practice for
/// using the fold method on iterators.
mod day9;
/// [Pipe Maze](https://adventofcode.com/2023/day/10)
/// ## Summary
/// Hard puzzle concerned with following directions through grid.
/// ## Reflection
/// I should have split more of this puzzle into traits and functions.
/// Programming is a struggle between monolithic blocks and fragmented
/// little structures. I definitely fell too far on the former this day.
mod day10; //TODO: Refactor and pass tests (improve cases around 'S' in part 2)
/// [Cosmic Expansion](https://adventofcode.com/2023/day/11)
/// ## Summary
/// Easy problem concerned with finding the number of times a
/// Manhattan distance crossed over certain boundaries.
mod day11;
