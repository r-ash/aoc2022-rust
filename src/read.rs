use std::{
    fs::File,
    io::{Lines, BufRead, BufReader},
    path::Path,
};

pub fn read_lines(filename: impl AsRef<Path>) -> Lines<BufReader<File>> {
    let file = File::open(filename).expect("no such file");
    return BufReader::new(file).lines()
}
