use aoc2022_rust::problem;
use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let input = problem::load(10);
    let solution_1 = solve1(&input);
    println!("Puzzle 1 {}", solution_1);
    // let solution_2 = solve2(&input);
    // println!("Puzzle 2 {}", solution_2);
}

struct Buffer {
    value: i32
}

fn get_value(input: &str) -> Option<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"addx ([-]*\d*)").unwrap();
    }
    let capture = RE.captures(input);
    capture?.get(1)?.as_str().parse::<i32>().ok()
}

fn solve1(input: &str) -> i32 {
    let cumulative_sum: Vec<i32> = input
        .lines()
        .map(|x| get_value(x.trim()))
        .scan(0, |acc, x| {
            match x {
                Some(v) => {
                    *acc = (*acc) + v;
                    println!("{}, {}", v, *acc);
                    Some((*acc))
                },
                None => {
                    println!("{}", *acc);
                    Some(*acc)
                }
            }
        }).collect();


    calc_signal_strength(&cumulative_sum, 20) +
        calc_signal_strength(&cumulative_sum, 60) +
        calc_signal_strength(&cumulative_sum, 100) +
        calc_signal_strength(&cumulative_sum, 140) +
        calc_signal_strength(&cumulative_sum, 180) +
        calc_signal_strength(&cumulative_sum, 220)
}

fn calc_signal_strength(sums: &Vec<i32>, step: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut cycle = 0;
    for pair in sums.windows(2) {
        cycle += 1;
        if pair[0] == pair[1] {
            continue;
        }
        // check if cycle in the set of 20, 40, ... then add em
        sum += pair[0];
    }
    let value = if step == sums.len() as i32 {
        sums[(step - 1) as usize]
    } else {
        sums[(step) as usize]
    };
    step * value
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
        let input = problem::load_test(10);
        let solution_1 = solve1(&input);
        assert_eq!(solution_1, 13140);
        // let solution_2 = solve2(&input);
        // assert_eq!(solution_2, 4);
    }
}