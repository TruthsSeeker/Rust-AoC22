use std::{fs::File, error::Error, io::Read, collections::{HashMap}, path};

mod heightmap;
use pathfinder::Pathfinder;

use crate::heightmap::HeightMap;

mod pathfinder;

fn main() {
    println!("Hello, world!");
    let data = match load_data("data/input.txt") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };
    let map = parse_height_map(&data);
    let height_map = HeightMap::new(map);

    part1(height_map.clone());

    part2(height_map);
}

fn part1(height_map: HeightMap) {
    let start = height_map.start.clone();
    let mut pathfinder = pathfinder::Pathfinder {
        map: height_map,
        came_from: HashMap::new(),
        start: start
    };
    let path = pathfinder.a_star().unwrap_or(vec![0]);
    println!("Path length: {}", path.len() - 1);
}

fn part2(height_map: HeightMap) {
    let potential_starts = height_map.arena.clone().iter()
        .enumerate()
        .filter(|(_ , node)| node.height == 'a')
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();
    let mut paths: Vec<Vec<usize>> = Vec::new();
    for start in potential_starts {
        println!("Evaluating start {}", &start);
        let mut pathfinder = Pathfinder {
            map: height_map.clone(),
            came_from: HashMap::new(),
            start
        };
        match pathfinder.a_star() {
            Some(path) => paths.push(path),
            None => continue,
        }
    }
    paths.sort_by_key(|path| path.len());
    println!("Shortest path {}", paths.pop().unwrap().len());
}

fn load_data(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_height_map(data: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in data.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }
    map
}



#[cfg(test)]
mod test {
    use super::*;

}
