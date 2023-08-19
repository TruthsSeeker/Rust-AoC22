use std::{collections::{VecDeque, HashMap}, cell::{Cell, RefCell}, rc::Weak, rc::Rc};

#[derive(Clone)]
pub struct Directory {
    pub value: usize,
    pub name: String,
    pub parent: Weak<RefCell<Directory>>,
    pub children: HashMap<String, Rc<RefCell<Directory>>>,
    pub files: HashMap<String, usize>
}



impl Directory {
    fn dfs(self: &Self, value: usize) -> Option<&Directory> {
        if self.value == value {
            return Some(self)
        }
        let mut result = None;
        for (_, child) in &self.children {
            result = child.borrow().dfs(value);
        }
        result
    }

    fn bfs(self: &Self, value: usize) -> Option<&Directory> {
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(node) = queue.pop_front() {
            if node.value == value {
                return Some(node)
            }

            for (_, child) in &node.children {
                queue.push_back(&child.borrow())
            }
        }
        None
    }

    fn depth_first_traversal(self: &Self, lambda: fn(&usize) ) {
        lambda(&self.value);

        for (_, child) in &self.children {
            child.borrow().depth_first_traversal(lambda);
        }
    }

    fn breadth_first_traversal(self: &Self, lambda: fn(&usize)) {
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(node) = queue.pop_front() {
            lambda(&node.value);

            for (_, child) in &node.children {
        
                queue.push_back(&child.clone().borrow());
            }
        }
    }

    fn sum_children(self: &Self) -> usize {
        let mut sum = 0;
        for (_, child) in &self.children {
            sum += child.borrow().value;
        }
        sum
    }

    pub fn propagate_value(self: &mut Self, value: usize) {
        self.value += value;
        if let Some(parent) = self.parent.upgrade() {
            parent.borrow_mut().propagate_value(value);
        }
    }
    pub fn get_root(self: &Self) -> Directory {
        let mut root = self.clone();
        while let Some(parent) = root.parent.upgrade() {
            root = parent.borrow().clone();
        }
        root
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_propagate_value() {
    }
}