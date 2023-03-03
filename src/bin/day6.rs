use aoc2022_rust::problem;
use std::collections::HashSet;

fn main() {
    let input = problem::load(6);
    let solution_1 = solve1(&input);
    println!("Puzzle 1 {}", solution_1);
    let solution_2 = solve2(&input);
    println!("Puzzle 2 {}", solution_2);
}


fn characters_unique(characters: &str, n: usize) -> bool {
    let chars: HashSet<char> = characters.chars().collect();
    chars.len() == n
}

fn solve(input: &str, n: usize) -> usize{
    for x in n..input.len() {
        if characters_unique(&input[(x-n)..x], n) {
            return x
        }
    }
    return 0
}


fn solve1(input: &str) -> usize {
    solve(input, 4)
}

fn solve2(input: &str) -> usize {
    solve(input, 14)
}


#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let input = problem::load_test(6);
        let solution_1 = solve1(&input);
        assert_eq!(solution_1, 7);
        let solution_2 = solve2(&input);
        assert_eq!(solution_2, 19);
    }
}