use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|l| { l.chars().map(|c| c == '1').collect::<Vec<bool>>() }).collect::<Vec<Vec<bool>>>()
}

fn get_commons(lines: &Vec<Vec<bool>>, most: bool) -> Vec<bool> {
    lines.iter().fold(vec![0; lines.first().unwrap_or(&Vec::new()).len()], | sum, l| {
        l.iter().zip(sum).map(|(b, count)| count + (*b as usize) ).collect()
    }).iter().map(|count| if most { count >= &(lines.len() - count) } else { count < &(lines.len() - count) }).collect::<Vec<bool>>()
}

fn bool_vec_to_dec(v: &Vec<bool>) -> u64 {
    v.iter().fold(0, |num, value| (num << 1) + *value as u64)
}

#[aoc(day3, part1)]
fn solve_part1(lines: &Vec<Vec<bool>>) -> u64 {
    let commons = get_commons(lines, true);
    let gamma = bool_vec_to_dec(&commons);
    let epsilon = !gamma & ((1<<commons.len()) - 1);
    
    gamma * epsilon
}

fn reduce_to_single_by_significant(lines: &Vec<Vec<bool>>, pos: usize, most: bool) -> Vec<bool> {
     let commons = get_commons(lines, most);

    let lines: Vec<Vec<bool>> = lines.to_owned().into_iter().filter(|el| el[pos] == commons[pos]).collect();
    if lines.len() == 1 { lines[0].to_owned() } else { reduce_to_single_by_significant(&lines, pos + 1, most) }
}

#[aoc(day3, part2)]
fn solve_part2(lines: &Vec<Vec<bool>>) -> u64 {
    let oxygen_rating = bool_vec_to_dec(&reduce_to_single_by_significant(&lines, 0, true));
    let co2_scrubber_rating = bool_vec_to_dec(&reduce_to_single_by_significant(&lines, 0, false));
    oxygen_rating * co2_scrubber_rating
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&vec![vec![true,true,true], vec![true,true,true]]), 0);
        assert_eq!(solve_part1(&vec![vec![false,false,false], vec![false,false,false]]), 0);
        assert_eq!(solve_part1(&vec![vec![true,false,false], vec![true,false,false]]), 12);
    }
}