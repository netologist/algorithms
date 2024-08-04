use raft_consensus::{Raft, State, LogEntry};

#[test]
fn test_request_vote() {
    let mut raft = Raft::new();
    assert!(raft.request_vote(1, 1));
    assert_eq!(raft.current_term, 1);
    assert_eq!(raft.voted_for, Some(1));
}

#[test]
fn test_append_entries() {
    let mut raft = Raft::new();
    let entries = vec![LogEntry { term: 1, command: "cmd1".to_string() }];
    assert!(raft.append_entries(1, entries.clone()));
    assert_eq!(raft.log, entries);
}

#[test]
fn test_state_transitions() {
    let mut raft = Raft::new();
    raft.become_candidate();
    assert_eq!(raft.state, State::Candidate);

    raft.become_leader();
    assert_eq!(raft.state, State::Leader);

    raft.become_follower(2);
    assert_eq!(raft.state, State::Follower);
    assert_eq!(raft.current_term, 2);
}
