
use bloom_filter::BloomFilter;

fn main() {
    let mut bf = BloomFilter::new(1000, 3);
    
    bf.add("apple");
    bf.add("banana");
    bf.add("cherry");
    
    println!("{}", bf.contains(&"apple"));   // true
    println!("{}", bf.contains(&"banana"));  // true
    println!("{}", bf.contains(&"durian"));  // false (probably)
}