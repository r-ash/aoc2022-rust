use aoc2022_rust::problem;

fn main() {
    let input = problem::load(1);
    let solution = solve(input);
    println!("Puzzle 1: {}, puzzle 2: {}", solution.0, solution.1);
}

fn solve(input: String) -> (u32, u32) {
    let mut data = input
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    data.sort_by(|a, b| b.cmp(a));
    let solution_1 = get_highest_n(&data, 1);
    let solution_2 = get_highest_n(&data, 3);
    return (solution_1, solution_2);
}

fn get_highest_n(data: &Vec<u32>, n: usize) -> u32 {
    return data.iter().take(n).sum();
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let input = problem::load_test(1);
        let solution = solve(input);
        assert_eq!(solution.0, 24000);
        assert_eq!(solution.1, 45000);
    }
}