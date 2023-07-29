use std::{path::Path, fs::File, io::{self, Read}};

fn main() {
    println!("Hello, world!");
    let path_str = "data/inventory.txt";
    let file_contents = match read_file_to_str(path_str){
        Ok(s) => s,
        Err(e) => panic!("Error reading file: {}", e),
    };
    let split = file_contents.split("\n");
    let mut inventories_calories = vec![0];
    let mut top3 = vec![0, 0, 0];
    for line in split {
        let index = inventories_calories.len() - 1;
        if line == "" {
            let inventory = inventories_calories[index];
            top3 = rank_in_top3(&inventory, &top3);
            inventories_calories.push(0);
            continue;
        }
        let calories = parse_calories(line);
        inventories_calories[index] += calories;
    }
    println!("Max calories: {}", inventories_calories.iter().max().unwrap_or(&0));
    println!("Top 3: {:?}", top3);
    let top3_sum: i32 = top3.iter().sum();
    println!("Top 3 sum: {}", top3_sum);
}

fn read_file_to_str(path_str: &str) -> Result<String, io::Error> {
    let path = Path::new(path_str);
    println!("Path to file: {}", path.display());
    let mut file = File::open(path_str)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?; 
    Ok(s)
}

fn parse_calories(line: &str) -> i32 {
    let calories = match line.parse::<i32>() {
        Ok(n) => n,
        Err(e) => panic!("Error parsing calories: {}", e),
    };
    calories
}

fn rank_in_top3(inventory: &i32, top3: &Vec<i32>) -> Vec<i32> {
    if top3.len() != 3 {
        panic!("top3 must have 3 elements")
    }   
    let mut new_top3 = top3.clone();
    if inventory > &new_top3[0] {
        new_top3[2] = new_top3[1];
        new_top3[1] = new_top3[0];
        new_top3[0] = *inventory;
    } else if inventory > &new_top3[1] {
        new_top3[2] = new_top3[1];
        new_top3[1] = *inventory;
    } else if inventory > &new_top3[2] {
        new_top3[2] = *inventory;
    }
    new_top3
}