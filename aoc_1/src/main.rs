
// https://adventofcode.com/2022/day/1

use std::{env, fs::File, io::{self, BufRead, BufReader, Lines}, path::Path};

fn main() {
    let args:Vec<String> = env::args().collect();
    let filename = args.get(1).expect("no input file path given");

    // Read file
    let lines = read_lines(filename).expect("error reading file");

    // Part 1
    // part_1(lines);

    // Part 2
    part_2(lines);
}


pub fn part_1(lines: Lines<BufReader<File>>) {
    match parse_elves(lines).iter().max_by(|a, b| b.1.cmp(&a.1)) {
        Some(elf) => println!("{}", elf.1),
        None => println!("no elves"),
    }
}

pub fn part_2(lines: Lines<BufReader<File>>) {
    let top_elves = 3;

    let mut elves = parse_elves(lines);
    elves.sort_by(|a, b| b.1.cmp(&a.1));
    elves.truncate(top_elves);

    for elf in elves.iter() {
        println!("Elf {} has {} calories", elf.0+1, elf.1);
    }

    let sum = elves.iter().map(|a| a.1).sum::<i32>();
    println!("Total calories: {}", sum);
}


// from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Elf(i32, i32);

fn parse_elves(lines: Lines<BufReader<File>>) -> Vec<Elf> {
    let mut elves:Vec<Elf> = Vec::new();
    
    let mut elf_n: i32 = 0;
    let mut elf_calories:i32 = 0;

    for line in lines.flatten() {
        match line.parse::<i32>() {
            Ok(calories) => elf_calories += calories,
            Err(_) => {
                elves.push(Elf(elf_n, elf_calories));

                elf_n += 1;
                elf_calories = 0;
            },
        }
    }

    // Don't forget the last elf
    elves.push(Elf(elf_n, elf_calories));

    return elves;
}