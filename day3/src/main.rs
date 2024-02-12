
// https://adventofcode.com/2022/day/3

use std::{io::{BufReader, Lines}, fs::File};

use util::advent_of_code::redirect;

fn main() {
    redirect(part_1, part_2);
}

pub fn part_1(lines: Lines<BufReader<File>>) {
    let priority_sum:i32 = lines.flatten()
                                .map(|line| total_priority(&line))
                                .sum();

    println!("Total sum: {}", priority_sum);
}

pub fn part_2(lines: Lines<BufReader<File>>) {
    let n = 3;
    let mut sum_badges = 0;
    
    let mut line_count = 0;
    let mut vec:Vec<String> = Vec::new();
    for line in lines {
        // accumulate n lines (group of rucksacks)
        if line_count < n {
            vec.push(line.unwrap());
            line_count += 1;
            if line_count < n { continue }
        }
        line_count = 0;

        // search badge and calculate priority
        let badge_priority = search_badge(&vec).unwrap();
        // sum badges
        sum_badges += badge_priority;

        // clear vector for next group
        vec.clear();
    }

    println!("Total sum: {}", sum_badges);
}

fn total_priority(rucksack:&str) -> i32 {
    // rucksack is guaranteed to have two halves (aka even length)
    let (left, right) = rucksack.split_at(rucksack.len() / 2);

    // enunciate guarantees a single duplicate always, we can safely unwrap
    let duplicate = search_duplicate(left, right).unwrap();
    
    // translate duplicate into priority
    return char_to_priority(duplicate).into();
}

// searches for a single duplicate, enunciate guarantees it
pub fn search_duplicate(left:&str, right:&str) -> Option<char>{
    let mut flag_array = [false; 26*2];
    
    for c in left.chars() {
        let priority:usize = char_to_priority(c).try_into().unwrap();
        flag_array[priority-1] = true;
    }

    for c in right.chars() {
        let priority:usize = char_to_priority(c).try_into().unwrap();
        if flag_array[priority-1] {
            return Some(c);
        }
    }

    return None;
}

// to search a badge is the same as a n-way duplicate search
pub fn search_badge(vec:&Vec<String>) -> Option<i32> {
    let mut counting_array = [0; 26*2];
    let mut flag_array = [false; 26*2];

    for str in vec {
        // raise flag array
        for c in str.chars() {
            let priority:usize = char_to_priority(c).into();
            flag_array[priority-1] = true;
        }

        // increment count array
        for i in 0..52 {
            if flag_array[i] {
                counting_array[i] += 1;
            }
        }

        // reset flags
        flag_array = [false; 26*2];
    }

    // check which one is duplicated and return it
    for i in 0..52 {
        if counting_array[i] == vec.len() {
            return Some(i as i32 + 1);
        }
    }
    
    return None
}

pub fn char_to_priority(c:char) -> u8 {
    // a = 01100001   A = 01000001
    // z = 01111010   Z = 01011010

    // truncate the first 3 bits to keep it under 31
    let base_priority = (c as u8) & 0b00011111;

    if c.is_ascii_lowercase() {
        return base_priority;
    } else {
        return base_priority + 26;
    }
}