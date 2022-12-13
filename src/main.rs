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
        "4" => days::four::main(),
        "5" => days::five::main(),
        "6" => days::six::main(),
        "7" => days::seven::main(),
        "8" => days::eight::main(),
        "9" => days::nine::main(),
        "10" => days::ten::main(),
        "11" => days::eleven::main(),
        _ => days::one::main(),
    }
    .expect("failed");
}
