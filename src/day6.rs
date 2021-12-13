use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day6)]
fn parse_input_day6(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.lines().next().unwrap().split(",").map(|n| n.parse()).collect()
}

fn simulate(init: &Vec<usize>, generations: u32) -> usize {
    let mut fish = init.into_iter().fold([0; 9], |mut counts, &f| { counts[f]+=1; counts });

    for _ in 0 .. generations {
        let spawns = fish[0];

        for i in 0..8 {
            fish[i] = fish[i+1]
        }
        fish[6] += spawns;
        fish[8] = spawns;
    }
    fish.into_iter().sum()
}

#[aoc(day6, part1)]
fn solve_part1(initial_fish: &Vec<usize>) -> usize {
    simulate(initial_fish, 80)
}

#[aoc(day6, part2)]
fn solve_part2(initial_fish: &Vec<usize>) -> usize {
    simulate(initial_fish, 256)
}

#[cfg(test)]
mod tests {
    use super::solve_part1;

    #[test]
    fn test_part1() {
        assert_eq!(5934, solve_part1(&vec![3,4,3,1,2]))
    }
}