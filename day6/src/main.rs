
// https://adventofcode.com/2022/day/6

use std::{collections::VecDeque, fs::File, io::{BufReader, Lines}, usize};

use util::advent_of_code::redirect;

fn main() {
    redirect(part_1, part_2);
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