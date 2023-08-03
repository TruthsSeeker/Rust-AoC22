use std::{fs::File, io::Read, collections::VecDeque};

fn main() {
    let contents = read_file("data/input.txt");
    let packet_start = find_packet_start(&mut contents.clone(), 4);
    println!("Packet start: {}", packet_start);

    let message_start = find_packet_start(&mut contents.clone(), 14);
    println!("Message start: {}", message_start);
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    contents
}

fn find_packet_start(input: &mut String, packet_size: usize) -> i32 {
    let mut start = 0;
    let mut buf = VecDeque::<char>::new();
    for (i, c) in input.chars().enumerate() {
        if buf.len() < packet_size {
            buf.push_back(c);
        } else {
            if check_unique(&buf) {
                start = i as i32;
                break;
            } else {
                buf.pop_front();
                buf.push_back(c);
            }
        }
    }
    start
}

fn check_unique(input: &VecDeque<char>) -> bool {
    let set = input.iter().collect::<std::collections::HashSet<_>>();
    set.len() == input.len()
}