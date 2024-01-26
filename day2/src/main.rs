
// https://adventofcode.com/2022/day/2

use std::{env,  fs::File, io::{self, BufRead, BufReader, Lines}, path::Path};

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Use: cargo run <1|2> <input filepath>");
        return;
    }

    let part = args.get(1).expect("no part selected");
    let filename = args.get(2).expect("no input file path given");

    // Read file
    let lines = read_lines(filename).expect("error reading file");

    match part.as_str() {
        // Part 1
        "1" => part_1(lines),
        // Part 2
        "2" => part_2(lines),
        // Error
        _ => println!("selected part is invalid"),
    }
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




// from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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