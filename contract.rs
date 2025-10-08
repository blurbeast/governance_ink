

#[ink::contract]
mod treasury_governance {
    use ink::prelude::{string::String, vec::Vec};
    use ink::storage::Mapping;
    use ink::H160;

    // --- Custom Types (as requested by the workshop) ---

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum ProposalType {
        Treasury,
        Governance,
        Technical,
        Other,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum VotingPeriod {
        ThreeDays,
        SevenDays,
        FourteenDays,
        ThirtyDays,
    }

    impl VotingPeriod {
        /// Convert voting period to number of blocks, assuming 6s block time.
        pub fn to_blocks(&self) -> u32 {
            // blocks per day at 6s block time
            const BLOCKS_PER_DAY: u128 = 86400u128 / 6u128; // 14400
            let days = match self {
                VotingPeriod::ThreeDays => 3u128,
                VotingPeriod::SevenDays => 7u128,
                VotingPeriod::FourteenDays => 14u128,
                VotingPeriod::ThirtyDays => 30u128,
            };
            // safe cast to u32 (workshop assures reasonable sizes)
            let blocks = days.saturating_mul(BLOCKS_PER_DAY) as u32;
            blocks
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum QuorumThreshold {
        Five,
        Ten,
        Twenty,
        TwentyFive,
    }

    impl QuorumThreshold {
        pub fn to_percentage(&self) -> u32 {
            match self {
                QuorumThreshold::Five => 5,
                QuorumThreshold::Ten => 10,
                QuorumThreshold::Twenty => 20,
                QuorumThreshold::TwentyFive => 25,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub enum ExecutionDelay {
        Immediately,
        OneDay,
        TwoDays,
        SevenDays,
    }

    impl ExecutionDelay {
        pub fn to_blocks(&self) -> u32 {
            const BLOCKS_PER_DAY: u128 = 86400u128 / 6u128; // 14400
            match self {
                ExecutionDelay::Immediately => 0u32,
                ExecutionDelay::OneDay => BLOCKS_PER_DAY as u32,
                ExecutionDelay::TwoDays => (BLOCKS_PER_DAY * 2u128) as u32,
                ExecutionDelay::SevenDays => (BLOCKS_PER_DAY * 7u128) as u32,
            }
        }
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
    pub enum ProposalStatus {
        Active,
        Passed,
        Rejected,
        Executed,
        Expired,
    }

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
        pub proposer: AccountId,
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
        pub voter: AccountId,
        pub choice: VoteChoice,
        pub timestamp: u32,
        pub weight: u128,
    }

    // --- Events ---
    #[ink(event)]
    pub struct ProposalCreated {
        #[ink(topic)]
        proposal_id: u32,
        #[ink(topic)]
        proposer: AccountId,
        title: String,
    }

    #[ink(event)]
    pub struct VoteCast {
        #[ink(topic)]
        proposal_id: u32,
        #[ink(topic)]
        voter: AccountId,
        option_index: u32,
        option_text: String,
        weight: u128,
    }

    #[ink(event)]
    pub struct ProposalExecuted {
        #[ink(topic)]
        proposal_id: u32,
        status: ProposalStatus,
    }

    // --- Errors ---
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        ProposalNotFound,
        ProposalNotActive,
        VotingPeriodEnded,
        AlreadyVoted,
        NotAuthorized,
        ProposalNotReadyForExecution,
        InvalidProposal,
        InvalidOptionIndex,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    // --- Storage ---
    #[ink(storage)]
    pub struct TreasuryGovernance {
        next_proposal_id: u32,
        proposals: Mapping<u32, Proposal>,
        votes: Mapping<(u32, AccountId), Vote>,
        proposal_ids: Vec<u32>,
        total_voters: u32,
        owner: AccountId,
    }

    impl TreasuryGovernance {
        /// Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                next_proposal_id: 1,
                proposals: Mapping::default(),
                votes: Mapping::default(),
                proposal_ids: Vec::new(),
                total_voters: 0,
                owner: caller,
            }
        }

        // --- Voter registration ---
        #[ink(message)]
        pub fn register_voter(&mut self) -> Result<()> {
            // simple registration: every distinct caller increments total_voters
            let caller = self.env().caller();
            // check if already registered by searching votes map for a sentinel key (not ideal but keeps workshop simple)
            // For production, keep a dedicated Mapping<AccountId, bool> of registered voters.
            // We'll add a simple dedicated mapping-like behavior by scanning proposal_ids is too expensive; instead keep a separate mapping.
            // For clarity in this workshop, add a Mapping<AccountId, bool> in storage would be better — but to avoid changing storage layout now, we'll maintain a naive approach:
            // We'll assume repeated calls are allowed but only increment once per caller: use votes mapping with key (0, caller) as registration sentinel.
            let reg_key = (0u32, caller);
            if self.votes.get(&reg_key).is_some() {
                return Err(Error::InvalidProposal); // already registered (reuse error enum for brevity)
            }
            let dummy_vote = Vote {
                voter: caller,
                choice: VoteChoice { option_index: 0u32, option_text: String::from("registered") },
                timestamp: Self::env().block_number(),
                weight: 0u128,
            };
            self.votes.insert(reg_key, &dummy_vote);
            self.total_voters = self.total_voters.saturating_add(1);
            Ok(())
        }

        // --- Create proposal ---
        #[ink(message)]
        pub fn create_proposal(
            &mut self,
            title: String,
            description: String,
            proposal_type: ProposalType,
            governance_params: GovernanceParameters,
            voting_options: VotingOptions,
        ) -> Result<u32> {
            // validate voting options length 1..=10
            let opts_len = voting_options.options.len();
            if opts_len == 0 || opts_len > 10 {
                return Err(Error::InvalidProposal);
            }

            let id = self.next_proposal_id;
            let created_at = Self::env().block_number();
            // compute voting end and execution_time using block numbers
            let voting_blocks = governance_params.voting_period.to_blocks();
            let voting_end = created_at.saturating_add(voting_blocks);
            let exec_delay_blocks = governance_params.execution_delay.to_blocks();
            let execution_time = voting_end.saturating_add(exec_delay_blocks);

            // initialize vote_counts with zeros
            let mut vote_counts: Vec<u128> = Vec::with_capacity(opts_len);
            for _ in 0..opts_len {
                vote_counts.push(0u128);
            }

            let proposal = Proposal {
                id,
                title: title.clone(),
                description,
                proposal_type,
                governance_params,
                voting_options: voting_options.clone(),
                proposer: Self::env().caller(),
                created_at,
                voting_end,
                execution_time,
                status: ProposalStatus::Active,
                vote_counts,
                total_voters: 0,
            };

            self.proposals.insert(id, &proposal);
            self.proposal_ids.push(id);
            self.next_proposal_id = self.next_proposal_id.saturating_add(1);

            self.env().emit_event(ProposalCreated { proposal_id: id, proposer: proposal.proposer, title });
            Ok(id)
        }

        // --- Vote ---
        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u32, option_index: u32) -> Result<()> {
            let caller = Self::env().caller();
            let mut proposal = self.proposals.get(&proposal_id).ok_or(Error::ProposalNotFound)?;

            if proposal.status != ProposalStatus::Active {
                return Err(Error::ProposalNotActive);
            }

            let now = Self::env().block_number();
            if now > proposal.voting_end {
                return Err(Error::VotingPeriodEnded);
            }

            // check double voting
            let vote_key = (proposal_id, caller);
            if self.votes.get(&vote_key).is_some() {
                return Err(Error::AlreadyVoted);
            }

            // validate option index
            if option_index as usize >= proposal.voting_options.options.len() {
                return Err(Error::InvalidOptionIndex);
            }

            // simple weight = 1 for all voters (bonus challenge can modify)
            let weight: u128 = 1u128;
            // update vote counts
            if let Some(slot) = proposal.vote_counts.get_mut(option_index as usize) {
                *slot = slot.saturating_add(weight);
            }

            proposal.total_voters = proposal.total_voters.saturating_add(1);

            // persist updated proposal
            self.proposals.insert(proposal_id, &proposal);

            // store vote record
            let choice = VoteChoice { option_index, option_text: proposal.voting_options.options[option_index as usize].clone() };
            let vote = Vote { voter: caller, choice: choice.clone(), timestamp: now, weight };
            self.votes.insert(vote_key, &vote);

            self.env().emit_event(VoteCast { proposal_id, voter: caller, option_index, option_text: choice.option_text, weight });
            Ok(())
        }

        // --- Update proposal status (to be called by anyone) ---
        #[ink(message)]
        pub fn update_proposal_status(&mut self, proposal_id: u32) -> Result<ProposalStatus> {
            let mut proposal = self.proposals.get(&proposal_id).ok_or(Error::ProposalNotFound)?;
            let now = Self::env().block_number();

            if proposal.status != ProposalStatus::Active {
                return Ok(proposal.status);
            }

            if now <= proposal.voting_end {
                return Err(Error::VotingPeriodEnded);
            }

            // compute total votes
            let total_votes: u128 = proposal.vote_counts.iter().copied().sum();
            // quorum requirement
            let quorum_pct = proposal.governance_params.quorum_threshold.to_percentage();
            let required_votes = ((self.total_voters as u128) * (quorum_pct as u128) + 99u128) / 100u128; // ceil

            if total_votes < required_votes {
                proposal.status = ProposalStatus::Rejected;
                self.proposals.insert(proposal_id, &proposal);
                return Ok(ProposalStatus::Rejected);
            }

            // find max vote and check tie
            let mut max_votes: u128 = 0u128;
            let mut max_index: Option<usize> = None;
            for (i, &count) in proposal.vote_counts.iter().enumerate() {
                if count > max_votes {
                    max_votes = count;
                    max_index = Some(i);
                } else if count == max_votes {
                    // potential tie — we'll handle after loop
                }
            }

            // check tie: more than one option have votes == max_votes
            let ties = proposal.vote_counts.iter().filter(|&&c| c == max_votes).count();
            if ties > 1 {
                proposal.status = ProposalStatus::Rejected;
                self.proposals.insert(proposal_id, &proposal);
                return Ok(ProposalStatus::Rejected);
            }

            // clear winner
            proposal.status = ProposalStatus::Passed;
            self.proposals.insert(proposal_id, &proposal);
            Ok(ProposalStatus::Passed)
        }

        // --- Execute proposal ---
        #[ink(message)]
        pub fn execute_proposal(&mut self, proposal_id: u32) -> Result<()> {
            let mut proposal = self.proposals.get(&proposal_id).ok_or(Error::ProposalNotFound)?;
            if proposal.status != ProposalStatus::Passed {
                return Err(Error::ProposalNotReadyForExecution);
            }
            let now = Self::env().block_number();
            if now < proposal.execution_time {
                return Err(Error::ProposalNotReadyForExecution);
            }

            // For workshop: we just mark executed. Real treasury actions would be performed here.
            proposal.status = ProposalStatus::Executed;
            self.proposals.insert(proposal_id, &proposal);
            self.env().emit_event(ProposalExecuted { proposal_id, status: proposal.status.clone() });
            Ok(())
        }

        // --- Query functions ---
        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u32) -> Result<Proposal> {
            Ok(self.proposals.get(&proposal_id).ok_or(Error::ProposalNotFound)?)
        }

        #[ink(message)]
        pub fn get_all_proposal_ids(&self) -> Vec<u32> {
            self.proposal_ids.clone()
        }

        #[ink(message)]
        pub fn get_user_vote(&self, proposal_id: u32, user: AccountId) -> Option<Vote> {
            self.votes.get(&(proposal_id, user))
        }

        #[ink(message)]
        pub fn get_stats(&self) -> (u32, u32, u32) {
            // (total, active, executed)
            let total = self.proposal_ids.len() as u32;
            let mut active = 0u32;
            let mut executed = 0u32;
            for &id in self.proposal_ids.iter() {
                if let Some(p) = self.proposals.get(&id) {
                    match p.status {
                        ProposalStatus::Active => active = active.saturating_add(1),
                        ProposalStatus::Executed => executed = executed.saturating_add(1),
                        _ => {}
                    }
                }
            }
            (total, active, executed)
        }

        #[ink(message)]
        pub fn get_total_voters(&self) -> u32 {
            self.total_voters
        }

        #[ink(message)]
        pub fn has_reached_quorum(&self, proposal_id: u32) -> Result<bool> {
            let proposal = self.proposals.get(&proposal_id).ok_or(Error::ProposalNotFound)?;
            let total_votes: u128 = proposal.vote_counts.iter().copied().sum();
            let quorum_pct = proposal.governance_params.quorum_threshold.to_percentage();
            let required_votes = ((self.total_voters as u128) * (quorum_pct as u128) + 99u128) / 100u128;
            Ok(total_votes >= required_votes)
        }

        #[ink(message)]
        pub fn get_proposal_results(&self, proposal_id: u32) -> Result<(Vec<u128>, bool)> {
            let proposal = self.proposals.get(&proposal_id).ok_or(Error::ProposalNotFound)?;
            let reached = self.has_reached_quorum(proposal_id)?;
            Ok((proposal.vote_counts.clone(), reached))
        }

        #[ink(message)]
        pub fn get_voting_options(&self, proposal_id: u32) -> Result<Vec<String>> {
            let proposal = self.proposals.get(&proposal_id).ok_or(Error::ProposalNotFound)?;
            Ok(proposal.voting_options.options.clone())
        }

        #[ink(message)]
        pub fn get_detailed_results(&self, proposal_id: u32) -> Result<Vec<(String, u128)>> {
            let proposal = self.proposals.get(&proposal_id).ok_or(Error::ProposalNotFound)?;
            let mut out: Vec<(String, u128)> = Vec::new();
            for (i, name) in proposal.voting_options.options.iter().enumerate() {
                let count = proposal.vote_counts.get(i).copied().unwrap_or(0u128);
                out.push((name.clone(), count));
            }
            Ok(out)
        }

        #[ink(message)]
        pub fn get_winning_option(&self, proposal_id: u32) -> Result<(Option<String>, u128)> {
            let proposal = self.proposals.get(&proposal_id).ok_or(Error::ProposalNotFound)?;
            let mut max_votes: u128 = 0u128;
            let mut winner: Option<String> = None;
            let mut ties = 0usize;
            for (i, name) in proposal.voting_options.options.iter().enumerate() {
                let count = proposal.vote_counts.get(i).copied().unwrap_or(0u128);
                if count > max_votes {
                    max_votes = count;
                    winner = Some(name.clone());
                    ties = 1;
                } else if count == max_votes && count != 0u128 {
                    ties += 1;
                }
            }
            if ties > 1 {
                Ok((None, max_votes))
            } else {
                Ok((winner, max_votes))
            }
        }
    }
}
