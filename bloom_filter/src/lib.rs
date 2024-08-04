use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct BloomFilter {
    bit_vec: Vec<bool>,
    num_hashes: usize,
}

impl BloomFilter {
    pub fn new(size: usize, num_hashes: usize) -> BloomFilter {
        BloomFilter {
            bit_vec: vec![false; size],
            num_hashes,
        }
    }

    fn hash<T: Hash>(&self, item: &T, seed: u64) -> usize {
        let mut hasher = DefaultHasher::new();
        hasher.write_u64(seed);
        item.hash(&mut hasher);
        (hasher.finish() as usize) % self.bit_vec.len()
    }

    pub fn add<T: Hash>(&mut self, item: T) {
        for i in 0..self.num_hashes {
            let pos = self.hash(&item, i as u64);
            self.bit_vec[pos] = true;
        }
    }

    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        for i in 0..self.num_hashes {
            let pos = self.hash(item, i as u64);
            if !self.bit_vec[pos] {
                return false;
            }
        }
        true
    }
}