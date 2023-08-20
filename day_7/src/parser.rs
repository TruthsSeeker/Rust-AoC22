use std::collections::{VecDeque, HashMap};

use crate::directory::Directory;

#[derive(Debug, PartialEq)]
pub enum LineType {
    Command,
    File,
    Directory,
}

pub struct Parser<'a> {
    contents: VecDeque<&'a str>,
    // name: &'a str,
    pub arena: Vec<Directory<'a>>,
    cursor: usize,
}

//TODO: update directories as the file structure gets parsed

impl Parser<'_> {
    pub fn new<'a>(contents: &'a str) -> Parser<'a> {
        let directory = Directory {
            idx: 0,
            size: 0,
            name: "root",
            parent: None,
            files: HashMap::new(),
            children: vec![],
        };
        let parser: Parser<'a> = Parser {
            contents: contents.lines().collect::<VecDeque<&str>>(),
            // name: "root",
            cursor: 0,
            arena: vec![directory]
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
                        idx: self.arena.len(),
                        size: 0,
                        name: name,
                        parent: Some(self.cursor),
                        children: vec![],
                        files: HashMap::new(),
                    };
                    self.arena[self.cursor].children.push(directory.idx);
                    // match self.arena.get(self.cursor) {
                    //     Some(dir) => dir.children.push(directory.idx),
                    //     None => panic!("Directory not found"),
                    // }
                    self.arena.push(directory)
                },
            };
            
        }
        let value = files.values().sum();
        match self.arena.get_mut(self.cursor) {
            Some(dir) => {
                files.iter().for_each(|(file, size)| {
                    dir.files.insert(file.to_string(), *size);
                });
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
                        if let Some(parent) = self.arena[self.cursor].parent {
                            self.cursor = parent;
                            // self.name = self.arena[self.cursor].name;
                        }
                    },
                    _ => {
                        if let Some(dir) = self.find_among(
                            &self.arena[self.cursor].children, 
                            |idx| self.arena[idx].name == name) {
                            // self.name = name;
                            self.cursor = dir.idx;
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
        match self.arena.get(self.cursor) {
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
        let mut cursor = self.cursor;
        match self.arena.get_mut(cursor) {
            Some(directory) => directory.size += value,
            None => panic!("Directory not found"),
        }
        while let Some(parent) = self.arena[cursor].parent {
            match self.arena.get_mut(parent) {
                Some(directory) => directory.size += value,
                None => break,
            }
            cursor = parent;
        }
    }
    
    pub fn find_directories<F>(self: &Self, predicate: F) -> Vec<&Directory> 
    where F: Fn(&Directory) -> bool {
        let mut directories = vec![];
        for directory in self.arena[..].iter() {
            if predicate(&directory) {
                directories.push(directory);
            }
        }
        directories
    }

    fn find_among<F>(self: &Self, idxs: &[usize], predicate: F) -> Option<&Directory>
    where F: Fn(usize) -> bool {
        for idx in idxs {
            let index = idx.clone();
            if predicate(*idx) {
                return Some(&self.arena[index]);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    
    
    use super::*;
    use crate::utils::load_file;
    
    #[test]
    fn test_parse_line() {
        let contents = load_file("data/test.txt").unwrap();
        let parser = Parser::new(&contents);
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
        // assert_eq!(parser.arena[0].children.len(), 2);
        assert_eq!(parser.arena[0].size, 23352670);
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
        let binding = parser.find_directories(|a| a.name == "a");
        let a_dir = binding.first().unwrap();
        assert_eq!(a_dir.name, "a");
        assert_eq!(a_dir.size, 0);
        
    }

    #[test]
    fn test_parse() {
        let test_data = load_file("data/test.txt").unwrap();
        let mut parser = Parser::new(&test_data);
        parser.parse();
        let root = parser.get_root();
        assert_eq!(root.unwrap().size, 48381165);
    }

    #[test]
    fn test_find_directories() {
        let test_data = load_file("data/test.txt").unwrap();
        let mut parser = Parser::new(&test_data);
        parser.parse();
        let directories = parser.find_directories(|dir| dir.size <= 100000);
        assert_eq!(directories.len(), 2);
    }
}



