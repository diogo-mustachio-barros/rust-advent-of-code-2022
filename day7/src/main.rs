
// https://adventofcode.com/2022/day/7

use std::{collections::HashMap, env, fs::File, hash::Hash, io::{self, BufRead, BufReader, Lines}, path::Path};

use regex::Regex;

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
        if let (FileType::Dir, size) = fs.value {
            if size <= 100000 {
                acc.push((FileType::Dir, size)); 
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
    if let (FileType::File, size) = fs.value 
    {
        return (FileType::File, size);
    }
    else
    {
        let mut total_size = 0;
        for child in fs.children.values() {
            total_size += child.value.1;
        }
        return (FileType::Dir, total_size);
    }
}

pub fn part_2(lines: Lines<BufReader<File>>) {
    // parse input
    let mut fs = parse_input(lines);
    
    // recursively calculate file and dir sizes
    fs.map_values(calc_sizes); 

    let total_size = fs.value.1;
    let max_size = 70000000;
    let update_size = 30000000;
    let minimum_size = update_size - (max_size - total_size);
    // println!("{}", minimum_size);

    let dir = fs.fold(&move |fs, best: Item| {
        if let (FileType::Dir, size) = fs.value {
            if size >= minimum_size && size < best.1 {
                return (FileType::Dir, size);
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

// non-empty tree for simplicity
struct AddressableTree<K, V> {
    key: K,
    value: V,
    children: HashMap<K, AddressableTree<K, V>>,
}

impl <K: Eq + Hash + PartialEq + Clone, V> AddressableTree<K, V> {
    pub fn singleton(key: K, value: V) -> AddressableTree<K, V> {
        AddressableTree { key: key
                        , value: value
                        , children: HashMap::new() }
    }

    pub fn add_child(&mut self, key: K, value: V) {
        let key_clone = key.clone();
        self.children.insert(key, AddressableTree::singleton(key_clone, value));
    }

    pub fn add_child_node(&mut self, key: K, node:AddressableTree<K, V>) {
        self.children.insert(key, node);
    }

    pub fn remove_child(&mut self, key: &mut K) -> AddressableTree<K, V> {
        self.children.remove(key).unwrap()
    }

    pub fn map_values(&mut self, f: fn(&AddressableTree<K, V>) -> V)
    {
        // map children
        for child in self.children.values_mut() {
            child.map_values(f);
        }

        // map value
        self.value = f(&self);
    }

    pub fn fold<T>(&self, f: &impl Fn(&AddressableTree<K, V>, T) -> T, initial: T) -> T
    {
        let mut acc = initial;

        for child in self.children.values() {
            acc = child.fold(f, acc);
        }

        acc = f(self, acc);

        return acc;
    }
}

struct TreeNavigator<K, V> {
    previous: Vec<AddressableTree<K, V>>,
    current: AddressableTree<K, V>,
}

impl <K: Eq + Hash + PartialEq + Clone, V> TreeNavigator<K, V> {
    pub fn new(tree: AddressableTree<K, V>) -> TreeNavigator<K, V> {
        TreeNavigator {previous: Vec::new(), current: tree}
    }

    pub fn go_into(mut self, key: &mut K) -> Self {
        let mut current = self.current;
        let new_current = current.remove_child(key);

        self.previous.push(current);
        self.current = new_current;

        return self;
    }

    pub fn get_out(mut self) -> Self {
        let current = self.current;
        let mut new_current = self.previous.pop().unwrap();

        new_current.add_child_node(current.key.clone(), current);

        self.current = new_current;
        return self;
    }

    pub fn apply_to_current<F>(mut self, f: F) -> Self
    where F:FnOnce(AddressableTree<K, V>) -> AddressableTree<K, V> {
        self.current = f(self.current);
        return self;
    }

    pub fn get(mut self) -> AddressableTree<K, V> {

        // collapse all explored trees back into shape
        while !self.previous.is_empty()
        {
            let mut previous_tree = self.previous.pop().unwrap();

            previous_tree.add_child_node(self.current.key.clone(), self.current);

            self.current = previous_tree;
        }
        
        return self.current;
    }
}

// from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}