#![feature(iter_next_chunk, slice_group_by)]

use std::{
    fmt::Debug,
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
pub fn print_results<T, V>(part_1: T, part_2: V)
where
    T: Debug,
    V: Debug,
{
    println!("RESULTS\nPart 1: {:?}\nPart 2: {:?}", part_1, part_2);
}
