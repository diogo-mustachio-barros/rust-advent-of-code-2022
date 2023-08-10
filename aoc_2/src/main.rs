use std::{path::Path, io::{self, BufRead}, fs::File, env, fmt::Display};

fn main() {
    let args:Vec<String> = env::args().collect();
    let filename;

    match args.get(1) {
        None => return (),
        Some(s) => filename = s
    }

    // Part 1
    // let result = calc(filename, strategy_1);
    
    // Part 2
    let result = calc(filename, strategy_2);

    println!("{result}");
}

fn strategy_1(str:&str, _:&RPS) -> RPS {
    match str {
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => todo!()
    }
}

fn strategy_2(str:&str, predicted:&RPS) -> RPS {
    match str {
        "X" => match predicted {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        },
        "Y" => match predicted {
            RPS::Rock => RPS::Rock,
            RPS::Paper => RPS::Paper,
            RPS::Scissors => RPS::Scissors,
        },
        "Z" => match predicted {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        },
        _ => todo!()
    }
}

fn calc<P>(filename:P, strategy: fn(&str, &RPS) -> RPS) -> i32
where P:AsRef<Path> {
    let mut total_score = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(str) = line 
            {
                let round = Round::from_str(&str, strategy);
                total_score += round.calc_score();
            }
        }
    }

    return total_score;
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

enum RPS {
    Rock, Paper, Scissors
}

impl RPS {
    fn from_str(str:&str) -> RPS {
        match str {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => todo!()
        }
    }

    fn against(&self, other:Self) -> Outcome {
        match (self, other) {
            (RPS::Rock, RPS::Scissors) 
            | (RPS::Scissors, RPS::Paper) 
            | (RPS::Paper, RPS::Rock) => Outcome::Win,
            
            (RPS::Rock, RPS::Rock)
            | (RPS::Paper, RPS::Paper)
            | (RPS::Scissors, RPS::Scissors) => Outcome::Draw,

            _ => Outcome::Lose,
        }
    }

    fn get_score(self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

impl Display for RPS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RPS::Rock => write!(f, "Rock"),
            RPS::Paper => write!(f, "Paper"),
            RPS::Scissors => write!(f, "Scissors"),
        }
    }
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
        let outcome_score = self.played.against(self.predicted).get_score();
        let played_score = self.played.get_score();

        return outcome_score + played_score;
    }
}

// from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
