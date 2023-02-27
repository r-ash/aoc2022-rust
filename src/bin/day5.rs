use aoc2022_rust::problem;
use regex::Regex;
use std::clone::Clone;

#[derive(Debug, Clone)]
struct Stack<T> {
    stack: Vec<T>
}

impl<T> Stack<T> {
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn pop_n(&mut self, n: u32) -> Vec<T> {
        self.stack.drain((self.stack.len() - usize::try_from(n).unwrap())..).collect()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

#[derive(Debug, Clone)]
struct Stacks {
    stacks: Vec<Stack<char>>
}

impl Stacks {
    fn tops(self) -> String {
        let mut ret = "".to_string();
        for stack in self.stacks {
            ret.push(*stack.peek().unwrap());
        }
        ret
    }

    fn move_crates(&mut self, mov: Movement) {
        for _ in 0..mov.n {
            let value = self.stacks[mov.from - 1].pop().unwrap();
            self.stacks[mov.to - 1].push(value);
        }
    }

    fn move_2(&mut self, mov: Movement) {
        let out: Vec<char> = self.stacks[mov.from - 1].pop_n(mov.n);
        for item in out {
            self.stacks[mov.to - 1].push(item);
        }
    }
}

fn main() {
    let input = problem::load(5);
    let initial_1 = Stack{ stack: vec!['W', 'M', 'L', 'F']};
    let initial_2 = Stack{ stack: vec!['B', 'Z', 'V', 'M', 'F']};
    let initial_3 = Stack{ stack: vec!['H', 'V', 'R', 'S', 'L', 'Q']};
    let initial_4 = Stack{ stack: vec!['F', 'S', 'V', 'Q', 'P', 'M', 'T', 'J']};
    let initial_5 = Stack{ stack: vec!['L', 'S', 'W']};
    let initial_6 = Stack{ stack: vec!['F', 'V', 'P', 'M', 'R', 'J', 'W']};
    let initial_7 = Stack{ stack: vec!['J', 'Q', 'C', 'P', 'N', 'R', 'F']};
    let initial_8 = Stack{ stack: vec!['V', 'H', 'P', 'S', 'Z', 'W', 'R', 'B']};
    let initial_9 = Stack{ stack: vec!['B', 'M', 'J', 'C', 'G', 'H', 'Z', 'W']};
    let initial = Stacks{ stacks: vec![initial_1, initial_2, initial_3, initial_4,
                                       initial_5, initial_6, initial_7, initial_8, initial_9]};
    let initial2 = initial.clone();
    let solution_1 = solve1(initial, &input);
    println!("Puzzle 1 {}", solution_1);
    let solution_2 = solve2(initial2, &input);
    println!("Puzzle 2 {}", solution_2);
}

struct Movement {
    n: u32,
    from: usize,
    to: usize
}

fn solve1(mut crates: Stacks, moves: &str) -> String {
    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let parsed_moves: Vec<Movement> = move_re.captures_iter(moves).map(|y| Movement {
        n: (&y[1]).parse::<u32>().unwrap(),
        from: (&y[2]).parse::<usize>().unwrap(),
        to: (&y[3]).parse::<usize>().unwrap(),
    }).collect();


    for single_move in parsed_moves {
        crates.move_crates(single_move);
    }

    return crates.tops()
}

fn solve2(mut crates: Stacks, moves: &str) -> String {
    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let parsed_moves: Vec<Movement> = move_re.captures_iter(moves).map(|y| Movement {
        n: (&y[1]).parse::<u32>().unwrap(),
        from: (&y[2]).parse::<usize>().unwrap(),
        to: (&y[3]).parse::<usize>().unwrap(),
    }).collect();


    for single_move in parsed_moves {
        crates.move_2(single_move);
    }

    return crates.tops()
}


#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let input = problem::load_test(5);
        let initial_1 = Stack{ stack: vec!['Z', 'N']};
        let initial_2 = Stack{ stack: vec!['M', 'C', 'D']};
        let initial_3 = Stack{ stack: vec!['P']};
        let initial = Stacks{ stacks: vec![initial_1, initial_2, initial_3]};
        let initial2 = initial.clone();
        let solution_1 = solve1(initial, &input);
        assert_eq!(solution_1, "CMZ");
        let solution_2 = solve2(initial2, &input);
        assert_eq!(solution_2, "MCD");
    }
}