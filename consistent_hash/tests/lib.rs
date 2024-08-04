use consistent_hash::ConsistentHash;

#[test]
fn test_new_consistent_hash() {
    let ch = ConsistentHash::new(100);
    assert_eq!(ch.replicas, 100);
    assert!(ch.ring.is_empty());
    assert!(ch.nodes.is_empty());
}

#[test]
fn test_add_node() {
    let mut ch = ConsistentHash::new(10);
    ch.add("node1");
    assert_eq!(ch.nodes.len(), 1);
    assert_eq!(ch.ring.len(), 10);
    assert!(ch.nodes.contains(&"node1".to_string()));
}

#[test]
fn test_remove_node() {
    let mut ch = ConsistentHash::new(10);
    ch.add("node1");
    ch.add("node2");
    ch.remove("node1");
    assert_eq!(ch.nodes.len(), 1);
    assert_eq!(ch.ring.len(), 10);
    assert!(!ch.nodes.contains(&"node1".to_string()));
    assert!(ch.nodes.contains(&"node2".to_string()));
}

#[test]
fn test_get_node() {
    let mut ch = ConsistentHash::new(10);
    ch.add("node1");
    ch.add("node2");
    ch.add("node3");

    let node = ch.get("key1");
    assert!(node.is_some());
    
    // Test consistency
    let node1 = ch.get("key1");
    let node2 = ch.get("key1");
    assert_eq!(node1, node2);
}

#[test]
fn test_get_node_empty_ring() {
    let ch = ConsistentHash::new(10);
    let node = ch.get("key1");
    assert!(node.is_none());
}

#[test]
fn test_distribution() {
    let mut ch = ConsistentHash::new(100);
    ch.add("node1");
    ch.add("node2");
    ch.add("node3");

    let mut distribution = std::collections::HashMap::new();
    for i in 0..1000 {
        let key = format!("key{}", i);
        let node = ch.get(&key).unwrap();
        *distribution.entry(node.to_string()).or_insert(0) += 1;
    }

    // Check if all nodes are used
    assert_eq!(distribution.len(), 3);

    // Check if distribution is roughly even (within 20% of average)
    let avg = 1000 / 3;
    for count in distribution.values() {
        assert!(*count as f64 >= avg as f64 * 0.8);
        assert!(*count as f64 <= avg as f64 * 1.2);
    }
}

#[test]
fn test_consistency_after_node_removal() {
    let mut ch = ConsistentHash::new(100);
    ch.add("node1");
    ch.add("node2");
    ch.add("node3");

    let mut initial_mapping = std::collections::HashMap::new();
    for i in 0..1000 {
        let key = format!("key{}", i);
        let node = ch.get(&key).unwrap().to_string();
        initial_mapping.insert(key, node);
    }

    ch.remove("node2");

    let mut changed_keys = 0;
    for (key, initial_node) in initial_mapping.iter() {
        let new_node = ch.get(key).unwrap();
        if new_node != initial_node {
            changed_keys += 1;
        }
    }

    // Check if less than 50% of keys changed nodes
    assert!(changed_keys < 500);
}
