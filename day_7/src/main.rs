use std::{fs::File, io::Read, collections::VecDeque};
mod directory;
pub mod parser;
mod utils;

use parser::Parser;

use crate::utils::load_file;




fn main() {
    let mut contents = load_file("data/input.txt").unwrap();
    
    let mut parser = Parser::new(&contents);
    parser.parse();
    let total_size = parser.get_root().unwrap().size;
    println!("{}", total_size);
    let directories = parser.find_directories(|directory| directory.size <= 100000);
    let sum = directories.iter().fold(0, |acc, directory| acc + directory.size);
    println!("{}", sum);
    
    let min_size_to_delete = 30000000 - (70000000 - total_size);
    let mut smallest_candidate = usize::max_value();
    parser.arena.iter().for_each(|directory| {
        if directory.size >= min_size_to_delete && directory.size < smallest_candidate {
            smallest_candidate = directory.size;
        }
    });
    println!("delete: {}", smallest_candidate);
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test() {
        let mut file = File::open("data/test.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        
        let mut parser = Parser::new(&contents);
        parser.parse();
        assert_eq!(parser.arena[0].size, 48381165);
    }
}
