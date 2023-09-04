
fn main() {
    let input = load_file("data/input.txt");
    let mut processor = Processor::new();
    let commands = parse_commands(&input);
    let mut result: i64 = 0;
    for command in commands {
        match command {
            Command::Noop => {
                let value = processor.process_noop();
                if value.is_some() {
                    result += value.unwrap();
                }
            },
            Command::AddX(value) => {
                let value = processor.process_addx(value);
                if value.is_some() {
                    result += value.unwrap();
                }
            },
        }
    }
    println!("Result: {}", result);
    processor.draw();
}

struct Processor {
    cycle: i64,
    register: i64,
    screen: Vec<Vec<bool>>,
}

impl Processor {

    pub fn new() -> Self {
        Processor {
            cycle: 1,
            register: 1,
            screen: vec![vec![false; 40]; 6],
        }
    }
    
    fn check_for_sampling(self: &Self) -> bool {
        (self.cycle - 20) % 40 == 0
    }
    
    pub fn process_noop(self: &mut Self) -> Option<i64> {
        let mut result: Option<i64> = None;
        self.process(&mut result);
        result
    }
    
    pub fn process_addx(self: &mut Self, value: i64) -> Option<i64> {
        let mut result: Option<i64> = None;
        self.process(&mut result);
        self.process(&mut result);
        self.register += value;
        result
    }

    fn process(self: &mut Self, result: &mut Option<i64>) {
        if self.check_for_sampling() {
            *result = Some(self.register * self.cycle);
        }
        let sprite_pixels = vec![self.register -1, self.register, self.register + 1];
        let current_pixel = (self.cycle - 1) % 40;
        if sprite_pixels.contains(&current_pixel) {
            self.draw_pixel();
        }
        self.cycle += 1;
    }

    fn draw_pixel(self: &mut Self) {
        let x = (self.cycle - 1) / 40 ;
        let y = (self.cycle - 1) % 40 ;
        self.screen[x as usize][y as usize] = true;
    }

    pub fn draw(self: &Self) {
        for row in &self.screen {
            for pixel in row {
                if *pixel {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

#[derive(PartialEq, Debug)]
enum Command {
    Noop,
    AddX(i64),
}

fn parse_commands(input: &str) -> Vec<Command> {
    let mut result: Vec<Command> = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let command = parts.next().unwrap();
        match command {
            "noop" => result.push(Command::Noop),
            "addx" => {
                let value = parts.next().unwrap().parse::<i64>().unwrap();
                result.push(Command::AddX(value));
            },
            _ => panic!("Unknown command: {}", command),
        }
    }
    result
}

fn load_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_commands() {
        let input = "noop\naddx 1\nnoop";
        let commands = parse_commands(input);
        assert_eq!(commands.len(), 3);
        assert_eq!(commands[0], Command::Noop);
        assert_eq!(commands[1], Command::AddX(1));
        assert_eq!(commands[2], Command::Noop);
    }
}
