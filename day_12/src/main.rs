use std::{fs::File, error::Error, io::Read, collections::{HashMap, HashSet, BinaryHeap}, cmp::Reverse};

mod heightmap;
use heightmap::Node;

use crate::heightmap::HeightMap;

fn main() {
    println!("Hello, world!");
    let data = match load_data("data/input.txt") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };
    let map = parse_height_map(&data);
    let height_map = HeightMap::new(map);
    println!("Data:\n{}", data); 
    println!("Start: {} {:?}", height_map.start, height_map.arena[height_map.start]);
    println!("End: {} {:?}", height_map.end, height_map.arena[height_map.end]);
    println!("Node count: {}", height_map.arena.len());

    let mut pathfinder = Pathfinder {
        map: height_map,
        came_from: HashMap::new()
    };
    let path = pathfinder.a_star().unwrap_or(vec![0]);
    println!("Path length: {}", path.len() - 1);
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

struct Pathfinder {
    map: HeightMap,
    came_from: HashMap<usize, usize>
}

impl Pathfinder {
    fn reconstruct_path(&self, current: usize) -> Vec<usize> {
        let mut total_path = vec![current];
        let mut current = current;
        while self.came_from.contains_key(&current) {
            current = self.came_from[&current];
            total_path.push(current);
        }
        total_path.reverse();
        total_path
    }
    
    fn heuristic(&self, idx: usize) -> i32 {
        let current = &self.map.arena[idx];
        let end = &self.map.arena[self.map.end];
        let x = (current.x as i32 - end.x as i32).pow(2);
        let y = (current.y as i32 - end.y as i32).pow(2);
        ((x + y) as f32).sqrt().round() as i32
    }

    pub fn a_star(&mut self) -> Option<Vec<usize>>{
        let mut open_set = HashSet::from([self.map.start]);

        // for node n g_score[n] is the cheapest path from start to n.
        let mut g_score = HashMap::from([(self.map.start, 0)]);

        // for node n f_score[n] = g_score[n] + heuristic(n)
        // It represents the current best guess at how short a path from n to the end could be
        let mut f_score = HashMap::from([(self.map.start, self.heuristic(self.map.start))]);

        while !open_set.is_empty() {
            let lowest_fscore = match f_score.iter().filter(|(k, _)| open_set.contains(k)).min_by_key(| a| a.1) {
                Some((key, _)) => *key,
                None => panic!("f_score is empty"),
            };

            let current = open_set.take(&lowest_fscore).unwrap();
            if current == self.map.end {
                return Some(self.reconstruct_path(current))
            }

            let node = &self.map.arena[current];
            for neighbor in &node.neighbors {
                let tentative_g_score = match g_score.get(&current) {
                    Some(score) => score + 1,
                    None => i32::MAX,
                };
                let neighbor_g_score = match g_score.get(neighbor) {
                    Some(score) => *score,
                    None => i32::MAX,
                };
                if tentative_g_score < neighbor_g_score {
                    self.came_from.insert(*neighbor, current);
                    g_score.insert(*neighbor, tentative_g_score);
                    f_score.insert(*neighbor, tentative_g_score + self.heuristic(*neighbor));

                    open_set.insert(*neighbor);
                }
            }
        }
        None
    }
}


#[cfg(test)]
mod test {
    use super::*;

}
