use std::collections::HashMap;

pub struct Node {
    pub state: char,
    pub path: HashMap<char, usize>
}

pub type NodeList = Vec<Node>;

impl Node {
    pub fn new(s: char) -> Node {
        return Node{state: s, path: HashMap::new()};
    }

    pub fn get_path(&self, s: char) -> Option<usize> {
        return self.path.get(&s).and_then(|i| Some(*i));
    }
    pub fn set_path(&mut self, s: char, dest: usize) {
        if self.path.insert(s, dest).is_some() {
            println!("PathError: the path already exists. ");
        }
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        return format!("{},{},{}", 
            self.state, 
            self.get_path('g').map(|i| i.to_string())
                .unwrap_or("?".to_owned()), 
            self.get_path('w').map(|i| i.to_string())
                .unwrap_or("?".to_owned()));
    }
}
