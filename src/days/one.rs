use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

// Individual calorie counting for elves.
// 1 src of energy per line.
//

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Elf(usize);

pub fn main() -> std::io::Result<()> {
    println!("Welcome to day 1;");
    let path = Path::new("./puzzle_inputs/day1_1/input");
    let file = File::open(&path).expect("could not open file input");
    let reader = BufReader::new(file);
    let mut elves: Vec<Elf> = vec![];

    let mut current_total = 0;
    for line in reader.lines() {
        match line.expect("error reading file").parse::<usize>() {
            Err(_) => {
                elves.push(Elf(current_total));
                current_total = 0;
            }
            Ok(item_cals) => current_total += item_cals,
        }
    }
    elves.sort();
    // Single elves max
    println!("Top Elf: {:?}", elves.last().unwrap());

    // top3
    let top3 = elves
        .iter()
        .rev()
        .take(3)
        .fold(0usize, |acc, elf| acc + elf.0);
    println!("Top 3 Elves: {:?}", top3);

    Ok(())
}
