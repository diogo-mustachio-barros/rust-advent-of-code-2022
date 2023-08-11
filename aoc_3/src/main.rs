use std::{env, path::Path, io::{self, BufRead}, fs::File};

fn main() {
    let args:Vec<String> = env::args().collect();
    let filename;

    match args.get(1) {
        None => return (),
        Some(s) => filename = s
    }

    let sum = part1(filename);
    println!("{sum}");

    let sum = part2(filename, 3);
    println!("{sum}");
}

pub fn part1<P>(filename:P) -> i32
where P: AsRef<Path> {
    let mut priority_sum:i32 = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(str) = line 
            {
                priority_sum += total_priority(&str)
            }
        }
    }

    return priority_sum;
}

fn part2<P>(filename:P, n:usize) -> i32
where P: AsRef<Path> {
    let mut sum_badges = 0;
    
    if let Ok(lines) = read_lines(filename) 
    {
        let mut line_count = 0;
        let mut vec:Vec<String> = Vec::new();
        for line in lines {
            if line_count < n {
                vec.push(line.unwrap());
                line_count += 1;
                if line_count < n { continue }
            }
            line_count = 0;

            let badge = search_badge(&vec).unwrap();
            vec.clear();
            
            sum_badges += badge;
        }
    }

    return sum_badges
}

fn total_priority(rucksack:&str) -> i32 {
    let (left, right) = rucksack.split_at(rucksack.len() / 2);
    let duplicate = search_duplicate(left, right).unwrap();
    return char_to_priority(duplicate).into();
}

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

pub fn search_badge(vec:&Vec<String>) -> Option<i32> {
    if vec.is_empty() {
        return None
    }

    let mut counting_array = [0; 26*2];
    let mut flag_array = [false; 26*2];

    for i in 0..vec.len()-1 {
        let str = vec.get(i).unwrap();
        for c in str.chars() {
            let priority:usize = char_to_priority(c).try_into().unwrap();
            if !flag_array[priority-1] {
                counting_array[priority-1] += 1;
                flag_array[priority-1] = true;
            }
        }

        // reset flags
        flag_array = [false; 26*2];
    }

    for c in vec.get(vec.len()-1).unwrap().chars() {
        let priority:usize = char_to_priority(c).try_into().unwrap();
        if counting_array[priority-1] == vec.len()-1 {
            return Some(priority as i32);
        }
    }
    
    return None
}

pub fn char_to_priority(c:char) -> u8 {
    // a = 01100001   A = 01000001
    // z = 01111010   Z = 01011010

    let base_priority = (c as u8) & 0b11111;

    if c.is_ascii_lowercase() {
        return base_priority.into();
    } else {
        return (base_priority + 26).into();
    }
}


// from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}