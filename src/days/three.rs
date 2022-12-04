use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Find common item types
// compartments split into 2 at midpoint of string.
// a - z -> 1 - 26, A - Z -> 27, 52
// then sum
pub fn main() -> std::io::Result<()> {
    let path = Path::new("./puzzle_inputs/day3/input");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|s| s.ok()).collect();

    // Part 1;
    let score = lines
        .iter()
        .map(|s| s.split_at(s.chars().count() / 2))
        .map(|(left, right)| (left.chars().collect::<HashSet<_>>(), right))
        .map(|(left_set, right)| right.chars().find(|c| left_set.contains(c)).unwrap())
        .fold(0, |acc, c| acc + as_point_value(&c));

    // Part 2
    let mut badges: Vec<char> = Vec::with_capacity(lines.len() / 3);
    let mut line_iter = lines.iter();
    while let Ok([x, y, z]) = line_iter.next_chunk::<3>() {
        let elf1 = x.chars().collect::<HashSet<_>>();
        let elf2 = y.chars().collect::<HashSet<_>>();
        badges.push(
            z.chars()
                .find(|c| elf1.contains(c) && elf2.contains(c))
                .expect("Should be atleast one matching char"),
        )
    }
    let score2 = badges.iter().fold(0, |acc, c| acc + as_point_value(c));

    println!("Results\nPart 1: {}\nPart 2: {}", score, score2);
    Ok(())
}

fn as_point_value(c: &char) -> u32 {
    let base = *c as u32 - 64;
    if base >= 27 {
        base - 32
    } else {
        base + 26
    }
}
