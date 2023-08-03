use std::fs;

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    let input = read_file();
    let (boxes_part, instructions_part) = separate_box_instructions(input);
    
    // Parse boxes
    let parsed_box_lines = parse_box_lines(boxes_part);
    let boxes = parse_boxes(parsed_box_lines);

    // Parse instructions
    let instructions = parse_instructions(instructions_part);

    // Execute instructions one by one
    let mut boxes_copy = boxes.clone();
    boxes_copy = execute_instructions(boxes_copy, &instructions);
    println!("Boxes after executing instructions one by one:");
    print_top_of_pile(boxes_copy);

    // Execute instructions with crane
    let mut boxes_copy = boxes.clone();
    boxes_copy = execute_instructions_with_crane(boxes_copy, &instructions);
    println!("Boxes after executing instructions with crane:");
    print_top_of_pile(boxes_copy);


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

fn parse_boxes(parsed_lines: Vec<Vec<char>>) -> Vec<Vec<char>> {
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

fn parse_box_lines(boxes: String) -> Vec<Vec<char>> {
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
    parsed_lines
}

fn parse_instructions(input: String) -> Vec<Instruction> {
    input.lines().map(|line| -> Instruction {
        let properties: Vec<usize> = line.split(' ').filter_map(|s| s.parse::<usize>().ok()).collect();
        Instruction {
            amount: properties[0],
            from: properties[1] - 1,
            to: properties[2] - 1,
        }
    }).collect()
}

fn execute_instructions(mut boxes: Vec<Vec<char>>, instructions: &Vec<Instruction>) -> Vec<Vec<char>> {
    for instruction in instructions {
        for _ in 0..instruction.amount {
            let popped_box = boxes[instruction.from].pop();
            boxes[instruction.to].push(popped_box.unwrap());
        }
    }
    boxes
}

fn execute_instructions_with_crane(mut boxes: Vec<Vec<char>>, instructions: &Vec<Instruction>) -> Vec<Vec<char>>{
    for instruction in instructions {
        let drain_from = boxes[instruction.from].len() - instruction.amount;
        let mut moved_boxes = boxes[instruction.from].drain(drain_from..).collect();
        boxes[instruction.to].append(&mut moved_boxes);
    }
    boxes
}

fn print_top_of_pile(boxes: Vec<Vec<char>>) {
    for box_pile in boxes {
        if box_pile.len() == 0 {
            continue;
        }
        print!("{}", box_pile.last().unwrap());
    }
    println!();
}

mod test {
    #[test]
    fn test_parse_instructions() {
        let input = "move 5 from 4 to 5\nmove 2 from 5 to 8\n";
        let instructions = super::parse_instructions(input.to_string());
        assert_eq!(instructions.len(), 2);
        assert_eq!(instructions[0].amount, 5);
        assert_eq!(instructions[0].from, 3);
        assert_eq!(instructions[0].to, 4);
        assert_eq!(instructions[1].amount, 2);
        assert_eq!(instructions[1].from, 4);
        assert_eq!(instructions[1].to, 7);
    }

    #[test]
    fn test_execute_instructions() {
        let input = "move 1 from 3 to 2\nmove 2 from 2 to 1\n";
        let instructions = super::parse_instructions(input.to_string());
        let mut boxes = vec!(vec!('a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'), vec!('q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o'), vec!('z', 'x', 'c', 'v', 'b', 'n', 'm'));
        boxes = super::execute_instructions(boxes, &instructions);
        assert_eq!(boxes[0], vec!('a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'o'));
        assert_eq!(boxes[1], vec!('q', 'w', 'e', 'r', 't', 'y', 'u', 'i'));
        assert_eq!(boxes[2], vec!('z', 'x', 'c', 'v', 'b', 'n'));
    }

    #[test]
    fn test_execute_instructions_with_crane() {
        let input = "move 1 from 3 to 2\nmove 2 from 2 to 1\n";
        let instructions = super::parse_instructions(input.to_string());
        let mut boxes = vec!(vec!('a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'), vec!('q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o'), vec!('z', 'x', 'c', 'v', 'b', 'n', 'm'));
        boxes = super::execute_instructions_with_crane(boxes, &instructions);
        assert_eq!(boxes[0], vec!('a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'o', 'm'));
        assert_eq!(boxes[1], vec!('q', 'w', 'e', 'r', 't', 'y', 'u', 'i'));
        assert_eq!(boxes[2], vec!('z', 'x', 'c', 'v', 'b', 'n'));
    }
}