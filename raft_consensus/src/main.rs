use raft_consensus::Raft;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let raft = Arc::new(Mutex::new(Raft::new()));
    let raft_clone = raft.clone();

    thread::spawn(move || {
        let mut raft = raft_clone.lock().unwrap();
        raft.become_candidate();
        println!("Became candidate: {:?}", raft);

        // Simulate receiving votes and becoming leader
        raft.become_leader();
        println!("Became leader: {:?}", raft);
    });

    thread::sleep(Duration::from_secs(1));

    let raft = raft.lock().unwrap();
    println!("Final state: {:?}", raft);
}