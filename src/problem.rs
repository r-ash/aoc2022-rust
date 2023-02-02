use std::fs;

pub fn load(day: u32) -> String {
    load_raw(day).trim().replace('\r', "")
}

pub fn load_raw(day: u32) -> String {
    let file = format!("inputs/{:02}", day);
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
}

pub fn load_test(day: u32) -> String {
    load_raw_test(day).trim().replace('\r', "")
}

pub fn load_raw_test(day: u32) -> String {
    let file = format!("inputs/test_{:02}", day);
    fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
}