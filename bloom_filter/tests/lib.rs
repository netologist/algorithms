use bloom_filter::BloomFilter;

#[test]
fn test_bloom_filter() {
    let mut bloom = BloomFilter::new(100, 3);

    bloom.add("hello");
    bloom.add("world");

    assert!(bloom.contains(&"hello"));
    assert!(bloom.contains(&"world"));
    assert!(!bloom.contains(&"foo"));
    assert!(!bloom.contains(&"bar"));
}

#[test]
fn test_false_positives() {
    let mut bloom = BloomFilter::new(10, 3);

    bloom.add("hello");

    assert!(bloom.contains(&"hello"));

    // Due to the small size and few hash functions, we might get false positives
    // This isn't a precise check, just to illustrate false positives
    assert!(!bloom.contains(&"world")); // This might fail
}