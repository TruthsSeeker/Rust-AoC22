use std::{collections::VecDeque, cell::RefCell, rc::{Weak, Rc}, borrow::BorrowMut};

pub struct Directory {
    pub value: i32,
    pub name: String,
    pub parent: Weak<RefCell<Directory>>,
    pub children: Vec<Directory>,
}



impl Directory {
    fn dfs(self: &Self, value: i32) -> Option<&Directory> {
        if self.value == value {
            return Some(self)
        }
        
        for child in &self.children {
            return child.dfs(value)
        }
        None
    }

    fn bfs(self: &Self, value: i32) -> Option<&Directory> {
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(node) = queue.pop_front() {
            if node.value == value {
                return Some(node)
            }

            for child in &node.children {
                queue.push_back(&child)
            }
        }
        None
    }

    fn depth_first_traversal(self: &Self, lambda: fn(&i32) ) {
        lambda(&self.value);

        for child in &self.children {
            child.depth_first_traversal(lambda);
        }
    }

    fn breadth_first_traversal(self: &Self, lambda: fn(&i32)) {
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(node) = queue.pop_front() {
            lambda(&node.value);

            for child in &node.children {
                queue.push_back(&child);
            }
        }
    }

    fn sum_children(self: &Self) -> i32 {
        let mut sum = 0;
        for child in &self.children {
            sum += child.value;
        }
        sum
    }
}
