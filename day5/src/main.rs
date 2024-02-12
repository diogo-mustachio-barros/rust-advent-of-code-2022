
// https://adventofcode.com/2022/day/5

use std::{fs::File, io::{BufReader, Lines}, usize};
use regex::Regex;
use util::advent_of_code::redirect;

fn main() {
    redirect(part_1, part_2);
}

pub fn part_1 (lines: Lines<BufReader<File>>) {
    calc(lines, crate_mover_9000_loader, crate_mover_9000_unloader)
} 

pub fn part_2 (lines: Lines<BufReader<File>>) {
    calc(lines, crate_mover_9001_loader, crate_mover_9001_unloader)
} 

fn calc( lines: Lines<BufReader<File>>
       , loader: fn (&mut Vec<char>, i32) -> Vec<char>
       , unloader: fn (&mut Vec<char>, Vec<char>)
       ) {
    // parse input
    let (mut stacks, moves) = parse_input(lines);
    
    // apply moves inplace
    apply_moves( &mut stacks
               , moves
               , loader
               , unloader);
    
    // get top element from each stack and concat them
    for mut stack in stacks {
        let top = stack.pop().unwrap();
        print!("{}", top);
    }
    println!();
}

// crate mover 9000
fn crate_mover_9000_loader (from:&mut Vec<char>, amount:i32) -> Vec<char> {
    return (0..amount).map(|_| from.pop().unwrap()).collect();
}

fn crate_mover_9000_unloader (to:&mut Vec<char>, items:Vec<char>) {
    items.iter().for_each(|item| to.push(*item))
} 

// crate mover 9001
fn crate_mover_9001_loader (from:&mut Vec<char>, amount:i32) -> Vec<char> {
    return (0..amount).map(|_| from.pop().unwrap()).collect();
}

fn crate_mover_9001_unloader (to:&mut Vec<char>, mut items:Vec<char>) {
    items.reverse();
    to.append(&mut items);
} 



struct Move {
    amount:i32,
    from:usize,
    to:usize,
}

fn parse_input(lines: Lines<BufReader<File>>) -> (Vec<Vec<char>>, Vec<Move>) {
    
    // flatten out lines to avoid unwraps and such
    let mut lines = lines.flatten();
    

    // -- parse stacks until space


    let mut line = lines.next().unwrap();

    // how many stacks are there
    let n = (line.len() + 1) / 4;

    // initialize stacks
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0 .. n {
        stacks.push(Vec::new());
    }

    // iterate lines and assign each char to its stack
    let mut i;
    while line != "" {
        i = 1;

        for c in line.chars() {
            if c.is_alphabetic() {
                stacks.get_mut((i-1)/4).unwrap().push(c);
            }

            i += 1;
        }

        line = lines.next().unwrap();
    } 
    
    // invert all stacks
    for i in 0..n {
        stacks.get_mut(i).unwrap().reverse();
    }


    // -- parse moves until eof


    // regex to parse moves
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let mut moves: Vec<Move> = Vec::new();

    for line in lines {
        let matches = re.captures(&line).unwrap();
        let (_, [amount_s, from_s, to_s]) = matches.extract();
        
        let amount = amount_s.parse().unwrap();
        let from = from_s.parse::<usize>().unwrap() - 1;
        let to = to_s.parse::<usize>().unwrap() - 1;
        
        moves.push(Move{amount, from, to});
    }

    return (stacks, moves);
}

fn apply_moves( stacks: &mut Vec<Vec<char>>
              , moves: Vec<Move>
              , loader: fn (&mut Vec<char>, i32) -> Vec<char>
              , unloader: fn (&mut Vec<char>, Vec<char>)
              ) {
    for m in moves {
        let from = stacks.get_mut(m.from).unwrap();
        let items = loader(from, m.amount);
        
        let to = stacks.get_mut(m.to).unwrap();
        unloader(to, items);
    }
}
