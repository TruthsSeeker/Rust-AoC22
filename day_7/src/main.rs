use std::{fs::File, io::Read};
mod directory;
use directory::Directory;


#[derive(Debug, PartialEq)]
enum LineType {
    Command,
    File,
    Directory,
}

fn main() {

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




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line_type("$ ls"), LineType::Command);
        assert_eq!(parse_line_type("dir dfgjdlk"), LineType::Directory);
        assert_eq!(parse_line_type("123456 dfgag"), LineType::File);
    }
}
