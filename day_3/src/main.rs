use std::{io::{self, Read}, fs::File, collections::HashSet};

#[derive(Debug)]
struct Rucksack {
    first_compartment: Vec<char>,
    second_compartment: Vec<char>,
}

fn main() {
    let path = "data/input.txt";
    let input = match read_file(path) {
        Ok(input) => input,
        Err(e) => panic!("Error reading {}: {}", path, e),
    };

    let rucksacks = separate_rucksacks(&input);
    let mut common_items: Vec<char> = Vec::new();

    // Find common items in each rucksack
    for rucksack in rucksacks {
        let mut items = find_common_items(&rucksack);
        common_items.append(&mut items);
    }

    // Calculate total priority of common items
    let priority_total = common_items.iter().fold(0, |acc, item| acc + get_item_priority(*item));
    println!("Total priority: {}", priority_total);

    // Part 2
    let groups = separate_groups(&input);
    let mut group_badges: Vec<char> = Vec::new();
    for group in groups {
        let badge = match find_group_badge(&group) {
            Some(badge) => badge,
            None => panic!("No badge found for group {:?}", group),
        };
        group_badges.push(badge);
    }

    let group_badges_priorities = group_badges.iter().fold(0, |acc, badge| {
            acc + get_item_priority(*badge)
        }
    );
    println!("Total group badges priority: {}", group_badges_priorities);
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_rucksack(input: &str) -> Rucksack {
    let compartment_size = input.len() / 2 ;
    let items: Vec<char> = input.chars().collect();
    let first_compartment = items[..compartment_size].to_vec();
    let second_compartment = items[compartment_size..].to_vec();
    Rucksack {
        first_compartment: first_compartment,
        second_compartment: second_compartment,
    }
}

fn separate_rucksacks(input: &str) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();
    let split = input.split("\n");
    for line in split {
        let rucksack = parse_rucksack(line);
        rucksacks.push(rucksack);
    }
    rucksacks
}

fn find_common_items(rucksack: &Rucksack) -> Vec<char> {
    let mut common_items: Vec<char> = Vec::new();
    for item in &rucksack.first_compartment {
        if rucksack.second_compartment.contains(item) && !common_items.contains(item){
            common_items.push(*item);
        }
    }
    common_items
}

fn get_item_priority(item: char) -> i32 {
    // a-z: 1-26
    // A-Z: 27-52
    let mut priority = item as i32;
    if priority < 97 {
        priority -= 38
    } else {
        priority -= 96;
    }
    priority
}

fn separate_groups(input: &str) -> Vec<Vec<String>> {
    let mut groups: Vec<Vec<String>> = Vec::new();
    let split: Vec<&str> = input.split("\n").collect();
    let mut group: Vec<String> = Vec::new();
    for i in 0..split.len() {
        group.push(split[i].to_string());
        if i % 3 == 2 {
            groups.push(group);
            group = Vec::new();
        } 
    }
    groups
}

fn find_group_badge(group:  &Vec<String>) -> Option<char> {
    if group.len() <= 1 {
        return None;
    }
    let initial_set: HashSet<char> = HashSet::from_iter(group[0].chars());
    let mut common_set: HashSet<char> = initial_set.intersection(&group[1].chars().collect()).cloned().collect();
    for i in 2..group.len() {
        common_set = common_set.intersection(&group[i].chars().collect()).cloned().collect();
    }
    common_set.drain().last()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_item_priority() {
        assert_eq!(get_item_priority('a'), 1);
        assert_eq!(get_item_priority('z'), 26);
        assert_eq!(get_item_priority('A'), 27);
        assert_eq!(get_item_priority('Z'), 52);
    }

    #[test]
    fn test_parse_rucksack() {
        let input = "abcdefghij";
        let rucksack = parse_rucksack(input);
        assert_eq!(rucksack.first_compartment, vec!['a', 'b', 'c', 'd', 'e']);
        assert_eq!(rucksack.second_compartment, vec!['f', 'g', 'h', 'i', 'j']);
    }

    #[test]
    fn test_find_common_items() {
        let input = "abcdezfghijz";
        let rucksack = parse_rucksack(input);
        let common_items = find_common_items(&rucksack);
        assert_eq!(common_items, vec!['z']);
    }

    #[test]
    fn test_separate_groups() {
        let input = "abc\ndef\nghi\njkl\nmno\npqr\nstu\nvwx\nyz";
        let groups = separate_groups(input);
        assert_eq!(groups[0], vec!["abc", "def", "ghi"]);
        assert_eq!(groups[1], vec!["jkl", "mno", "pqr"]);
        assert_eq!(groups[2], vec!["stu", "vwx", "yz"]);
    }

    #[test]
    fn test_find_group_badge()  {
        let group = vec!["asdf".to_string(), "aghj".to_string(), "aklm".to_string()];
        assert_eq!(find_group_badge(&group), Some('a'));
        let malformed_group = vec!["asdf".to_string()];
        assert_eq!(find_group_badge(&malformed_group), None);
        let no_common_group = vec!["asdf".to_string(), "ghjk".to_string(), "lmno".to_string()];
        assert_eq!(find_group_badge(&no_common_group), None);
        let group_of_2 = vec!["asdf".to_string(), "aghj".to_string()];
        assert_eq!(find_group_badge(&group_of_2), Some('a'));
    }
}