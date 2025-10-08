use ink::H160;
use ink::prelude::string::String;
use crate::enums::ProposalStatus;

#[ink::event]
pub struct ProposalCreated {
    #[ink(topic)]
    pub proposal_id: u32,
    pub proposer: H160,
    pub title: String,
}

#[ink::event]
pub struct ProposalExecuted {
    #[ink(topic)]
    pub proposal_id: u32,
    pub status: ProposalStatus,
}

#[ink::event]
pub struct VoterRegistered {
    #[ink(topic)]
    pub voter: H160,
    pub total_voters: u32,
}

#[ink::event]
pub struct VoteCast {
    #[ink(topic)]
    pub voter: H160,
    pub proposal_id: u32,
    pub option_index: u32,
    pub weight: u32,
    pub option_text: String
}