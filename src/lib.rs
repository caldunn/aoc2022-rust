#![feature(iter_next_chunk, slice_group_by)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
pub mod days;

pub fn read_file(day_and_file: &str) -> Vec<String> {
    let file = File::open(Path::new(
        format!("./puzzle_inputs/{}", day_and_file).as_str(),
    ))
    .expect("Could not open input file");
    let reader = BufReader::new(file);
    reader.lines().filter_map(|s| s.ok()).collect()
}
