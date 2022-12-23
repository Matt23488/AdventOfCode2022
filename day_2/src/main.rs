use std::fs;

fn main() {
    let plays = get_plays();
    part_one(&plays);
    part_two(&plays);
}

fn part_one(plays: &Vec<(char, char)>) {
    let total_points: u32 = plays.iter()
        .map(|(o, r)| (Play::new(*o), Play::new(*r)))
        .map(|(o, r)| r.get_points(&o))
        .sum();

    dbg!(total_points);
}

fn part_two(plays: &Vec<(char, char)>) {
    let total_points: u32 = plays.iter()
        .map(|(o, r)| (Play::new(*o), GameResult::new(*r)))
        .map(|(o, r)| r.get_points(&o))
        .sum();

    dbg!(total_points);
}

#[derive(Debug)]
enum GameResult {
    Loss,
    Draw,
    Win,
}

impl GameResult {
    fn new(code: char) -> Self {
        match code {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!(),
        }
    }

    fn get_points(&self, opponent: &Play) -> u32 {
        match opponent {
            Play::Rock => match self {
                Self::Loss => 0 + 3,
                Self::Draw => 3 + 1,
                Self::Win => 6 + 2,
            }
            Play::Paper => match self {
                Self::Loss => 0 + 1,
                Self::Draw => 3 + 2,
                Self::Win => 6 + 3,
            }
            Play::Scissors => match self {
                Self::Loss => 0 + 2,
                Self::Draw => 3 + 3,
                Self::Win => 6 + 1,
            }
        }
    }
}

#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn new(code: char) -> Self {
        match code {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Expected valid char")
        }
    }

    fn get_points(&self, other: &Self) -> u32 {
        match self {
            Self::Rock => {
                match other {
                    Self::Rock => 1 + 3,
                    Self::Paper => 1 + 0,
                    Self::Scissors => 1 + 6,
                }
            }
            Self::Paper => {
                match other {
                    Self::Rock => 2 + 6,
                    Self::Paper => 2 + 3,
                    Self::Scissors => 2 + 0,
                }
            }
            Self::Scissors => {
                match other {
                    Self::Rock => 3 + 0,
                    Self::Paper => 3 + 6,
                    Self::Scissors => 3 + 3,
                }
            }
        }
    }
}

fn get_plays() -> Vec<(char, char)> {
    let input = fs::read_to_string("input.txt").unwrap();

    input.lines().map(|line| (
        line.chars().nth(0).unwrap(),
        line.chars().nth(2).unwrap(),
    )).collect()
}
