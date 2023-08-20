use std::{fs::File, io::Read, collections::{HashMap, VecDeque}, rc::{Rc, Weak}, cell::{RefCell, Cell}};
mod directory;
pub mod parser;
mod utils;
use directory::Directory;
use parser::Parser;




fn main() {
    let mut file = File::open("data/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let mut parser = Parser::new(&contents);
    parser.parse();
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
        assert_eq!(parser.get_directory(None).unwrap().size, 48381165);
    }
}
