#[derive(Debug, Clone, PartialEq)]
pub enum PaxosState {
    None,
    Propose,
    Accept,
    Decide,
}

#[derive(Debug, Clone)]
pub struct Paxos {
    pub state: PaxosState,
    pub proposal_number: u64,
    pub proposal_value: Option<String>,
    pub accepted_proposal_number: u64,
    pub accepted_value: Option<String>,
}

impl Paxos {
    pub fn new() -> Self {
        Paxos {
            state: PaxosState::None,
            proposal_number: 0,
            proposal_value: None,
            accepted_proposal_number: 0,
            accepted_value: None,
        }
    }

    pub fn prepare(&mut self, proposal_number: u64) -> bool {
        if proposal_number > self.proposal_number {
            self.proposal_number = proposal_number;
            self.state = PaxosState::Propose;
            true
        } else {
            false
        }
    }

    pub fn propose(&mut self, proposal_number: u64, value: String) -> bool {
        if proposal_number == self.proposal_number {
            self.proposal_value = Some(value.clone());
            self.state = PaxosState::Accept;
            true
        } else {
            false
        }
    }

    pub fn accept(&mut self, proposal_number: u64) -> Option<String> {
        if proposal_number >= self.proposal_number {
            self.accepted_proposal_number = proposal_number;
            self.accepted_value = self.proposal_value.clone();
            self.state = PaxosState::Decide;
            self.accepted_value.clone()
        } else {
            None
        }
    }

    pub fn decide(&self) -> Option<String> {
        if self.state == PaxosState::Decide {
            self.accepted_value.clone()
        } else {
            None
        }
    }
}