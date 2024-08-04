#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug)]
pub struct Raft {
    pub state: State,
    pub current_term: u64,
    pub voted_for: Option<u64>,
    pub log: Vec<LogEntry>,
    pub commit_index: usize,
    pub last_applied: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub term: u64,
    pub command: String,
}

impl Raft {
    pub fn new() -> Self {
        Raft {
            state: State::Follower,
            current_term: 0,
            voted_for: None,
            log: vec![],
            commit_index: 0,
            last_applied: 0,
        }
    }

    pub fn request_vote(&mut self, term: u64, candidate_id: u64) -> bool {
        if term > self.current_term {
            self.current_term = term;
            self.voted_for = Some(candidate_id);
            return true;
        }
        false
    }

    pub fn append_entries(&mut self, term: u64, entries: Vec<LogEntry>) -> bool {
        if term >= self.current_term {
            self.current_term = term;
            self.log.extend(entries);
            return true;
        }
        false
    }

    pub fn become_candidate(&mut self) {
        self.state = State::Candidate;
        self.current_term += 1;
        self.voted_for = Some(self.current_term); // Simplified candidate ID
    }

    pub fn become_leader(&mut self) {
        self.state = State::Leader;
    }

    pub fn become_follower(&mut self, term: u64) {
        self.state = State::Follower;
        self.current_term = term;
        self.voted_for = None;
    }
}

