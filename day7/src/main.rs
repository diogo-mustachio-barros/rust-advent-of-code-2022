
// https://adventofcode.com/2022/day/7

use std::{fs::File, io::{BufReader, Lines}};

use regex::Regex;
use util::{addressable_tree::AddressableTree, advent_of_code::redirect, tree_navigator::TreeNavigator};

fn main() {
    redirect(part_1, part_2);
}

type FileSystem = AddressableTree<String, Item>;
type Item = (FileType, i32);

enum FileType {
    File,
    Dir
} 

pub fn part_1(lines: Lines<BufReader<File>>) {
    
    // parse input
    let mut fs = parse_input(lines);
    
    // recursively calculate file and dir sizes
    fs.map_values(calc_sizes); 

    // gather all dirs with size under 100000
    let dirs = fs.fold(&|fs, mut acc: Vec<Item>| {
        if let (FileType::Dir, size) = fs.get_value() {
            if *size <= 100000 {
                acc.push((FileType::Dir, *size)); 
            }
        }
        return acc;
    }, Vec::new());

    // sum all dir sizes
    let mut sum_sizes = 0;
    for dir in dirs {
        sum_sizes += dir.1;
    }

    println!("Total sum size: {}", sum_sizes);
}

fn calc_sizes(fs: &FileSystem) -> Item {
    if let (FileType::File, size) = fs.get_value() 
    {
        return (FileType::File, *size);
    }
    else
    {
        let mut total_size = 0;
        for child in fs.get_children() {
            total_size += child.get_value().1;
        }
        return (FileType::Dir, total_size);
    }
}

pub fn part_2(lines: Lines<BufReader<File>>) {
    // parse input
    let mut fs = parse_input(lines);
    
    // recursively calculate file and dir sizes
    fs.map_values(calc_sizes); 

    let total_size = fs.get_value().1;
    let max_size = 70000000;
    let update_size = 30000000;
    let minimum_size = update_size - (max_size - total_size);
    // println!("{}", minimum_size);

    let dir = fs.fold(&move |fs, best: Item| {
        if let (FileType::Dir, size) = fs.get_value() {
            if *size >= minimum_size && *size < best.1 {
                return (FileType::Dir, *size);
            }
        }
        return best;
    }, (FileType::Dir, max_size));

    println!("Result: {}", dir.1);
}

fn parse_input(lines: Lines<BufReader<File>>) -> FileSystem {
    // - first "cd /" is ignored
    // - ls adds children to the current node
    //     - dir has size 0
    //     - file has some size
    // - cd changes nodes
    //     - cd <node> goes into a child node
    //     - cd .. goes back to the parent node

    let commands = group_commands(lines);

    let root_key = "/".to_string();
    let tree: FileSystem = AddressableTree::singleton(root_key, (FileType::Dir, 0));
    let mut tree_nav = TreeNavigator::new(tree);
    
    for (command, output) in commands {
        if command == "$ cd /" 
        {
            // do nothing
        }
        else if command.starts_with("$ cd ..") 
        {
            tree_nav = tree_nav.get_out()
        } 
        else if command.starts_with("$ cd") 
        {
            let mut key = command[5..].to_string();
            tree_nav = tree_nav.go_into(&mut key);
        } 
        else if command.starts_with("$ ls") 
        {
            for output_line in output {
                if output_line.starts_with("dir") 
                {
                    // case of dir
                    let key = output_line[4..].to_string();
                    
                    tree_nav = tree_nav.apply_to_current(move |mut t| {
                        t.add_child(key, (FileType::Dir, 0));
                        return t;
                    });
                } 
                else 
                {
                    // case of file with size
                    let re = Regex::new(r"(\d+) (.+)").unwrap();
                    let matches = re.captures(&output_line).unwrap();
                    let (_, [size_s, filename_s]) = matches.extract();

                    let size = size_s.parse::<i32>().unwrap();
                    let filename = filename_s.to_string();

                    tree_nav = tree_nav.apply_to_current(move |mut t| {
                        t.add_child(filename, (FileType::File, size));
                        return t;
                    });
                }
            }
        }
    }

    return tree_nav.get();
}

fn group_commands(mut lines: Lines<BufReader<File>>) -> Vec<(String, Vec<String>)> {
    let mut grouped_lines: Vec<(String, Vec<String>)> = Vec::new();
    
    let mut command: String;
    let mut command_output: Vec<String>;
    
    // first case
    command = lines.next().unwrap().unwrap();
    command_output = Vec::new();

    for line in lines.flatten() {
        if line.starts_with("$") {
            grouped_lines.push((command, command_output));

            command = line;
            command_output = Vec::new();
        } else {
            command_output.push(line);
        }
    }

    grouped_lines.push((command, command_output));

    return grouped_lines;
}