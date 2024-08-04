use merkle_tree::MerkleTree;
fn main() {
    let data = vec!["hello", "world", "foo", "bar"];
    let merkle_tree = MerkleTree::new(&data);
    
    match merkle_tree.root() {
        Some(root) => println!("Merkle Tree Root: {}", root),
        None => println!("Merkle Tree is empty"),
    }
}
