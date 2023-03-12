use aoc2022_rust::problem;
use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let input = problem::load(7);
    let solution_1 = solve1(&input);
    println!("Puzzle 1 {}", solution_1);
    let solution_2 = solve2(&input);
    println!("Puzzle 2 {}", solution_2);
}

fn is_root_dir(line: &str) -> bool {
    lazy_static! {
        static ref RE_UP: Regex = Regex::new(r"cd /").unwrap();
    }
    RE_UP.is_match(line)
}

fn is_cd_up(line: &str) -> bool {
    lazy_static! {
        static ref RE_UP: Regex = Regex::new(r"cd \.\.").unwrap();
    }
    RE_UP.is_match(line)
}

fn get_dir(line: &str) -> Option<&str> {
    lazy_static! {
        static ref RE_DIR: Regex = Regex::new(r"cd ([\w/])").unwrap();
    }
    match RE_DIR.captures(line) {
        Some(cap) => Some(cap.get(1)?.as_str()),
        None => None
    }
}

fn get_file_size(line: &str) -> Option<u32> {
    lazy_static! {
        static ref RE_SIZE: Regex = Regex::new(r"(\d+) [\w\.]+").unwrap();
    }
    match RE_SIZE.captures(line) {
        Some(cap) => {
            Some(cap.get(1)?.as_str().parse::<u32>().unwrap())
        },
        None => None
    }
}

struct Dir {
    path: Vec<String>,
    size: u32,
}

impl Dir {
    fn new(path: Vec<String>) -> Self {
        Dir {
            path,
            size: 0,
        }
    }

    fn add_size(&mut self, size: u32) -> () {
        self.size += size;
    }
}

enum Action {
    CdRoot,
    CdDir(String),
    CdUp,
    AddSize(u32),
    NoAction
}

fn get_action(line: &str) -> Action {
    let new_dir: Option<&str> = get_dir(line);
    let file_size: Option<u32> = get_file_size(line);
    return if is_root_dir(line) {
        Action::CdRoot
    } else if new_dir.is_some() {
        Action::CdDir(new_dir.unwrap().to_string())
    } else if is_cd_up(line) {
        Action::CdUp
    } else if file_size.is_some() {
        Action::AddSize(file_size.unwrap())
    } else {
        Action::NoAction
    }
}

fn get_dir_sizes(input: &str) -> Vec<Dir> {
    let mut active_dirs: Vec<Dir> = vec![];
    let mut complete_dirs: Vec<Dir> = vec![];
    let mut current_path : Vec<String> = vec![];
    for line in input.lines() {
        let action: Action = get_action(line);
        match action {
            Action::CdRoot => {
                current_path.push("/".to_string());
                let root = Dir::new(current_path.clone());
                active_dirs.push(root);
            },
            Action::CdDir(new_dir) => {
                current_path.push(new_dir);
                let new_dir = Dir::new(current_path.clone());
                active_dirs.push(new_dir);
            },
            Action::CdUp => {
                current_path.pop();
                complete_dirs.push(active_dirs.pop().unwrap());
            },
            Action::AddSize(size) => {
                for dir in active_dirs.iter_mut() {
                    dir.add_size(size);
                }
            }
            Action::NoAction => ()
        }
    }
    // Add remaining dirs to completed, the log doesn't necessarily return to the root
    complete_dirs.extend(active_dirs);
    complete_dirs
}

fn solve1(input: &str) -> u32 {
    let dirs = get_dir_sizes(input);
    let mut sum_smallest: u32 = 0;
    for dir in dirs {
        if dir.size <= 100000 {
            sum_smallest += dir.size;
        }
    }
    sum_smallest
}

fn solve2(input: &str) -> u32 {
    let dirs = get_dir_sizes(input);
    let mut total_used = 0;
    for dir in dirs.iter() {
        if dir.size > total_used {
            total_used = dir.size;
        }
    }

    let needed_size: i32 = 30000000;
    let to_free: i32 = needed_size - (70000000 - total_used as i32);
    let mut smallest_delete = u32::MAX;
    for dir in dirs.iter() {
        if (dir.size as i32) > to_free && dir.size < smallest_delete {
            smallest_delete = dir.size
        }
    }
    smallest_delete
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn regex_works() {
        assert_eq!(is_root_dir("cd .."), false);
        assert_eq!(is_root_dir("cd a"), false);
        assert_eq!(is_root_dir("cd /"), true);
        assert_eq!(is_root_dir("ls"), false);
        assert_eq!(is_root_dir("123 a.x"), false);
        assert_eq!(is_root_dir("dir e"), false);

        assert_eq!(is_cd_up("cd .."), true);
        assert_eq!(is_cd_up("cd a"), false);
        assert_eq!(is_cd_up("cd /"), false);
        assert_eq!(is_cd_up("ls"), false);
        assert_eq!(is_cd_up("123 a.x"), false);
        assert_eq!(is_cd_up("dir e"), false);

        assert_eq!(get_dir("cd .."), None);
        assert_eq!(get_dir("cd a"), Some("a"));
        assert_eq!(get_dir("cd /"), Some("/"));
        assert_eq!(get_dir("ls"), None);
        assert_eq!(get_dir("123 a.x"), None);
        assert_eq!(get_dir("dir e"), None);

        assert_eq!(get_file_size("cd .."), None);
        assert_eq!(get_file_size("cd a"), None);
        assert_eq!(get_file_size("cd /"), None);
        assert_eq!(get_file_size("ls"), None);
        assert_eq!(get_file_size("123 a.x"), Some(123));
        assert_eq!(get_file_size("dir e"), None);
    }

    #[test]
    fn it_works() {
        let input = problem::load_test(7);
        let solution_1 = solve1(&input);
        assert_eq!(solution_1, 95437);
        let solution_2 = solve2(&input);
        assert_eq!(solution_2, 24933642);
    }
}