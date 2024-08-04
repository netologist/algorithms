
use merkle_tree::MerkleTree;
use sha2::{Sha256, Digest};
use hex::encode;


#[test]
fn test_empty_tree() {
    let tree = MerkleTree::new(&[]);
    assert!(tree.root().is_none());
}

#[test]
fn test_single_element_tree() {
    let tree = MerkleTree::new(&["hello"]);
    let mut hasher = Sha256::new();
    hasher.update(b"hello");
    let hash = encode(hasher.finalize());
    assert_eq!(tree.root(), Some(&hash));
}

#[test]
fn test_multiple_elements_tree() {
    let tree = MerkleTree::new(&["hello", "world"]);
    let mut hasher = Sha256::new();
    hasher.update(b"hello");
    let hash1 = encode(hasher.finalize());

    let mut hasher = Sha256::new();
    hasher.update(b"world");
    let hash2 = encode(hasher.finalize());

    let mut hasher = Sha256::new();
    hasher.update(hash1.as_bytes());
    hasher.update(hash2.as_bytes());
    let root_hash = encode(hasher.finalize());

    assert_eq!(tree.root(), Some(&root_hash));
}

#[test]
fn test_odd_number_of_elements() {
    let tree = MerkleTree::new(&["hello", "world", "foo"]);
    let mut hasher = Sha256::new();
    hasher.update(b"hello");
    let hash1 = encode(hasher.finalize());

    let mut hasher = Sha256::new();
    hasher.update(b"world");
    let hash2 = encode(hasher.finalize());

    let mut hasher = Sha256::new();
    hasher.update(b"foo");
    let hash3 = encode(hasher.finalize());

    let mut hasher = Sha256::new();
    hasher.update(hash3.as_bytes());
    hasher.update(hash3.as_bytes());
    let hash4 = encode(hasher.finalize());

    let mut hasher = Sha256::new();
    hasher.update(hash1.as_bytes());
    hasher.update(hash2.as_bytes());
    let hash5 = encode(hasher.finalize());

    let mut hasher = Sha256::new();
    hasher.update(hash5.as_bytes());
    hasher.update(hash4.as_bytes());
    let root_hash = encode(hasher.finalize());

    assert_eq!(tree.root(), Some(&root_hash));
}