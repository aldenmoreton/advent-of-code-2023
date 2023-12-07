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
