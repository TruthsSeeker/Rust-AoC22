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
    name: &'a str,
    arena: HashMap<&'a str, Directory<'a>>,
}

impl Parser<'_> {
    pub fn new<'a>(contents: &'a str) -> Parser<'a> {
        let directory = Directory {
            size: 0,
            name: "root",
            parent: None,
            files: HashMap::new(),
            children: vec![],
        };
        let parser: Parser<'a> = Parser {
            contents: contents.lines().collect::<VecDeque<&str>>(),
            name: "root",
            arena: HashMap::from([(directory.name, directory)])
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
                        size: 0,
                        name: name,
                        parent: Some(self.name),
                        children: vec![],
                        files: HashMap::new(),
                    };
                    match self.arena.get_mut(self.name) {
                        Some(dir) => dir.children.push(directory.name),
                        None => panic!("Directory not found"),
                    }
                    self.arena.insert(name, directory);
                },
            };
            
        }
        let value = files.values().sum();
        match self.arena.get_mut(self.name) {
            Some(dir) => {
                dir.files = files;
                self.propagate_value(value);
            },
            None => panic!("Directory not found"),
        }
    }

    fn process_cd(self: &mut Self) {
        if let Some(line) = self.contents.pop_front() {
            if let Some(name) = line.split_whitespace().nth(2) {
                match name {
                    ".." => {
                        if let Some(parent) = self.arena[self.name].parent {
                            self.name = parent;
                        }
                    },
                    _ => {
                        if let Some(dir) = self.arena.get(name) {
                            self.name = name;
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

    pub fn get_root(self: &Self) -> Option<Directory> {
        match self.arena.get(self.name) {
            Some(mut directory) => {
                while let Some(parent) = directory.parent {
                    if let Some(dir) = self.arena.get(parent) {
                        directory = dir;
                    }
                }
                return Some(directory.clone());
            },
            None => return None,
        };
    }

    pub fn propagate_value(self: &mut Self, value: usize) {
        match self.arena.get_mut(self.name) {
            Some(directory) => directory.size += value,
            None => panic!("Directory not found"),
        }
        while let Some(parent) = self.arena[self.name].parent {
            match self.arena.get_mut(parent) {
                Some(directory) => directory.size += value,
                None => break,
            }
            self.name = parent;
        }
    }


    pub fn get_directory(self: &Self, name: Option<&str>) -> Option<&Directory> {
        if let Some(name) = name {
            return self.arena.get(name)
        }
        self.arena.get(self.name)
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
        assert_eq!(parser.arena["root"].children.len(), 2);
        assert_eq!(parser.arena["root"].size, 23352670);
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
        assert_eq!(parser.get_directory(None).unwrap().name, "a");
        assert_eq!(parser.get_directory(None).unwrap().size, 0);
        
    }

    #[test]
    fn test_parse() {
        let test_data = load_file("data/test.txt").unwrap();
        let mut parser = Parser::new(&test_data);
        parser.parse();
        let root = parser.get_root();
        assert_eq!(root.unwrap().size, 48381165);
    }
}



