use paxos_consensus::Paxos;

fn main() {
    let mut node1 = Paxos::new();
    let mut node2 = Paxos::new();
    let mut node3 = Paxos::new();

    // Node 1 prepares a proposal
    node1.prepare(1);
    node1.propose(1, "value1".to_string());

    // Nodes 2 and 3 accept the proposal
    node2.prepare(1);
    node2.propose(1, "value1".to_string());
    let value2 = node2.accept(1);

    node3.prepare(1);
    node3.propose(1, "value1".to_string());
    let value3 = node3.accept(1);

    // Decide value
    let decided_value = node1.decide().or(value2).or(value3);
    println!("Decided value: {:?}", decided_value);
}
