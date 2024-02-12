
// https://adventofcode.com/2022/day/4

use std::{io::{BufReader, Lines}, fs::File, str::FromStr};

use util::advent_of_code::redirect;

fn main() {
    redirect(part_1, part_2);
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