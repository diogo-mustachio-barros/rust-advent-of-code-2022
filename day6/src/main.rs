
// https://adventofcode.com/2022/day/5

use std::{collections::VecDeque, env, fs::File, io::{self, BufRead, BufReader, Lines}, path::Path, usize};

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
    calc(lines, 4)
}

pub fn part_2(lines: Lines<BufReader<File>>) {
    calc(lines, 14)
}

fn calc(mut lines: Lines<BufReader<File>>, n: usize) {
    let line = lines.next().unwrap().unwrap();
    let mut sequence = line.chars();

    let mut queue: VecDeque<char> = VecDeque::new();
    let mut dup_flags = [false; 26];
    let mut count = 0;
    while queue.len() < n {
        let c = sequence.next().unwrap();
        queue.push_back(c);
        count += 1;
        
        let ord = char_to_ord(c) as usize;
        if dup_flags[ord] {
            let mut popped = queue.pop_front().unwrap();
            while popped != c {
                dup_flags[char_to_ord(popped) as usize] = false;

                popped = queue.pop_front().unwrap();
            }
        } else {
            dup_flags[ord] = true;
        }
    }

    // print solution sequence
    queue.iter().for_each(|c| print!("{}", c));
    println!();

    println!("Total count: {}", count)
}

fn char_to_ord(c: char) -> i32 {
    (c as i32) - 97
}



// from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}