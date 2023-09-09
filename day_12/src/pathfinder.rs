use std::collections::HashSet;

use std::collections::HashMap;

use crate::heightmap::HeightMap;

pub(crate) struct Pathfinder {
    pub(crate) map: HeightMap,
    pub(crate) came_from: HashMap<usize, usize>,
    pub(crate) start: usize
}

impl Pathfinder {
    pub(crate) fn reconstruct_path(&self, current: usize) -> Vec<usize> {
        let mut total_path = vec![current];
        let mut current = current;
        while self.came_from.contains_key(&current) {
            current = self.came_from[&current];
            total_path.push(current);
        }
        total_path.reverse();
        total_path
    }

    pub(crate) fn heuristic(&self, idx: usize) -> i32 {
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
