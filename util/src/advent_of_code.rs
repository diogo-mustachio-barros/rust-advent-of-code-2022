use std::{env, fs::File, io::{BufReader, Lines}};

use crate::files::read_lines;

pub fn redirect(part_1: fn(Lines<BufReader<File>>), part_2: fn(Lines<BufReader<File>>)) 
{
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