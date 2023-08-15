use std::{collections::{VecDeque, HashMap}, cell::{Cell, RefCell}, rc::Weak};

pub struct Directory {
    pub value: usize,
    pub name: String,
    pub parent: Weak<RefCell<Directory>>,
    pub children: Vec<Directory>,
    pub files: HashMap<String, usize>
}



impl Directory {
    fn dfs(self: &Self, value: usize) -> Option<&Directory> {
        if self.value == value {
            return Some(self)
        }
        
        for child in &self.children {
            return child.dfs(value)
        }
        None
    }

    fn bfs(self: &Self, value: usize) -> Option<&Directory> {
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

    fn depth_first_traversal(self: &Self, lambda: fn(&usize) ) {
        lambda(&self.value);

        for child in &self.children {
            child.depth_first_traversal(lambda);
        }
    }

    fn breadth_first_traversal(self: &Self, lambda: fn(&usize)) {
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(node) = queue.pop_front() {
            lambda(&node.value);

            for child in &node.children {
                queue.push_back(&child);
            }
        }
    }

    fn sum_children(self: &Self) -> usize {
        let mut sum = 0;
        for child in &self.children {
            sum += child.value;
        }
        sum
    }

    pub fn propagate_value(self: &mut Self, value: usize) {
        self.value += value;
        if let Some(parent) = self.parent.upgrade() {
            parent.borrow_mut().propagate_value(value);
        }
    }
}
