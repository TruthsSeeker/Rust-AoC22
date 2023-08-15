use std::{fs::File, io::Read, collections::{HashMap, VecDeque}, rc::{Rc, Weak}, cell::{RefCell, Cell}};
mod directory;
use directory::Directory;


#[derive(Debug, PartialEq)]
enum LineType {
    Command,
    File,
    Directory,
}

fn main() {
    let mut cursor: Option<&mut Directory> = None;

}

fn load_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_line_type(line: &str) -> LineType {
    if line.starts_with("$") {
        LineType::Command
    } else if line.starts_with("dir") {
        LineType::Directory
    } else {
        LineType::File
    }
}

fn parse_file(line: &str) -> (usize, &str) {
    let mut iter = line.split_whitespace();
    let value = iter.next().unwrap().parse::<usize>().unwrap();
    let name = iter.next().unwrap();
    (value, name)
}

fn parse_dir(line: &str) -> &str {
    let mut iter = line.split_whitespace();
    iter.next(); // skip the "dir" part
    iter.next().unwrap()
}

fn process_ls<'a>(parent: &Rc<RefCell<Directory>>, name: &str, mut contents: VecDeque<&'a str>) -> VecDeque<&'a str> {
    let mut files = HashMap::new();
    while let Some(line) = contents.pop_front() {
        match parse_line_type(line) {
            LineType::Command => break,
            LineType::File => {
                let (size, file) = parse_file(line);
                files.insert(file.to_string(), size);
            },
            LineType::Directory => continue,
        };
        
    }
    let value = files.values().sum();
    let directory = Directory {
        value: value ,
        name: name.to_string(),
        parent: Rc::downgrade(&parent),
        children: vec![],
        files: files,
    };
    parent.borrow_mut().children.push(directory);
    parent.borrow_mut().propagate_value(value);
    contents
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line_type("$ ls"), LineType::Command);
        assert_eq!(parse_line_type("dir dfgjdlk"), LineType::Directory);
        assert_eq!(parse_line_type("123456 dfgag"), LineType::File);
    }

    #[test]
    fn test_parse_file() {
        assert_eq!(parse_file("123456 dfgag"), (123456, "dfgag"));
    }

    #[test]
    fn test_parse_dir() {
        assert_eq!(parse_dir("dir dfgjdlk"), "dfgjdlk");
    }

    #[test]
    fn test_process_ls() {
        let test_data = load_file("data/test.txt").unwrap();
        let mut contents = test_data.lines().collect::<VecDeque<&str>>();
        contents.pop_front(); // remove $ cd /
        contents.pop_front(); // remove $ ls
        let parent = Directory {
            value: 0,
            name: "root".to_string(),
            parent: Weak::new(),
            children: vec![],
            files: HashMap::new(),
        };
        let parent_rc = Rc::new(RefCell::new(parent));
        let contents = process_ls(&parent_rc, "root", contents);
        assert_eq!(parent_rc.borrow().children.len(), 1);
        assert_eq!(parent_rc.borrow().value, 23352670);
    }
}
