use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(depths: &[i32]) -> usize {
    return depths.windows(2).filter(|w| w[0] < w[1]).count();
}

#[aoc(day1, part2)]
fn solve_part2(depths: &[i32]) -> usize {
    let windowed_depths = depths
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<i32>>();
    solve_part1(&windowed_depths)
}

// #[cfg(test)]
// mod tests {
// }