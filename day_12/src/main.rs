use std::{fs::File, error::Error, io::Read};

fn main() {
    println!("Hello, world!");
    let data = match load_data("data/input.txt") {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };
    let map = parse_height_map(&data);
    let height_map = HeightMap::new(map);
    println!("Data: {}", data); 
    println!("Start: {} {:?}", height_map.start, height_map.arena[height_map.start]);
    println!("End: {} {:?}", height_map.end, height_map.arena[height_map.end]);
    println!("Node count: {}", height_map.arena.len());
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

#[derive(Debug)]
struct HeightMap {
    arena: Vec<Node>,
    start: usize,
    end: usize,
    row_length: usize,
}

impl HeightMap {
    pub fn new(map: Vec<Vec<char>>) -> HeightMap {
        let mut arena: Vec<Node> = Vec::new();
        let mut start: usize = 0;
        let mut end: usize = 0;
        let row_length = map[0].len();
        for (y, row) in map.iter().enumerate() {
            for (x, height) in row.iter().enumerate() {
                let mut converted_height = *height;
                if *height == 'S' {
                    converted_height = 'a';
                    start = y * row_length + x;
                } else if *height == 'E' {
                    converted_height = 'z';
                    end = y * row_length + x;
                }
                let node = Node::new(x, y, converted_height);
                arena.push(node);
            }
        }
        let mut height_map = HeightMap {
            arena,
            start,
            end,
            row_length,
        };
        height_map.find_neighbors();
        height_map
    }

    pub fn find_neighbors(&mut self) {
        for i in 0..self.arena.len() {
            self.find_node_neighbors(i);
        }
    }

    fn find_node_neighbors(&mut self, node: usize) {
        let height = self.arena[node].height as i32;
        // Check north
        if node as i32 - self.row_length as i32 > 0 {
            let candidate = node - self.row_length;
            self.evaluate_neighbor(node, candidate, height);
        }
        // Check south
        if node + self.row_length < self.arena.len() {
            let candidate = node + self.row_length;
            self.evaluate_neighbor(node, candidate, height);
        }
        // Check west
        if (node % self.row_length) as i32 - 1 >= 0 {
            let candidate = node - 1;
            self.evaluate_neighbor(node, candidate, height);
        }
        // Check east
        if node + 1 < self.arena.len() && (node % self.row_length) as i32 + 1 < self.row_length as i32 {
            let candidate = node + 1;
            self.evaluate_neighbor(node, candidate, height);
        }
    }

    fn evaluate_neighbor(&mut self, node: usize, candidate: usize, height: i32) {
        let candidate_height = self.arena[candidate].height as i32;
        if (candidate_height - height).abs() <= 1 {
            self.arena[node].add_neighbor(candidate);
        }
    }
}

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    height: char,
    neighbors: Vec<usize>,
}

impl Node {
    pub fn new(x: usize, y: usize, height: char) -> Node {
        Node {
            x,
            y,
            height,
            neighbors: Vec::new(),
        }
    }

    pub fn add_neighbor(&mut self, neighbor: usize) {
        self.neighbors.push(neighbor);
    }
}