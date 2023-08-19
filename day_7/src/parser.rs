use std::{rc::{Rc, Weak}, collections::{VecDeque, HashMap}, cell::RefCell};

use crate::directory::Directory;
use crate::utils::load_file;

#[derive(Debug, PartialEq)]
pub enum LineType {
    Command,
    File,
    Directory,
}

pub struct Parser<'a> {
    contents: VecDeque<&'a str>,
    root: Rc<RefCell<Directory>>,
    pub cursor: Rc<RefCell<Directory>>,
    name: &'a str,
}

impl Parser<'_> {
    pub fn new<'a>(contents: &'a str) -> Parser<'a> {
        let directory = Directory {
            value: 0,
            name: "root".to_string(),
            parent: Weak::new(),
            children: HashMap::new(),
            files: HashMap::new(),
        };
        let parser: Parser<'a> = Parser {
            contents: contents.lines().collect::<VecDeque<&str>>(),
            root: Rc::new(RefCell::new(directory.clone())),
            cursor: Rc::new(RefCell::new(directory)),
            name: "root",
        };
        parser
    }

    fn parse_line_type(self: &Self, line: &str) -> LineType {
        if line.starts_with("$") {
            LineType::Command
        } else if line.starts_with("dir") {
            LineType::Directory
        } else {
            LineType::File
        }
    }
    
    fn parse_file<'a>(self: &Self, line: &'a str) -> (usize, &'a str) {
        let mut iter = line.split_whitespace();
        let value = iter.next().unwrap().parse::<usize>().unwrap();
        let name: &'a str = iter.next().unwrap();
        (value, name)
    }
    
    fn parse_dir_name<'a>(self: &Self, line: &'a str) -> &'a str {
        let mut iter = line.split_whitespace();
        iter.next(); // skip the "dir" part
        iter.next().unwrap()
    }
    
    fn process_ls<'a>(self: &mut Self) {
        self.contents.pop_front(); // remove $ ls
        let mut files = HashMap::new();
        while let Some(line) = self.contents.pop_front() {
            match self.parse_line_type(line) {
                LineType::Command => {
                    self.contents.push_front(line);
                    break;
                },
                LineType::File => {
                    let (size, file) = self.parse_file(line);
                    files.insert(file.to_string(), size);
                },
                LineType::Directory => {
                    let name = self.parse_dir_name(line);
                    let directory = Directory {
                        value: 0,
                        name: name.to_string(),
                        parent: Rc::downgrade(&self.cursor),
                        children: HashMap::new(),
                        files: HashMap::new(),
                    };
                    self.cursor.borrow_mut().children.insert(name.to_string(), Rc::new(RefCell::new(directory)));
                },
            };
            
        }
        let value = files.values().sum();
        let mut directory = self.cursor.borrow_mut();
        directory.files = files;
        directory.propagate_value(value);
    }

    fn process_cd(self: &mut Self) {
        if let Some(line) = self.contents.pop_front() {
            if let Some(name) = line.split_whitespace().nth(2) {
                match name {
                    ".." => {
                        if let Some(parent) = self.cursor.clone().borrow().parent.upgrade() {
                            self.cursor = parent;
                        }
                    },
                    _ => {
                        if let Some(child) = self.cursor.clone().borrow().children.get(name) {
                            self.name = name;
                            self.cursor = Rc::new(RefCell::new(child.borrow().clone()));
                        }
                    }
                }
            }
        }
    }

    fn process_command(self: &mut Self) {
        if let Some(line) = self.contents.front() {
            match self.parse_line_type(line) {
                LineType::Command => {
                    if let Some(command) = line.split_whitespace().nth(1) {
                        match command {
                            "ls" => self.process_ls(),
                            "cd" => self.process_cd(),
                            _ => panic!("Unknown command"),
                        }
                    }
                }
                _ => panic!("Expected command"),
            }
        }
    }

    pub fn parse(self: &mut Self) {
        while !self.contents.is_empty() {
            self.process_command();
        }
    }
}

#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn test_parse_line() {
        let contents = load_file("data/test.txt").unwrap();
        let mut parser = Parser::new(&contents);
        assert_eq!(parser.parse_line_type("$ ls"), LineType::Command);
        assert_eq!(parser.parse_line_type("dir dfgjdlk"), LineType::Directory);
        assert_eq!(parser.parse_line_type("123456 dfgag"), LineType::File);
    }

    #[test]
    fn test_parse_file() {
        let parser = Parser::new("");
        assert_eq!(parser.parse_file("123456 dfgag"), (123456, "dfgag"));
    }

    #[test]
    fn test_parse_dir() {
        let parser = Parser::new("");
        assert_eq!(parser.parse_dir_name("dir dfgjdlk"), "dfgjdlk");
    }

    #[test]
    fn test_process_ls() {
        let test_data = load_file("data/test.txt").unwrap();
        let mut contents = test_data.lines().collect::<VecDeque<&str>>();
        contents.pop_front(); // remove $ cd /
        let truncated_contents = contents.into_iter().collect::<Vec<&str>>().join("\n");
        let mut parser = Parser::new(&truncated_contents);
        parser.process_ls();
        assert_eq!(parser.cursor.borrow().children.len(), 2);
        assert_eq!(parser.cursor.borrow().value, 23352670);
    }

    #[test]
    fn test_process_cd() {
        let test_data = load_file("data/test.txt").unwrap();
        let mut contents = test_data.lines().collect::<VecDeque<&str>>();
        contents.pop_front(); // remove $ cd /
        let truncated_contents = contents.into_iter().collect::<Vec<&str>>().join("\n");
        let mut parser = Parser::new(&truncated_contents);
        parser.process_ls();
        parser.process_cd();
        assert_eq!(parser.cursor.borrow().name, "a");
        assert_eq!(parser.cursor.borrow().value, 0);
        
    }

    #[test]
    fn test_parse() {
        let test_data = load_file("data/test.txt").unwrap();
        let mut parser = Parser::new(&test_data);
        parser.parse();
        let root = parser.cursor.borrow().get_root();
        assert_eq!(root.value, 48381165);
    }
}



