
// https://adventofcode.com/2022/day/1

use std::cmp::Ordering;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args:Vec<String> = env::args().collect();
    let filename;

    match args.get(1) {
        None => return (),
        Some(s) => filename = s
    }

    // Part 1
    // let elves = calc_top_n_elves(filename, 1);

    // Part 2
    let elves = calc_top_n_elves(filename, 3);
    
    let mut sum = 0;
    for elf in elves {
        println!("{elf}");
        sum += elf.1;
    }
    println!("Total: {sum}");
}

struct Elf(i32, i32);

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl Eq for Elf {
    // needed?
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Display for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Elf[n={}, calories={}]", self.0, self.1)
    }
}

fn calc_top_n_elves<P>(filename: P, n:usize) -> Vec<Elf>
where P: AsRef<Path> {
    let mut top_n:Vec<Elf> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        let mut curr_elf_n = 0;
        let mut curr_elf_calories:i32 = 0;

        for line in lines {
            if let Ok(str) = line {
                if str == "" 
                {
                    top_n = update_top_n(top_n, Elf(curr_elf_n, curr_elf_calories), n);
    
                    curr_elf_n += 1;
                    curr_elf_calories = 0;
                } 
                else 
                {
                    curr_elf_calories += str.parse::<i32>().unwrap();
                }
            }
        }

        top_n = update_top_n(top_n, Elf(curr_elf_n, curr_elf_calories), n);
    }
    
    return top_n;
}

fn update_top_n(mut top_n:Vec<Elf>, elf:Elf, n:usize) -> Vec<Elf>{
    if top_n.len() < n 
    {
        top_n = push_sorted(top_n, elf);
    } 
    else if elf.1 > top_n.get(n-1).unwrap().1 
    {
        top_n.remove(n-1);
        top_n = push_sorted(top_n, elf);
    }

    return top_n;
}

// from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn push_sorted<T:Ord+Display> (mut vec:Vec<T>, value:T) -> Vec<T> {
    let mut insert_i = 0;

    for i in 0..vec.len() {
        let elem = vec.get(i).unwrap();
        
        if let Ordering::Greater = value.cmp(elem) {
            break;
        }

        insert_i += 1;
    }
    vec.insert(insert_i, value);

    return vec;
}
