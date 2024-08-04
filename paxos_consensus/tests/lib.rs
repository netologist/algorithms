use paxos_consensus::{Paxos, PaxosState};

#[test]
fn test_prepare() {
    let mut paxos = Paxos::new();
    assert!(paxos.prepare(1));
    assert_eq!(paxos.proposal_number, 1);
    assert_eq!(paxos.state, PaxosState::Propose);
}

#[test]
fn test_prepare_lower_proposal_number() {
    let mut paxos = Paxos::new();
    paxos.prepare(2);
    assert!(!paxos.prepare(1));
}

#[test]
fn test_propose() {
    let mut paxos = Paxos::new();
    paxos.prepare(1);
    assert!(paxos.propose(1, "value1".to_string()));
    assert_eq!(paxos.proposal_value, Some("value1".to_string()));
    assert_eq!(paxos.state, PaxosState::Accept);
}

#[test]
fn test_propose_wrong_proposal_number() {
    let mut paxos = Paxos::new();
    paxos.prepare(1);
    assert!(!paxos.propose(2, "value2".to_string()));
}

#[test]
fn test_accept() {
    let mut paxos = Paxos::new();
    paxos.prepare(1);
    paxos.propose(1, "value1".to_string());
    let accepted_value = paxos.accept(1);
    assert_eq!(accepted_value, Some("value1".to_string()));
    assert_eq!(paxos.state, PaxosState::Decide);
}

#[test]
fn test_accept_lower_proposal_number() {
    let mut paxos = Paxos::new();
    paxos.prepare(2);
    paxos.propose(2, "value2".to_string());
    let accepted_value = paxos.accept(1);
    assert_eq!(accepted_value, None);
}

#[test]
fn test_decide() {
    let mut paxos = Paxos::new();
    paxos.prepare(1);
    paxos.propose(1, "value1".to_string());
    paxos.accept(1);
    let decided_value = paxos.decide();
    assert_eq!(decided_value, Some("value1".to_string()));
}