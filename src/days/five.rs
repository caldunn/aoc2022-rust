// Towers of Hanoi question it seems?

pub fn main() -> std::io::Result<()> {
    let lines = crate::read_file("day5/input");
    let initial: Vec<&String> = lines
        .iter()
        .take_while(|s| !s.chars().nth(1).unwrap().is_numeric())
        .collect();
    let mut cargo = Cargo::from_slice(initial.as_slice());
    let mut cargo2 = cargo.clone();
    let starting = initial.len() + 2;
    let commands: Vec<_> = lines.iter().skip(starting).map(split_instruction).collect();

    commands.iter().for_each(|command| cargo.swap(command));

    commands
        .iter()
        .for_each(|command| cargo2.swap_ordered(command));

    println!("RESULT\nP1: {}\nP2: {}", cargo.top_row(), cargo2.top_row());
    Ok(())
}
#[derive(Clone)]
struct Cargo {
    cargo: Vec<Vec<char>>,
}

impl Cargo {
    fn from_slice(lines: &[&String]) -> Self {
        let columns = (lines[0].len() + 2) / 4;
        let mut cargo: Vec<Vec<char>> = Vec::with_capacity(columns);
        for _ in 0..columns {
            cargo.push(vec![]);
        }
        for line in lines {
            for (i, character) in line.chars().enumerate() {
                if i % 4 == 1 && character.is_alphabetic() {
                    cargo.get_mut(i / 4).unwrap().push(character);
                }
            }
        }
        // reverse as we are reading from the top
        for vec in &mut cargo {
            vec.reverse();
        }
        Self { cargo }
    }

    pub fn swap(&mut self, command: &Command) {
        let (from_idx, to_idx) = ((command.from - 1) as usize, (command.to - 1) as usize);
        for _ in 0..command.count {
            let swap = self.cargo.get_mut(from_idx).unwrap().pop().unwrap();
            self.cargo.get_mut(to_idx).unwrap().push(swap);
        }
    }
    pub fn swap_ordered(&mut self, command: &Command) {
        let (from_idx, to_idx) = ((command.from - 1) as usize, (command.to - 1) as usize);
        let length = self.cargo.get(from_idx).unwrap().len() - command.count as usize;
        let mut swap = self.cargo.get_mut(from_idx).unwrap().split_off(length);
        self.cargo.get_mut(to_idx).unwrap().append(&mut swap);
    }

    pub fn top_row(&self) -> String {
        self.cargo.iter().fold("".to_string(), |acc, v| {
            format!("{}{}", acc, *v.last().unwrap())
        })
    }
}
struct Command {
    from: u8,
    to: u8,
    count: u8,
}
fn split_instruction(s: &String) -> Command {
    let mut split = s.splitn(6, " ");
    let count = split.nth(1).unwrap().parse().unwrap();
    Command {
        from: split.nth(1).unwrap().parse().unwrap(),
        to: split.nth(1).unwrap().parse().unwrap(),
        count,
    }
}
