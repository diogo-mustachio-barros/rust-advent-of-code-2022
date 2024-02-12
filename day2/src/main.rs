
// https://adventofcode.com/2022/day/2

use std::{fs::File, io::{BufReader, Lines}};

use util::advent_of_code::redirect;

fn main() {
    redirect(part_1, part_2);
}

pub fn part_1(lines: Lines<BufReader<File>>) {
    let score = calc(lines, strategy_1);
    println!("Total score: {}", score);
}

fn strategy_1(str:&str, _:&RPS) -> RPS {
    match str {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        s => panic!("could not parse '{}' as a strategy", s)
    }
}

pub fn part_2(lines: Lines<BufReader<File>>) {
    let score = calc(lines, strategy_2);
    println!("Total score: {}", score);
}

fn strategy_2(str:&str, predicted:&RPS) -> RPS {
    match str {
        "X" => predicted.wins_against(),  // Outcome is a loss
        "Y" => predicted.clone(),         // Outcome is a draw
        "Z" => predicted.loses_against(), // Outcome is a win
        s => panic!("could not parse '{}' as a strategy", s)
    }
}



fn calc(lines:Lines<BufReader<File>>, strategy: fn(&str, &RPS) -> RPS) -> i32 {
    lines.flatten()
         .into_iter()
         .map(|line| Round::from_str(&line, strategy).calc_score())
         .sum()
}

struct Round {
    pub predicted:RPS, 
    pub played:RPS,
}

impl Round {
    fn from_str(str:&str, strategy:fn(&str, &RPS) -> RPS) -> Round {
        let vec:Vec<&str> = str.split(" ").collect();
    
        let predicted = RPS::from_str(vec.get(0).unwrap());
        let played = strategy(vec.get(1).unwrap(), &predicted);
    
        return Round{predicted, played};
    }

    fn calc_score(self) -> i32 {
        let outcome = self.played.against(self.predicted);
        let outcome_score = outcome.get_score();
        let played_score = self.played.get_score();

        return outcome_score + played_score;
    }
}

#[derive(PartialEq, Clone)]
enum RPS {
    Rock, Paper, Scissors
}

impl RPS {
    fn from_str(str:&str) -> RPS {
        match str {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            s => panic!("could not parse '{}' as RPS", s)
        }
    }

    fn wins_against(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    } 

    fn loses_against(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }

    fn against(&self, other:Self) -> Outcome {
        if *self == other {
            return Outcome::Draw;
        }

        if *self == other.loses_against() {
            return Outcome::Win;
        }

        return Outcome::Lose
    }

    fn get_score(self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

enum Outcome {
    Win, Draw, Lose
}

impl Outcome {
    fn get_score(self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}