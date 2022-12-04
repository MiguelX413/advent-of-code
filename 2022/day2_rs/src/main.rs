use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

impl Move {
    fn from_char(char: char) -> Result<Move, ()> {
        match char {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(()),
        }
    }

    fn cmp(self, other: Move) -> Outcome {
        if self == other {
            return Outcome::Draw;
        }
        match (self, other) {
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => Outcome::Win,
            _ => Outcome::Loss,
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = Path::new(&args[1]);
    let file = File::open(file_path).unwrap();
    let sum = io::BufReader::new(file)
        .lines()
        .map(|f| f.unwrap())
        .filter_map(|f| {
            let mut chars = f.trim().chars();
            let enemy = Move::from_char(chars.next()?).unwrap();
            chars.next()?;
            let me = Move::from_char(chars.next()?).unwrap();

            Some(me.cmp(enemy) as i32 + me as i32)
        })
        .sum::<i32>();

    println!("Sum: {}", sum);
}
