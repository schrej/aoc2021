use aoc_runner_derive::{aoc, aoc_generator};
use std::{vec};

#[derive(Debug, Clone, Copy)]
struct Point { x: usize, y: usize }

type Line = (Point, Point);

type Map = Vec<Vec<u32>>;

trait Maplike {
    fn mark(&mut self, x: usize, y: usize);
    fn mark_line(&mut self, line: &Line);
}

impl Maplike for Map {
    fn mark(&mut self, x: usize, y: usize) {
        if self.len() <= x {
            self.resize(x+1, vec![]);
        }
        if self[x].len() <= y {
            self[x].resize(y+1, 0);
        }
        self[x][y] += 1;
    }

    fn mark_line(&mut self, line: &Line) {
        let x_range: Vec<usize> = if line.0.x < line.1.x { (line.0.x ..= line.1.x).collect() } else { (line.1.x ..= line.0.x).rev().collect() };
        let y_range: Vec<usize> = if line.0.y < line.1.y { (line.0.y ..= line.1.y).collect() } else { (line.1.y ..= line.0.y).rev().collect() };

        if x_range.len() == y_range.len() {
            for (x, y) in x_range.into_iter().zip(y_range) {
                self.mark(x, y);
            }
        } else {
            for x in x_range {
                for y in y_range.clone() {
                    self.mark(x, y);
                }
            }
        }
    }
}

fn fill_map_and_count(lines: &Vec<Line>) -> usize {
    let mut map = vec![];
    lines.into_iter().for_each(|l| map.mark_line(l));
    map.into_iter()
        .map(|l| l.into_iter().filter(|&n| n > 1).count())
        .sum()
}

#[aoc_generator(day5)]
fn parse_input_day5(input: &str) -> Vec<Line> {
    input.lines()
        .into_iter()
        .map(|l|  {
                let points = l.split(" -> ").map(|p| {
                    let coords = p.split(",").filter_map(|n| n.parse().ok()).collect::<Vec<usize>>();
                    Point {x: coords[0], y: coords[1]}
                }).collect::<Vec<Point>>();
                (points[0], points[1]) as Line
        }).collect::<Vec<Line>>()
}

#[aoc(day5, part1)]
fn solve_part1(lines: &Vec<Line>) -> usize {
    let lines = lines.to_owned().into_iter().filter(|l| l.0.x == l.1.x || l.0.y == l.1.y).collect::<Vec<Line>>();
    fill_map_and_count(&lines)
}

#[aoc(day5, part2)]
fn solve_part2(lines: &Vec<Line>) -> usize {
    fill_map_and_count(lines)
}

// #[cfg(test)]
// mod tests {
//     use template;
// }