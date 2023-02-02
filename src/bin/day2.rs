use aoc2022_rust::problem;
use std::collections::HashMap;

fn main() {
    let input = problem::load(2);
    let solution_1 = solve1(&input);
    println!("Puzzle 1 {}", solution_1);
    let solution_2 = solve2(&input);
    println!("Puzzle 2 {}", solution_2);
}

fn solve1(input: &str) -> u32 {
    let turns = parse_input(input);

    let mut total_score = 0;
    for turn in turns {
        total_score += score_outcome(turn.0, turn.1);
        total_score += score_value(turn.1);
    }
    return total_score;
}

fn solve2(input: &str) -> u32 {
    let turns = parse_input(input);
    let mut total_score = 0;
    for turn in turns {
        total_score += turn.1;
        total_score += score_my_value(turn.0, turn.1);
    }
    return total_score;
}

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    let mut opponent_choice = HashMap::<&str, u32>::new();
    opponent_choice.insert("A", 0);
    opponent_choice.insert("B", 1);
    opponent_choice.insert("C", 2);

    let mut outcome = HashMap::<&str, u32>::new();
    outcome.insert("X", 0);
    outcome.insert("Y", 3);
    outcome.insert("Z", 6);

    return input
        .lines()
        .map(|x| {
            let mut split = x.split_whitespace();
            return (*opponent_choice.get(split.next().unwrap()).unwrap(),
                    *outcome.get(split.next().unwrap()).unwrap())
        }).collect();
}

fn score_outcome(opponent: u32, mine: u32) -> u32 {
    // With rock = 0, paper = 1, scissors = 2
    // then if mine == opponent + 1 (mod 3) then I win
    return if mine == opponent {
        3
    } else if mine == (opponent + 1) % 3 {
        6
    } else {
        0
    }
}

fn score_value(mine: u32) -> u32 {
    return mine + 1;
}

fn score_my_value(opponent: u32, outcome: u32) -> u32 {
    match outcome {
        0 => return score_value((opponent + 2) % 3),
        3 => return score_value(opponent),
        6 => return score_value((opponent + 1) % 3),
        _ => panic!("invalid value")
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let input = problem::load_test(2);
        let solution_1 = solve1(&input);
        assert_eq!(solution_1, 15);
        let solution_2 = solve2(&input);
        assert_eq!(solution_2, 12);
    }
}