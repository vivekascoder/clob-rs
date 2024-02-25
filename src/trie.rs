use std::collections::HashMap;

pub struct TrieNode {
    children: HashMap<String, String>,
    is_end: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode {
                children: HashMap::new(),
                is_end: false,
            },
        }
    }

    pub fn insert(&self) {
        // let current = self.root;
    }

    pub fn search() {}
}
