use crate::enums::*;
use ink::H160;
use ink::prelude::vec::Vec;
use ink::prelude::string::String;
use crate::enums::ProposalStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub governance_params: GovernanceParameters,
    pub voting_options: VotingOptions,
    pub proposer: H160,
    pub created_at: u32,
    pub voting_end: u32,
    pub execution_time: u32,
    pub status: ProposalStatus,
    pub vote_counts: Vec<u128>,
    pub total_voters: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct Vote {
    pub voter: H160,
    pub choice: VoteChoice,
    pub timestamp: u32,
    pub weight: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub struct VoteChoice {
    pub option_index: u32,
    pub option_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct GovernanceParameters {
        pub voting_period: VotingPeriod,
        pub quorum_threshold: QuorumThreshold,
        pub execution_delay: ExecutionDelay,
    }
    
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct VotingOptions {
        pub options: Vec<String>,
    }