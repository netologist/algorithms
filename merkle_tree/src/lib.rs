use sha2::{Sha256, Digest};
use hex::encode;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    root: Option<String>,
    nodes: Vec<String>,
}

impl MerkleTree {
    pub fn new(data: &[&str]) -> Self {
        let leaf_nodes: Vec<String> = data.iter()
            .map(|&datum| {
                let mut hasher = Sha256::new();
                hasher.update(datum.as_bytes());
                encode(hasher.finalize())
            })
            .collect();

        let root = if leaf_nodes.is_empty() {
            None
        } else {
            Some(MerkleTree::build_tree(leaf_nodes.clone()))
        };

        MerkleTree {
            root,
            nodes: leaf_nodes,
        }
    }

    fn build_tree(mut nodes: Vec<String>) -> String {
        while nodes.len() > 1 {
            if nodes.len() % 2 != 0 {
                nodes.push(nodes.last().cloned().unwrap());
            }
            let mut parent_level = Vec::new();
            for chunk in nodes.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(chunk[0].as_bytes());
                hasher.update(chunk[1].as_bytes());
                parent_level.push(encode(hasher.finalize()));
            }
            nodes = parent_level;
        }
        nodes[0].clone()
    }

    pub fn root(&self) -> Option<&String> {
        self.root.as_ref()
    }

    pub fn check_nodes(&self) -> Result<(), String> {
        if self.nodes.is_empty() {
            return Err("Warning: MerkleTree nodes vector is empty.".to_string());
        }
        Ok(())
    }
}