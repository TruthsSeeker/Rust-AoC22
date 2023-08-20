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
    println!("{}", parser.get_root().unwrap().size);
    let directories = parser.find_directories(|directory| directory.size <= 100000);
    let sum = directories.iter().fold(0, |acc, directory| acc + directory.size);
    println!("{}", sum);
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
