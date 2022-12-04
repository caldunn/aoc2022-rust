#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Elf(usize);

pub fn main() -> std::io::Result<()> {
    println!("Welcome to day 1;");
    let lines = crate::read_file("day1_1/input");

    let mut elves: Vec<Elf> = Vec::with_capacity(lines.len());
    let mut current_total = 0;

    for line in lines {
        match line.parse::<usize>() {
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
