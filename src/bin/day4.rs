use aoc2022_rust::problem;
use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let input = problem::load(4);
    let solution_1 = solve1(&input);
    println!("Puzzle 1 {}", solution_1);
    let solution_2 = solve2(&input);
    println!("Puzzle 2 {}", solution_2);
}

struct Pair {
    elf_1: (u32, u32),
    elf_2: (u32, u32)
}

impl Pair {
    fn is_subset(&self) -> bool {
        (self.elf_1.0 >= self.elf_2.0 && self.elf_1.1 <= self.elf_2.1) || // elf 1 inside elf 2
            (self.elf_1.0 <= self.elf_2.0 && self.elf_1.1 >= self.elf_2.1)// elf 2 inside elf 1
    }

    fn range_overlaps(&self) -> bool {
        self.elf_1.1 >= self.elf_2.0 && self.elf_1.0 <= self.elf_2.1
    }
}

fn new_pair(input: &str) -> Pair {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    let matches: Vec<u32> = RE.find_iter(input)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect();
    Pair {
        elf_1: (matches[0], matches[1]),
        elf_2: (matches[2], matches[3])
    }
}


fn solve1(input: &str) -> u32 {
    return input
        .lines()
        .map(|x| new_pair(x.trim()).is_subset())
        .filter(|x| *x)
        .count() as u32;
}

fn solve2(input: &str) -> u32 {
    return input
        .lines()
        .map(|x| new_pair(x.trim()).range_overlaps())
        .filter(|x| *x)
        .count() as u32;
}


#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let input = problem::load_test(4);
        let solution_1 = solve1(&input);
        assert_eq!(solution_1, 2);
        let solution_2 = solve2(&input);
        assert_eq!(solution_2, 4);
    }
}