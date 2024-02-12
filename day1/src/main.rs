
// https://adventofcode.com/2022/day/1

use std::{fs::File, io::{BufReader, Lines}};

use util::advent_of_code::redirect;

fn main() {
    redirect(part_1, part_2);
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