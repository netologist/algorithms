use consistent_hash::ConsistentHash;

fn main() {
    let mut ch = ConsistentHash::new(100);
    ch.add("node1");
    ch.add("node2");
    ch.add("node3");

    for i in 0..10 {
        let key = format!("key{}", i);
        println!("Key {} -> Node {:?}", key, ch.get(&key));
    }

    println!("\nRemoving node2");
    ch.remove("node2");

    for i in 0..10 {
        let key = format!("key{}", i);
        println!("Key {} -> Node {:?}", key, ch.get(&key));
    }
}