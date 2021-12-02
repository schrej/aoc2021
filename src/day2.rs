use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

enum Instruction {
    Up,
    Down,
    Forward,
    Unknown
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Result<Vec<(Instruction, i32)>, ParseIntError> {
    input.lines().map(|l| -> Result<(Instruction, i32), ParseIntError> {
        let parts = l.split(" ").collect::<Vec<&str>>();
        let instr = match parts[0] {
            "up" => Instruction::Up,
            "down" => Instruction::Down,
            "forward" => Instruction::Forward,
            &_ => Instruction::Unknown
        };
        let amount = parts[1].parse()?;
        return Ok((instr, amount));
    }).collect()
}

#[aoc(day2, part1)]
fn solve_part1(instructions: &[(Instruction, i32)]) -> i32 {
    let (horizontal, depth) = instructions.iter()
        .fold((0, 0), | (horizontal, depth), (instr, value)| match &instr {
            Instruction::Up => (horizontal, depth - value ),
            Instruction::Down => (horizontal, depth + value),
            Instruction::Forward => (horizontal + value, depth),
            &Instruction::Unknown => (horizontal, depth)
        });
    horizontal * depth
}

struct Position {
    depth: i32,
    aim: i32,
    horizontal: i32
}

#[aoc(day2, part2)]
fn solve_part2(instructions: &[(Instruction, i32)]) -> i32 {
    let final_pos = instructions.iter()
        .fold(Position {depth: 0,aim: 0, horizontal: 0}, | mut acc, instr| {
            match &instr.0 {
                Instruction::Up => { acc.aim -= instr.1 },
                Instruction::Down => { acc.aim += instr.1},
                Instruction::Forward => {
                    acc.horizontal += instr.1; 
                    acc.depth += acc.aim * instr.1;
                },
                &Instruction::Unknown => {}
            }
            acc
        });
    final_pos.depth * final_pos.horizontal
}

// #[cfg(test)]
// mod tests {
//     use day2;
// }