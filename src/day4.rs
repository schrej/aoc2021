use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

const MARK: i32 = -1;
type Board = [[i32; 5]; 5];

struct Game {
    numbers: Vec<i32>,
    boards: Vec<Board>,
}

impl Game {
    fn mark(&mut self, num: i32) -> () {
        for b in &mut self.boards {
            b.mark(num)
        }
    }

    fn winner(&self) -> Option<Board> {
        self.boards.clone().into_iter().find(|b| b.won())
    }
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Self { numbers: self.numbers.clone(), boards: self.boards.clone() }
    }
}

trait Markable {
    fn mark(&mut self, num: i32);
}

trait Winable {
    fn won(&self) -> bool;
}

trait Sumable {
    fn sum(&self) -> i32;
}

impl Markable for Board {
    fn mark(&mut self, num: i32) {
        for r in self {
            for c in r {
                if *c == num {
                    *c = MARK
                }
            }
        }
    }
}

impl Winable for Board {
    fn won(&self) -> bool {        
        // rows
        self.into_iter().map(|l| l.into_iter().all(|&f| f == MARK)).any(|l| l) ||
        // cols
        (0..5).map(|x| (0..5).all(|y| self[y][x] == MARK)).any(|r| r)
    }
}

impl Sumable for Board {
    fn sum(&self) -> i32 {
        self.into_iter()
            .fold(0, |sum, r| 
                sum + r.into_iter().fold(0, |sum, &c| if c == MARK { sum } else { sum + c })
            )
    }
}

#[aoc_generator(day4)]
fn parse_input_day4(input: &str) -> Result<Game, ParseIntError> {
    let mut iter = input.lines().into_iter();
    let numbers: Result<Vec<i32>, ParseIntError> = iter.next().unwrap().split(",").map(|n| n.parse::<i32>()).collect();

    let boards: Vec<Board> = iter
        .filter(|l| l.len() > 0)
        .filter_map(|l| l.split(" ").filter_map(|n| n.parse::<i32>().ok()).collect::<Vec<i32>>().try_into().ok())
        .collect::<Vec<[i32; 5]>>()
        .chunks(5).filter_map(|c| c.try_into().ok()).collect();

    Ok(Game {
        numbers: numbers?,
        boards: boards,
    })
}

#[aoc(day4, part1)]
fn solve_part1(game: &Game) -> i32 {
    let g = &mut game.clone();
    for n in game.numbers.clone() {
        g.mark(n);
        match g.winner() {
            Some(b) => {
                println!("{} - {:?}", n, b.won());
                return n * b.sum()
            },
            None => (),
        }
    }
    0
}

#[aoc(day4, part2)]
fn solve_part2(game: &Game) -> Option<i32> {
    let g = &mut game.clone();
    let mut winner: Option<i32> = None;
    for n in game.numbers.clone() {
        g.mark(n);
        let curr_winners = g.boards.iter().filter(|b| b.won());
        for w in curr_winners {
            winner = Some(w.sum() * n);
            println!("{:?}", winner); 
        }
        g.boards = g.boards.clone().into_iter().filter(|b| !b.won()).collect();
    }
    winner
}

#[cfg(test)]
mod tests {
    use super::{Board, Winable};

    #[test]
    fn test_won() {
        let b = [[0,0,0,0,-1],[0,0,0,0,-1],[0,0,0,0,-1],[0,0,0,0,-1],[0,0,0,0,-1]] as Board;
        assert!(b.won());
        let b = [[-1,-1,-1,-1,-1],[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0]] as Board;
        assert!(b.won());
    }
}