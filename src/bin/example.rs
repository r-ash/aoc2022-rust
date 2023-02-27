use aoc2022_rust::problem;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = problem::load(5);
    let solution_1 = solve1(&input);
    println!("Puzzle 1 {}", solution_1);
    // let solution_2 = solve2(&input);
    // println!("Puzzle 2 {}", solution_2);
}

struct Stack<T> {
    stack: Vec<T>
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

struct Stacks {
    stacks: Vec<Stack<char>>
}

impl Stacks {
    fn tops(&self) -> String {
        let mut ret = "".to_string();
        for stack in self.stacks {
          ret.push(*stack.peek().unwrap());
        }
        ret
    }

    fn move_crates(&self, mov: Movement) {
        for _ in 0..mov.n {
            self.stacks[mov.to].push(self.stacks[mov.from].pop().unwrap());
        }
    }
}

struct Movement {
    n: u32,
    from: usize,
    to: usize
}

fn parse_one(line: &str) -> Movement {
    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    return move_re.captures_iter(line)
        .map(|y| Movement {
        n: (&y[1]).parse::<u32>().unwrap(),
        from: (&y[2]).parse::<usize>().unwrap(),
        to: (&y[3]).parse::<usize>().unwrap(),
    }).collect()
}

fn solve1(crates: Stacks, moves: &str) -> String {
    let parsed_moves: Vec<Movement> = moves
        .lines()
        .map(|x| parse_one(x)).collect();


    for single_move in parsed_moves {
        crates.move_crates(single_move);
    }

    return crates.tops()
}

// fn solve2(input: &str) -> u32 {
//     return input
//         .lines()
//         .map(|x| new_pair(x.trim()).range_overlaps())
//         .filter(|x| *x)
//         .count() as u32;
// }


#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let input = problem::load_test(5);
        let initial_1 = Stack{ stack: vec!['N', 'Z']};
        let initial_2 = Stack{ stack: vec!['D', 'C', 'M']};
        let initial_3 = Stack{ stack: vec!['P']};
        let initial = Stacks{ stacks: vec![initial_1, initial_2, initial_3]};
        let solution_1 = solve1(initial, &input);
        assert_eq!(solution_1, "CMZ");
        // let solution_2 = solve2(&input);
        // assert_eq!(solution_2, 4);
    }
}