use std::{
    path::Path,
};

mod read;

pub fn solve_day1() {
    let elves: Vec<Elf> = elves_from_file("./inputs/day1");
    let mut calories: Vec<i32> = elves.into_iter().map(|e| e.total_calories).collect();
    calories.sort_by(|a, b| b.cmp(a));
    println!("{:?}", calories);
    let solution_1: i32 = get_highest_n(&calories, 1);
    let solution_2: i32 = get_highest_n(&calories, 3);
    println!("Puzzle 1: {}, puzzle 2: {}", solution_1, solution_2);
}

#[derive(Debug)]
struct Elf {
    calories: Vec<i32>,
    total_calories: i32
}

impl Default for Elf {
    fn default() -> Elf {
        Elf {
            calories: vec![],
            total_calories: 0
        }
    }
}

fn elves_from_file(filename: impl AsRef<Path>) -> Vec<Elf> {
    let lines = read::read_lines(filename);
    let mut elves: Vec<Elf> = vec![];
    let mut elf: Elf = Elf {..Default::default()};
    for line in lines {
        let text: String = line.unwrap();
        if text.is_empty() {
            elf.total_calories = elf.calories.iter().sum();
            elves.push(elf);
            elf = Elf {..Default::default()};
        } else {
            elf.calories.push(text.parse::<i32>().unwrap());
        }
    }
    return elves;
}

fn get_highest_n(calories: &Vec<i32>, n: usize) -> i32 {
    return calories.iter().take(n).sum();
}