use std::fs;

fn main() {
    println!("Hello, world!");
    let input = read_file();
    let (boxes, _) = separate_box_instructions(input);
    let parsed_boxes = parse_boxes(boxes);

    println!("Boxes: {:?}", parsed_boxes);

}

fn separate_box_instructions(input: String) -> (String, String) {
    let split: Vec<&str> = input.split("\n\n").collect();
    (split[0].to_string(), split[1].to_string())
}

fn read_file() -> String {
    let contents = match fs::read_to_string("data/input.txt") {
        Ok(contents) => contents,
        Err(_) => panic!("Could not read file"),
    };
        
    contents
}

fn parse_boxes(boxes: String) -> Vec<Vec<char>> {
    let mut parsed_lines: Vec<Vec<char>> = Vec::new();
    for line in boxes.lines() {
        let mut parsed_line: Vec<char> = Vec::new();
        for (i, c) in line.chars().enumerate() {
            // Skip uneven characters (brackets or empty space) and every fourth character (column separator)
            if  (i + 1) % 4 == 0 || i % 2 == 0  {
                continue;
            }

            // Stop parsing when we reach the index row
            if c.is_ascii_digit() {
                break;
            }
            parsed_line.push(c);
        }

        // Skip empty lines
        if parsed_line.len() == 0 {
            continue;
        }

        parsed_lines.push(parsed_line);
    }

    // Make sure all lines have the same length
    assert!(parsed_lines.iter().all(|line| -> bool {
        line.len() == parsed_lines.first().unwrap().len()
    }));
    let line_len = parsed_lines.first().unwrap().len();
    let mut parsed_boxes: Vec<Vec<char>> = vec!(Vec::new(); line_len);

    for line in parsed_lines {

        for (i, c) in line.iter().enumerate() {
            if c == &' ' {
                continue;
            }
            parsed_boxes[i].insert(0, *c);
        }
    }
    parsed_boxes
}