use std::{io::{self, Read}, fs::File};

fn main() {
    let contents = match read_file("data/input.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("Error reading file: {}", e),
    };
    let groups = contents.split("\n").collect::<Vec<&str>>();
    let mut count = 0;
    // for group in &groups {
    //     let pairs = parse_pairs(group);
    //     if check_containment(pairs.0, pairs.1) {
    //         count += 1;
    //     }
    // }
    // println!("Count contained: {}", count);
    // count = 0;

    for group in groups {
        let pairs = parse_pairs(group);
        if check_overlap(pairs.0, pairs.1) {
            count += 1;
        }
    }
    println!("Count overlapped: {}", count);
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_pairs(contents: &str) -> ((i32, i32), (i32, i32)) {
    let split = contents.split(',').map(|x| {
        x.split('-').map(|y| {
            y.parse::<i32>().unwrap()
        }).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();
    
    ((split[0][0], split[0][1]), (split[1][0], split[1][1]))
}

fn check_containment(pair1: (i32, i32), pair2: (i32, i32)) -> bool {
    if (pair1.0 >= pair2.0 && pair1.1 <= pair2.1) || (pair1.0 <= pair2.0 && pair1.1 >= pair2.1) {
        true
    } else {
        false
    }
}

fn check_overlap(pair1: (i32, i32), pair2: (i32, i32)) -> bool {
    println!("{}-{},{}-{}", pair1.0, pair1.1, pair2.0, pair2.1);
    if (pair1.0 >= pair2.0 && pair1.0 <= pair2.1) || (pair1.1 >= pair2.0 && pair1.1 <= pair2.1) 
    || check_containment(pair1, pair2){
        println!("true");
        true
    } else {
        println!("false");
        false
    }
}
