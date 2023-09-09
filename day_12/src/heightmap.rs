#[derive(Debug)]
pub(crate) struct HeightMap {
    pub(crate) arena: Vec<Node>,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) row_length: usize,
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

    pub(crate) fn find_node_neighbors(&mut self, node: usize) {
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

    pub(crate) fn evaluate_neighbor(&mut self, node: usize, candidate: usize, height: i32) {
        let candidate_height = self.arena[candidate].height as i32;
        if (candidate_height - height).abs() <= 1 {
            self.arena[node].add_neighbor(candidate);
        }
    }
}

#[derive(Debug)]
pub(crate) struct Node {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) height: char,
    pub(crate) neighbors: Vec<usize>,
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
