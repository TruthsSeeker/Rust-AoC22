use std::collections::HashMap;

#[derive(Clone)]
pub struct Directory<'a> {
    pub idx: usize,
    pub size: usize,
    pub name: &'a str,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub files: HashMap<String, usize>
}



impl Directory<'_> {
    pub fn new<'a>(idx: usize, name: &'a str, parent: Option<usize>) -> Directory<'a> {
        Directory {
            idx,
            size: 0,
            name,
            parent,
            children: vec![],
            files: HashMap::new(),
        }
    }    
    
}

