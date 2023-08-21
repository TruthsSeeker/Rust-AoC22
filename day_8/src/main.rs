use std::{fs::File, io::Read};

fn main() {
    let input = load_file("data/input.txt").unwrap();
    let forest = parse_forest(&input);
    let mut visible_trees = 0;
    for x in 0..forest.len() {
        for y in 0..forest[x].len() {
            let tree = (x as u32, y as u32);
            if is_visible_from_north(forest.clone(), tree) ||
                is_visible_from_south(forest.clone(), tree) ||
                is_visible_from_east(forest.clone(), tree) ||
                is_visible_from_west(forest.clone(), tree) {
                    visible_trees += 1;
                }
        }
    }
    println!("Visible trees: {}", visible_trees);

    let mut max_los = 0;
    for x in 0..forest.len() {
        for y in 0..forest[x].len() {
            let tree = (x as u32, y as u32);
            let los = calculate_los(forest.clone(), tree);
            if los > max_los {
                max_los = los;
            }
        }
    }
    println!("Max LOS: {}", max_los);
}

fn load_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_forest(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|column| {
            column
            .chars()
            .map(|c| c.to_digit(10)
                .unwrap_or(0))
            .collect()
        })
        .collect()
}

fn is_visible_from_north(forest: Vec<Vec<u32>>, tree: (u32, u32)) -> bool {
    let (x, y) = tree;
    let height = forest[x as usize][y as usize];
    for i in 0..x {
        if forest[i as usize][y as usize] >= height {
            return false;
        }
    }
    true
}

fn is_visible_from_south(forest: Vec<Vec<u32>>, tree: (u32, u32)) -> bool {
    let (x, y) = tree;
    let height = forest[x as usize][y as usize];
    for i in x+1..forest[x as usize].len() as u32 {
        if forest[i as usize][y as usize] >= height {
            return false;
        }
    }
    true
}

fn is_visible_from_east(forest: Vec<Vec<u32>>, tree: (u32, u32)) -> bool {
    let (x, y) = tree;
    let height = forest[x as usize][y as usize];
    for i in y+1..forest.len() as u32 {
        if forest[x as usize][i as usize] >= height {
            return false;
        }
    }
    true
}

fn is_visible_from_west(forest: Vec<Vec<u32>>, tree: (u32, u32)) -> bool {
    let (x, y) = tree;
    let height = forest[x as usize][y as usize];
    for i in 0..y {
        if forest[x as usize][i as usize] >= height {
            return false;
        }
    }
    true
}

fn los_north(forest: Vec<Vec<u32>>, tree: (u32, u32)) -> u32 {
    let mut los = 1;
    let (x, y) = tree;
    let height = forest[x as usize][y as usize];
    let mut max = 0;
    for i in (0..x).rev() {
        let current = forest[i as usize][y as usize];
        if current >= height && current <= max {
            return los;
        }
        los += 1;
        if current > max {
            max = current;
        }
    }
    los
}

fn los_south(forest: Vec<Vec<u32>>, tree: (u32, u32)) -> u32 {
    let mut los = 1;
    let (x, y) = tree;
    let height = forest[x as usize][y as usize];
    let mut max = 0;
    for i in x+1..forest[x as usize].len() as u32 {
        let current = forest[i as usize][y as usize];
        if current >= height && current <= max {
            return los;
        }
        los += 1;
        if current > max {
            max = current;
        }
    }
    los
}

fn los_east(forest: Vec<Vec<u32>>, tree: (u32, u32)) -> u32 {
    let mut los = 1;
    let (x, y) = tree;
    let height = forest[x as usize][y as usize];
    let mut max = 0;
    for i in y+1..forest.len() as u32 {
        let current = forest[x as usize][i as usize];
        if current >= height {
            return los;
        }
        los += 1;
        if current > max {
            max = current;
        }
    }
    los
}

fn los_west(forest: Vec<Vec<u32>>, tree: (u32, u32)) -> u32 {
    let mut los = 0;
    let (x, y) = tree;
    let height = forest[x as usize][y as usize];
    let mut max = 0;
    for i in (0..y).rev() {
        let current = forest[x as usize][i as usize];
        if current >= height {
            return los;
        }
        los += 1;
        if current > max {
            max = current;
        }
    }
    los
}

fn calculate_los(forest: Vec<Vec<u32>>, tree: (u32, u32)) -> u32 {
    let (x, y) = tree;
    if x == 0 || y == 0 || x + 1 == forest.len() as u32 || y + 1 == forest[x as usize].len() as u32 {
        return 0;
    }
    los_north(forest.clone(), tree) *
    los_south(forest.clone(), tree) *
    los_east(forest.clone(), tree) *
    los_west(forest.clone(), tree)
}