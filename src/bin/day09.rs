use aoc2022_rust::problem;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = problem::load(09);
    let solution_1 = solve1(&input);
    println!("Puzzle 1 {}", solution_1);
    let solution_2 = solve2(&input);
    println!("Puzzle 2 {}", solution_2);
}

#[derive(Debug)]
enum Direction {
    Right,
    Up,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    distance: i32, // -ve distance for going down/left
}

impl Move {
    fn new(direction: Direction, distance: i32) -> Self {
        Move {
            direction,
            distance,
        }
    }
}

struct Rope {
    n_knots: usize,
    knot_locations: Vec<(i32, i32)>,
    tail_visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(n_knots: i32) -> Self {
        let mut knot_locations: Vec<(i32, i32)> = vec![];
        for _ in 0..n_knots {
            knot_locations.push((0, 0));
        }
        let tail_visited = HashSet::from([knot_locations[n_knots as usize - 1]]);
        Rope {
            n_knots: n_knots as usize,
            knot_locations,
            tail_visited
        }
    }

    fn perform_move(&mut self, movement: Move) -> () {
        for _ in 0..movement.distance.abs() {
            match movement.direction {
                Direction::Right => self.knot_locations[0].0 += movement.distance.signum(),
                Direction::Up => self.knot_locations[0].1 += movement.distance.signum()
            }
            for knot in 1..self.n_knots {
                self.knot_locations[knot] = update_knot_position(self.knot_locations[knot],
                                     self.knot_locations[knot - 1])
            }
            self.update_state();
        }
    }

    fn update_state(&mut self) {
        self.tail_visited.insert(self.knot_locations[self.n_knots - 1]);
    }
}

fn update_knot_position(mut knot_update: (i32, i32), knot_follow: (i32, i32)) -> (i32, i32) {
    let x_dist = knot_follow.0 - knot_update.0;
    let y_dist = knot_follow.1 - knot_update.1;
    if x_dist.abs() <= 1 && y_dist.abs() <= 1 {
        // do nothing we're already adjacent
        return knot_update
    }
    if x_dist >= 1 {
        // The case when head is right of tail
        knot_update.0 += 1;
    } else if x_dist <= -1 {
        // The case when head is left of tail
        knot_update.0 -= 1;
    }
    if y_dist >= 1 {
        // The head is above tail
        knot_update.1 += 1;
    } else if y_dist <= -1 {
        // The head is below tail
        knot_update.1 -= 1;
    }
    knot_update
}

fn parse_moves(input: &str) -> Vec<Move> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<direction>[RLUD]) (?P<distance>\d+)").unwrap();
    }
    let mut moves: Vec<Move> = vec![];
    for capture in RE.captures_iter(input) {
        let distance = capture["distance"].parse::<i32>().unwrap();
        moves.push(match &capture["direction"] {
            "R" => Move::new(Direction::Right, distance),
            "L" => Move::new(Direction::Right, -distance),
            "U" => Move::new(Direction::Up, distance),
            "D" => Move::new(Direction::Up, -distance),
            _ => panic!("Found invalid direction"),
        })
    }
    moves
}

fn solve1(input: &str) -> usize {
    let moves: Vec<Move> = parse_moves(input);
    let mut rope: Rope = Rope::new(2);
    for movement in moves {
        rope.perform_move(movement);
    }
    return rope.tail_visited.len();
}

fn solve2(input: &str) -> usize {
    let moves: Vec<Move> = parse_moves(input);
    let mut rope: Rope = Rope::new(10);
    for movement in moves {
        rope.perform_move(movement);
    }
    return rope.tail_visited.len();
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let input = problem::load_test(09);
        let solution_1 = solve1(&input);
        assert_eq!(solution_1, 88);
        let solution_2 = solve2(&input);
        assert_eq!(solution_2, 36);
    }
}
