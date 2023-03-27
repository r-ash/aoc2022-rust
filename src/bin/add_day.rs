use clap::Parser;
use std::fs::File;
use std::io::Write;

/// Add template for solving advent of code for a specific day
#[derive(Parser, Debug)]
struct Cli {
    /// The day number
    day_number: String,
}

fn main() {
    let args = Cli::parse();
    File::create(format!("inputs/{}", args.day_number))
        .expect("Failed to create input file");
    File::create(format!("inputs/test_{}", args.day_number))
        .expect("Failed to create input test file");
    let mut src_file = File::create(format!("src/bin/day{}.rs", args.day_number))
        .expect("Failed to create source file");
    src_file.write_all(get_solution_template(args.day_number).as_bytes())
        .expect("Unable to write src file");
}

fn get_solution_template(day_number: String) -> String {
    format!("fn main() {{
    let input = problem::load({day_number});
    let solution_1 = solve1(&input);
    println!(\"Puzzle 1 {{}}\", solution_1);
    // let solution_2 = solve2(&input);
    // println!(\"Puzzle 2 {{}}\", solution_2);
}}

fn solve1(input: &str) -> u32 {{
    return input
        .lines()
        .map(|x| new_pair(x.trim()).is_subset())
        .filter(|x| *x)
        .count() as u32;
}}

//fn solve2(input: &str) -> u32 {{
//    return input
//        .lines()
//        .map(|x| new_pair(x.trim()).range_overlaps())
//        .filter(|x| *x)
//        .count() as u32;
//}}

#[cfg(test)]
mod tests {{
    use crate::*;
    #[test]
    fn it_works() {{
        let input = problem::load_test({day_number});
        let solution_1 = solve1(&input);
        assert_eq!(solution_1, 2);
        //let solution_2 = solve2(&input);
        //assert_eq!(solution_2, 4);
    }}
}}")
}

