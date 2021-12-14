use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day7)]
fn parse_input_day7(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().next().unwrap().split(",").map(|n| n.parse()).collect()

}

#[aoc(day7, part1)]
fn solve_part1(crabs: &Vec<u32>) -> i32 {
    let mut crabs = crabs.to_owned();
    crabs.sort_unstable();
    let med_pos = crabs[crabs.len()/2];
    crabs.into_iter().map(|c| (med_pos as i32 - c as i32).abs()).sum()
}

#[aoc(day7, part2)]
fn solve_part2(crabs: &Vec<u32>) -> i32 {
//     let mut crabs = crabs.to_owned();
//     crabs.sort_unstable();
    let avg_pos = (crabs.into_iter().sum::<u32>() as f32 / crabs.len() as f32) as i32;
    (avg_pos-1 ..= avg_pos + 1).map(|p|
        crabs.into_iter().map(|&c| (0..=(p - c as i32).abs()).sum::<i32>()).sum()
    ).min().unwrap()
}
// 97164405 is wrong

#[cfg(test)]
mod tests {
    use crate::day7::solve_part2;

    use super::solve_part1;

    const INPUT: &[u32] = &[16,1,2,0,4,2,7,1,2,14];

    #[test]
    fn test_part1() {
        assert_eq!(37, solve_part1(&INPUT.to_vec()))
    }

    #[test]
    fn test_part2() {
        assert_eq!(168, solve_part2(&INPUT.to_vec()))
    }
}