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
    cursor: Rc<RefCell<Directory>>,
    name: &'a str,
}

impl Parser<'_> {
    fn new<'a>(contents: &'a str) -> Parser<'a> {

        let parser: Parser<'a> = Parser {
            contents: contents.lines().collect::<VecDeque<&str>>(),
            cursor: Rc::new(RefCell::new(Directory {
                value: 0,
                name: "root".to_string(),
                parent: Weak::new(),
                children: vec![],
                files: HashMap::new(),
            })),
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
    
    pub fn process_ls<'a>(self: &mut Self) {
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
                LineType::Directory => continue,
            };
            
        }
        let value = files.values().sum();
        let directory = Directory {
            value: value ,
            name: self.name.to_string(),
            parent: Rc::downgrade(&self.cursor),
            children: vec![],
            files: files,
        };
        self.cursor.borrow_mut().children.push(directory);
        self.cursor.borrow_mut().propagate_value(value);
    }

    pub fn process_cd(self: &mut Self) {
        if let Some(line) = self.contents.pop_front() {
            let name = self.parse_dir_name(line);
            match name {
                ".." => {
                    if let Some(parent) = self.cursor.clone().borrow().parent.upgrade() {
                        self.cursor = parent;
                    }
                },
                _ => {
                    self.name = name;
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use std::rc::Weak;

    use crate::parser;

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
        contents.pop_front(); // remove $ ls
        let truncated_contents = contents.into_iter().collect::<Vec<&str>>().join("\n");
        let mut parser = Parser::new(&truncated_contents);
        parser.process_ls();
        assert_eq!(parser.cursor.borrow().children.len(), 1);
        assert_eq!(parser.cursor.borrow().value, 23352670);
    }
}



