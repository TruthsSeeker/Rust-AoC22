use std::collections::HashSet;

use utils::load_file;
mod utils;

fn main() {
    let file = load_file("data/input.txt").unwrap();
    let commands = file.split("\n").collect::<Vec<&str>>();
    part1(commands.clone());
    part2(commands.clone());
}

fn part1(commands: Vec<&str>) {
    let mut tail_positions = HashSet::new();
    tail_positions.insert((0, 0));
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut rope = vec![head, tail];
    for command in commands {
        let parsed_command = parse_command(command);
        process_command(parsed_command, &mut rope, &mut tail_positions);
    }
    println!("Part 1 tail positions: {}", tail_positions.len());
}

fn part2(commands: Vec<&str>) {
    let mut tail_positions = HashSet::new();
    tail_positions.insert((0, 0));
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    for command in commands {
        let parsed_command = parse_command(command);
        process_command(parsed_command, &mut rope, &mut tail_positions);
    }
    println!("Part 2 tail positiions: {}", tail_positions.len());
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction")
        }
    }
}

fn parse_command(command: &str) -> (Direction, i32) {
    let mut chars = command.chars();
    let direction = Direction::from_char(chars.next().unwrap_or(' '));
    chars.next(); // skip space
    let distance = chars.as_str().parse::<i32>().unwrap();
    (direction, distance)
}

fn process_command(command: (Direction, i32), rope: &mut Vec<(i32, i32)>, tail_positions: &mut HashSet<(i32, i32)>) {
    let (direction, times) = command;
    for _ in 0..times {
        rope[0] = move_head(&direction, &rope[0]);
        for knot in 0..rope.len() - 1 {
            let head = rope[knot];
            let mut tail = rope[knot + 1];
            let distance = calculate_distance(&head, &tail);
            if is_tail_move_required(distance) {
                tail = move_tail(&head, &tail);
                rope[knot + 1] = tail;
                if knot == rope.len() - 2 {
                    tail_positions.insert(tail);
                }
            }
        }
    }
}

// a distance of of > 2.0_f32.sqrt() requires the tail to move because 
// that is the distance between two points one diagonal move away from each other
fn calculate_distance(head: &(i32, i32), tail: &(i32, i32)) -> f32 {
    let (x_head, y_head) = head;
    let (x_tail, y_tail) = tail;
    (((x_head - x_tail) as f32).powf(2.0) + ((y_head - y_tail) as f32).powf(2.0)).sqrt()
}

fn is_tail_move_required(distance: f32) -> bool {
    distance > 2.0_f32.sqrt()
}

fn move_tail(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    let (x_head, y_head) = head;
    let (x_tail, y_tail) = tail;
    let x_diff = x_head - x_tail;
    let y_diff = y_head - y_tail;
    
    let mut x_tail_new = *x_tail;
    if x_diff > 0 {
        x_tail_new += 1;
    } else if x_diff < 0 {
        x_tail_new -= 1;
    }

    let mut y_tail_new = *y_tail;
    if y_diff > 0 {
        y_tail_new += 1;
    } else if y_diff < 0 {
        y_tail_new -= 1;
    }
    (x_tail_new, y_tail_new)
}

fn move_head(direction: &Direction, head: &(i32, i32)) -> (i32, i32) {
    let (x_head, y_head) = head;
    match direction {
        Direction::Up => (*x_head, *y_head + 1),
        Direction::Down => (*x_head, *y_head - 1),
        Direction::Left => (*x_head - 1, *y_head),
        Direction::Right => (*x_head + 1, *y_head)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        let command = "U 2";
        let (direction, distance) = parse_command(command);
        assert_eq!(direction, Direction::Up);
        assert_eq!(distance, 2);
    }

    #[test]
    fn test_calculate_distance() {
        let head = (0, 0);
        let tail = (1, 1);
        let distance = calculate_distance(&head, &tail);
        assert_eq!(distance, 2.0_f32.sqrt());
    }

    #[test]
    fn test_is_tail_move_required() {
        let distance = 2.0_f32.sqrt();
        assert_eq!(is_tail_move_required(distance), false);
        let head = (0, 0);
        let tail = (2, 1);
        let distance = calculate_distance(&head, &tail);
        assert_eq!(is_tail_move_required(distance), true);
    }

    #[test]
    fn test_move_tail() {
        let head = (0, 0);
        let tail = (2, 1);
        let tail_new = move_tail(&head, &tail);
        assert_eq!(tail_new, (1, 0));
        
        let head = (0, 0);
        let tail = (0, 2);
        let tail_new = move_tail(&head, &tail);
        assert_eq!(tail_new, (0, 1));

        let head = (0, 0);
        let tail = (-2, 0);
        let tail_new = move_tail(&head, &tail);
        assert_eq!(tail_new, (-1, 0));
    }
}