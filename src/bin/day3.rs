use aoc2022_rust::problem;
use std::collections::HashSet;

fn main() {
    let input = problem::load(3);
    let solution_1 = solve1(&input);
    println!("Puzzle 1 {}", solution_1);
    let solution_2 = solve2(&input);
    println!("Puzzle 2 {}", solution_2);
}

fn solve1(input: &str) -> u32 {
    return input
        .lines()
        .map(|x| get_priority(get_common_item(x.trim()))).sum();
}

fn get_common_item(backpack: &str) -> char {
    let size = backpack.chars().count();
    let mut iter = backpack.chars();
    let set: HashSet<char> = (&mut iter).take(size/2).collect();
    for item in iter {
        if set.contains(&item) {
            return item;
        }
    }
    panic!("No common item in backpack");
}

fn get_priority(item: char) -> u32 {
    // We want a-z -> 1 - 26 and A-Z -> 27-52
    return if item.is_lowercase() {
        item as u32 - 96
    } else {
        item as u32 - 38
    }
}

fn solve2(input: &str) -> u32 {
    let mut lines = input.lines().peekable();
    let mut sum: u32 = 0;
    // Relying on that fact that if we can get 1 we can get 3 as this is a nice puzzle
    while lines.peek().is_some() {
        sum += get_priority(get_common_items(lines.next().unwrap(),
                                             lines.next().unwrap(),
                                             lines.next().unwrap()));
    }
    return sum;
}

fn get_common_items(backpack1: &str, backpack2: &str, backpack3: &str) -> char {
    for item1 in backpack1.chars() {
        for item2 in backpack2.chars() {
            for item3 in backpack3.chars() {
                if item1 == item2 && item2 == item3 {
                    return item1;
                }
            }
        }
    }
    panic!("failed to find common item")
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let input = problem::load_test(3);
        let solution_1 = solve1(&input);
        assert_eq!(solution_1, 157);
        let solution_2 = solve2(&input);
        assert_eq!(solution_2, 70);
    }
}