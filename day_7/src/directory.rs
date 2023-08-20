use std::{collections::{VecDeque, HashMap}, cell::{Cell, RefCell}, rc::Weak, rc::Rc};

#[derive(Clone)]
pub struct Directory<'a> {
    pub size: usize,
    pub name: &'a str,
    pub parent: Option<&'a str>,
    pub children: Vec<&'a str>,
    pub files: HashMap<String, usize>
}



impl Directory<'_> {
    pub fn new<'a>(name: &'a str, id: Option<usize>, parent: Option<&'a str>) -> Directory<'a> {
        Directory {
            size: 0,
            name,
            parent,
            children: vec![],
            files: HashMap::new(),
        }
    }    
    
}

#[cfg(test)]
mod test {
    #[test]
    fn test_propagate_value() {
    }
}