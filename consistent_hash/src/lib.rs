use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct ConsistentHash {
    pub ring: BTreeMap<u64, String>,
    pub nodes: Vec<String>,
    pub replicas: usize,
}

impl ConsistentHash {
    pub fn new(replicas: usize) -> Self {
        ConsistentHash {
            ring: BTreeMap::new(),
            nodes: Vec::new(),
            replicas,
        }
    }

    pub fn add(&mut self, node: &str) {
        for i in 0..self.replicas {
            let key = self.hash(&format!("{}:{}", node, i));
            self.ring.insert(key, node.to_string());
        }
        self.nodes.push(node.to_string());
    }

    pub fn remove(&mut self, node: &str) {
        for i in 0..self.replicas {
            let key = self.hash(&format!("{}:{}", node, i));
            self.ring.remove(&key);
        }
        self.nodes.retain(|n| n != node);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        if self.ring.is_empty() {
            return None;
        }
        let hash = self.hash(key);
        match self.ring.range(hash..).next() {
            Some((_, node)) => Some(node),
            None => self.ring.values().next(),
        }
    }

    pub fn hash(&self, key: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }
}