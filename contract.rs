
#[ink::contract]
pub mod treasury_governance {

    use ink::storage::Mapping;
    use ink::prelude::{vec::Vec, string::String, vec};
    use ink::H160;
    use crate::enums::*;
    use crate::storage::*;
    use crate::errors::*;
    use crate::events::*;

    #[ink(storage)]
    pub struct TreasuryGovernance {
        /// Next proposal ID
        pub next_proposal_id: u32,
        /// All proposals
        pub proposals: Mapping<u32, Proposal>,
        /// User votes on proposals (proposal_id -> voter -> Vote)
        pub votes: Mapping<(u32, H160), Vote>,
        /// List of all proposal IDs
        pub proposal_ids: Vec<u32>,
        /// Total number of registered voters (for quorum calculation)
        pub total_voters: u32,
        /// Contract owner
        pub owner: H160,
        pub registered_voters: Mapping<H160, bool>,
    }
    
    impl Default for TreasuryGovernance {
        fn default() -> Self {
            Self::new()
        }
    }

    impl TreasuryGovernance {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                next_proposal_id: 1,
                proposals: Mapping::default(),
                votes: Mapping::default(),
                proposal_ids: Vec::new(),
                total_voters: 0,
                owner: Self::env().caller(),
                registered_voters: Mapping::default(),
            }
        }
    }

    impl TreasuryGovernance {
        
        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u32, option_index: u32) -> Result<(), Error> {
            let caller = self.env().caller();
            let current_block = self.env().block_number();
       
            // Get proposal
            let mut proposal = self.proposals.get(proposal_id)
                .ok_or(Error::ProposalNotFound)?;
       
            // Validate proposal is active
            if proposal.status != ProposalStatus::Active {
                return Err(Error::ProposalNotActive);
            }
       
            // Check voting period hasn't ended
            if current_block >= proposal.voting_end {
                return Err(Error::VotingPeriodEnded);
            }
       
            // Check user hasn't already voted
            if self.votes.get((proposal_id, caller)).is_some() {
                return Err(Error::AlreadyVoted);
            }
       
            // Validate option index
            if option_index >= proposal.voting_options.options.len() as u32 {
                return Err(Error::InvalidProposal);
            }
       
            // Get option text
            let option_text = proposal.voting_options.options[option_index as usize].clone();
       
            // Create vote record with weight of 1
            let vote = Vote {
                voter: caller,
                choice: VoteChoice {
                    option_index,
                    option_text: option_text.clone(),
                },
                timestamp: current_block,
                weight: 1,
            };
       
            // Update vote counts
            proposal.vote_counts[option_index as usize] += 1;
            proposal.total_voters += 1;
       
            // Store vote and updated proposal
            self.votes.insert((proposal_id, caller), &vote);
            self.proposals.insert(proposal_id, &proposal);
       
            self.env().emit_event(VoteCast {
                proposal_id,
                voter: caller,
                option_index,
                option_text,
                weight: 1,
            });
            Ok(())
        }
        
        #[ink(message)]
        pub fn register_voter(&mut self) {
            let caller = self.env().caller();
                    
            if self.registered_voters.get(&caller).is_none() {
                self.registered_voters.insert(caller, &true);
                self.total_voters += 1;
        
                self.env().emit_event(VoterRegistered {
                    voter: caller,
                    total_voters: self.total_voters,
                });
            }
        }
        
        #[ink(message)]
        pub fn create_proposal(
            &mut self,
            title: String,
            description: String,
            proposal_type: ProposalType,
            governance_params: GovernanceParameters,
            voting_options: VotingOptions,
        ) -> Result<u32, Error> {
            // Validate voting options
            if voting_options.options.is_empty() || voting_options.options.len() > 10 {
                return Err(Error::InvalidProposal);
            }

            let proposal_id = self.next_proposal_id;
            let caller = self.env().caller();
            let current_block = self.env().block_number();

            // Calculate voting end time
            let voting_blocks = governance_params.voting_period.to_blocks();
            let voting_end = current_block.saturating_add(voting_blocks);

            // Calculate execution time
            let execution_delay = governance_params.execution_delay.to_blocks();
            let execution_time = voting_end.saturating_add(execution_delay);

            // Initialize vote counts
            let vote_counts = vec![0u128; voting_options.options.len()];

            let proposal = Proposal {
                id: proposal_id,
                title: title.clone(),
                description,
                proposal_type,
                governance_params,
                voting_options,
                proposer: caller,
                created_at: current_block,
                voting_end,
                execution_time,
                status: ProposalStatus::Active,
                vote_counts,
                total_voters: 0,
            };

            self.proposals.insert(proposal_id, &proposal);
            self.proposal_ids.push(proposal_id);
            self.next_proposal_id += 1;

            self.env().emit_event(ProposalCreated {
                proposal_id,
                proposer: caller,
                title,
            });

            Ok(proposal_id)
        }

        #[ink(message)]
        pub fn update_proposal_status(&mut self, proposal_id: u32) -> Result<(), Error> {
            let current_block = self.env().block_number();

            let mut proposal = self.proposals.get(proposal_id)
                .ok_or(Error::ProposalNotFound)?;

            // Only update if still active and voting period ended
            if proposal.status != ProposalStatus::Active {
                return Ok(());
            }

            if current_block < proposal.voting_end {
                return Ok(());
            }

            // Calculate quorum requirement
            let quorum_percentage = proposal.governance_params.quorum_threshold.to_percentage();
            let required_votes = (self.total_voters as u128 * quorum_percentage as u128) / 100;

            let total_votes: u128 = proposal.vote_counts.iter().sum();

            // Check if quorum reached
            if total_votes < required_votes {
                proposal.status = ProposalStatus::Rejected;
                self.proposals.insert(proposal_id, &proposal);
                return Ok(());
            }

            // Find winning option (highest vote count)
            let max_votes = proposal.vote_counts.iter().max().copied().unwrap_or(0);
            let winners: Vec<usize> = proposal.vote_counts
                .iter()
                .enumerate()
                .filter(|(_, count)| **count == max_votes)
                .map(|(idx, _)| idx)
                .collect();

                // Handle ties
                if winners.len() > 1 {
                    proposal.status = ProposalStatus::Rejected;
                } else {
                    proposal.status = ProposalStatus::Passed;
                }
            self.proposals.insert(proposal_id, &proposal);
            Ok(())
        }
        
        #[ink(message)]
        pub fn execute_proposal(&mut self, proposal_id: u32) -> Result<(), Error> {
            let current_block = self.env().block_number();
       
            let mut proposal = self.proposals.get(proposal_id)
                    .ok_or(Error::ProposalNotFound)?;
       
            // Validate proposal is in Passed status
            if proposal.status != ProposalStatus::Passed {
                return Err(Error::ProposalNotReadyForExecution);
            }
       
            // Check execution delay has passed
            if current_block < proposal.execution_time {
                return Err(Error::ProposalNotReadyForExecution);
            }
       
            // Update status to Executed
            proposal.status = ProposalStatus::Executed;
            self.proposals.insert(proposal_id, &proposal);
       
            self.env().emit_event(ProposalExecuted {
                proposal_id,
                status: ProposalStatus::Executed,
            });
       
            Ok(())
        }
            
        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u32) -> Option<Proposal> {
            self.proposals.get(proposal_id)
        }
        
        #[ink(message)]
        pub fn get_all_proposal_ids(&self) -> Vec<u32> {
            self.proposal_ids.clone()
        }
        
        #[ink(message)]
        pub fn get_user_vote(&self, proposal_id: u32, user: H160) -> Option<Vote> {
            self.votes.get((proposal_id, user))
        }
        
        #[ink(message)]
        pub fn get_stats(&self) -> (u32, u32, u32) {
            let total = self.proposal_ids.len() as u32;
            let mut active = 0u32;
            let mut executed = 0u32;
        
            for id in &self.proposal_ids {
                if let Some(proposal) = self.proposals.get(*id) {
                    match proposal.status {
                        ProposalStatus::Active => active += 1,
                        ProposalStatus::Executed => executed += 1,
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
        pub fn has_reached_quorum(&self, proposal_id: u32) -> bool {
            if let Some(proposal) = self.proposals.get(proposal_id) {
                let quorum_percentage = proposal.governance_params.quorum_threshold.to_percentage();
                let required_votes = (self.total_voters as u128 * quorum_percentage as u128) / 100;
                let total_votes: u128 = proposal.vote_counts.iter().sum();
                
                total_votes >= required_votes
            } else {
                false
            }
        }
        
        #[ink(message)]
        pub fn get_proposal_results(&self, proposal_id: u32) -> Option<(Vec<u128>, bool)> {
            if let Some(proposal) = self.proposals.get(proposal_id) {
                let reached_quorum = self.has_reached_quorum(proposal_id);
                Some((proposal.vote_counts, reached_quorum))
            } else {
                None
            }
        }
        
        #[ink(message)]
        pub fn get_voting_options(&self, proposal_id: u32) -> Option<Vec<String>> {
            self.proposals.get(proposal_id)
                .map(|p| p.voting_options.options)
        }
        
        #[ink(message)]
        pub fn get_detailed_results(&self, proposal_id: u32) -> Option<Vec<(String, u128)>> {
            if let Some(proposal) = self.proposals.get(proposal_id) {
                let results: Vec<(String, u128)> = proposal.voting_options.options
                           .iter()
                           .zip(proposal.vote_counts.iter())
                           .map(|(option, &count)| (option.clone(), count))
                           .collect();
                       Some(results)
                } else {
                None
            }
        }
        
        #[ink(message)]
        pub fn get_winning_option(&self, proposal_id: u32) -> Option<(String, u128)> {
            if let Some(proposal) = self.proposals.get(proposal_id) {
                let max_votes = proposal.vote_counts.iter().max().copied()?;
                let winner_idx = proposal.vote_counts.iter().position(|&v| v == max_votes)?;
                let option_name = proposal.voting_options.options.get(winner_idx)?.clone();
                Some((option_name, max_votes))
            } else {
                None
            }
        }
    }
    
}
