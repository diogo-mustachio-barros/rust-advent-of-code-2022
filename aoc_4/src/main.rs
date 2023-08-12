use std::{path::Path, env, io::{self, BufRead}, fs::File, str::FromStr};

fn main() {
    let args:Vec<String> = env::args().collect();
    let filename;

    match args.get(1) {
        None => return (),
        Some(s) => filename = s
    }

    let result = calc(filename, Range::contained_in);
    println!("{result}");

    let result = calc(filename, Range::overlaps_with);
    println!("{result}");
}

pub fn calc<P>(filename: P, f:fn(&Range, &Range) -> bool) -> i32
where P: AsRef<Path> {
    let mut count = 0;

    if let Ok(lines) = read_lines(filename) {
        for r_line in lines {
            if let Ok(line) = r_line {
                let (range_1_s, range_2_s) = line.split_once(",").unwrap();

                let range_1 = Range::from_str(range_1_s).unwrap();
                let range_2 = Range::from_str(range_2_s).unwrap();
                
                if f(&range_1, &range_2) || f(&range_2, &range_1) {
                    count += 1;
                }
            }
        }
    }

    return count;
}

pub struct Range{min:i32, max:i32}
impl Range {
    fn contained_in(&self, other:&Self) -> bool {
        if self.min >= other.min && self.max <= other.max {
            return true;
        }
        return false;
    }

    fn overlaps_with(&self, other:&Self) -> bool {
        if self.min > other.max || self.max < other.min {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct RangeParseError;
impl FromStr for Range {
    type Err = RangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min_s, max_s) = s.split_once("-").unwrap();

        let min = i32::from_str(min_s).unwrap();
        let max = i32::from_str(max_s).unwrap();
        
        return Ok(Range{min, max})
    }
}

// from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}