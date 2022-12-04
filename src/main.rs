use aoc::days;
use std::env::args;
fn main() {
    println!("Advent of code 2022 in rust");
    let day = args().take(2).skip(1).next().expect("Please provide a day");
    // I could use this as an opportunity to write my first macro.
    match day.as_str() {
        "1" => days::one::main(),
        "2" => days::two::main(),
        "3" => days::three::main(),
        _ => days::one::main(),
    }
    .expect("failed");
}
