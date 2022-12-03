use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}
impl Play {
    fn from_char(c: char) -> Self {
        match c {
            'A' | 'X' => Play::Rock,
            'B' | 'Y' => Play::Paper,
            'C' | 'Z' => Play::Scissors,
            _ => panic!("invalid char"),
        }
    }
}
#[derive(Debug)]
struct Round {
    player: Play,
    opponent: Play,
}

impl Round {
    fn from_space_split_string(raw: &String) -> Result<Self, ()> {
        let (opp, player) = raw.split_at(1);
        Ok(Self {
            player: Play::from_char(player.trim().chars().nth(0).unwrap()),
            opponent: Play::from_char(opp.trim().chars().nth(0).unwrap()),
        })
    }
    fn part_2_split(raw: &String) -> Result<Self, ()> {
        let (opp, player_raw) = raw.split_at(1);
        let opponent = Play::from_char(opp.trim().chars().nth(0).unwrap());
        let player = match (&opponent, player_raw.trim().chars().nth(0).unwrap()) {
            (Play::Rock, 'X') => Play::Scissors,
            (Play::Rock, 'Z') => Play::Paper,

            (Play::Paper, 'X') => Play::Rock,
            (Play::Paper, 'Z') => Play::Scissors,

            (Play::Scissors, 'X') => Play::Paper,
            (Play::Scissors, 'Z') => Play::Rock,

            (x, 'Y') => x.clone(),
            _ => panic!("invalid input"),
        };
        Ok(Self { player, opponent })
    }
    fn player_score(&self) -> usize {
        // I was lazy. I will fix this up... maybe
        match (&self.opponent, &self.player) {
            (Play::Rock, Play::Rock) => 3 + 1,
            (Play::Rock, Play::Paper) => 6 + 2,
            (Play::Rock, Play::Scissors) => 0 + 3,

            (Play::Paper, Play::Rock) => 0 + 1,
            (Play::Paper, Play::Paper) => 3 + 2,
            (Play::Paper, Play::Scissors) => 6 + 3,

            (Play::Scissors, Play::Rock) => 6 + 1,
            (Play::Scissors, Play::Paper) => 0 + 2,
            (Play::Scissors, Play::Scissors) => 3 + 3,
        }
    }
}
fn main() -> std::io::Result<()> {
    println!("Welcome to day 2;");
    let path = Path::new("./puzzle_inputs/day2/input");
    let file = File::open(&path).expect("could not open file input");
    let reader = BufReader::new(file);

    let mut rounds_strat1: Vec<Round> = vec![];
    let mut rounds_strat2: Vec<Round> = vec![];

    for line in reader.lines() {
        let line_ref = line.expect("could not parse input line.");
        rounds_strat1
            .push(Round::from_space_split_string(&line_ref).expect("unable to parse line"));
        rounds_strat2.push(Round::part_2_split(&line_ref).expect("unable to parse line"));
    }
    let score = rounds_strat1
        .iter()
        .fold(0usize, |acc, round| acc + round.player_score());

    let score2 = rounds_strat2
        .iter()
        .fold(0usize, |acc, round| acc + round.player_score());
    println!("SCORES\nStrategy-1: {}\nStrategy-2: {}", score, score2);
    Ok(())
}
