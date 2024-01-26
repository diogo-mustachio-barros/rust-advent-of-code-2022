
// https://adventofcode.com/2022/day/4

use std::{path::Path, env, io::{self, BufRead, BufReader, Lines}, fs::File, str::FromStr};

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

pub fn part_1 (lines: Lines<BufReader<File>>) {
    let result = calc(lines, Range::contained_in);
    println!("{result}");
} 

pub fn part_2 (lines: Lines<BufReader<File>>) {
    let result = calc(lines, Range::overlaps_with);
    println!("{result}");
}

pub fn calc(lines: Lines<BufReader<File>>, f:fn(&Range, &Range) -> bool) -> i32 {
    let mut count = 0;

    for line in lines.flatten() {
        let (range_1_s, range_2_s) = line.split_once(",").unwrap();

        let range_1 = Range::from_str(range_1_s);
        let range_2 = Range::from_str(range_2_s);
        
        if f(&range_1, &range_2) || f(&range_2, &range_1) {
            count += 1;
        }
    }

    return count;
}

pub struct Range{min:i32, max:i32}
impl Range {
    fn contained_in(&self, other:&Self) -> bool {
        self.min >= other.min && self.max <= other.max
    }

    fn overlaps_with(&self, other:&Self) -> bool {
        self.min <= other.max && self.max >= other.min
    }

    fn from_str(s: &str) -> Self {
        let (min_s, max_s) = s.split_once("-").unwrap();

        let min = i32::from_str(min_s).unwrap();
        let max = i32::from_str(max_s).unwrap();
        
        return Range{min, max}
    }
}

// from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}