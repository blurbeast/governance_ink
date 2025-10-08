#![cfg_attr(not(feature = "std"), no_std, no_main)]
mod contract;
// #[ink::contract]
// mod treasury_governance {

//     use ink::storage::Mapping;
//     use ink::prelude::vec::Vec;
//     use ink::prelude::string::String;
//     use ink::H160;

//     #[derive(Debug, Clone, PartialEq, Eq)]
//     #[ink::scale_derive(Encode, Decode, TypeInfo)]
//     #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
//     pub enum ProposalType {
//         Treasury,
//         Governance,
//         Technical,
//         Other,
//     }

//     #[derive(Debug, Clone, PartialEq, Eq)]
//     #[ink::scale_derive(Encode, Decode, TypeInfo)]
//     #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
//     pub enum VotingPeriod {
//         ThreeDays,
//         SevenDays,
//         FourteenDays,
//         ThirtyDays,
//     }

//     impl VotingPeriod {
//             /// Convert voting period to blocks (6-second block time)
//             pub fn to_blocks(&self) -> u32 {
//                 match self {
//                     VotingPeriod::ThreeDays => 3 * 24 * 60 * 10,      // 43,200 blocks
//                     VotingPeriod::SevenDays => 7 * 24 * 60 * 10,      // 100,800 blocks
//                     VotingPeriod::FourteenDays => 14 * 24 * 60 * 10,  // 201,600 blocks
//                     VotingPeriod::ThirtyDays => 30 * 24 * 60 * 10,    // 432,000 blocks
//                 }
//             }
//         }

//         impl QuorumThreshold {
//                 /// Convert to percentage value
//                 pub fn to_percentage(&self) -> u32 {
//                     match self {
//                         QuorumThreshold::Five => 5,
//                         QuorumThreshold::Ten => 10,
//                         QuorumThreshold::Twenty => 20,
//                         QuorumThreshold::TwentyFive => 25,
//                     }
//                 }
//             }
//     #[derive(Debug, Clone, PartialEq, Eq)]
//     #[ink::scale_derive(Encode, Decode, TypeInfo)]
//     #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
//     pub enum QuorumThreshold {
//         Five,
//         Ten,
//         Twenty,
//         TwentyFive,
//     }

//     #[derive(Debug, Clone, PartialEq, Eq)]
//     #[ink::scale_derive(Encode, Decode, TypeInfo)]
//     #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
//     pub enum ExecutionDelay {
//         Immediately,
//         OneDay,
//         TwoDays,
//         SevenDays,
//     }

//     #[derive(Debug, Clone, PartialEq, Eq)]
//     #[ink::scale_derive(Encode, Decode, TypeInfo)]
//     #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
//     pub struct GovernanceParameters {
//         pub voting_period: VotingPeriod,
//         pub quorum_threshold: QuorumThreshold,
//         pub execution_delay: ExecutionDelay,
//     }

//     #[derive(Debug, Clone, PartialEq, Eq)]
//     #[ink::scale_derive(Encode, Decode, TypeInfo)]
//     #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
//     pub struct VotingOptions {
//         pub options: Vec<String>,
//     }

//     #[derive(Debug, Clone, PartialEq, Eq)]
//     #[ink::scale_derive(Encode, Decode, TypeInfo)]
//     #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
//     pub enum ProposalStatus {
//         Active,
//         Passed,
//         Rejected,
//         Executed,
//         Expired,
//     }

//     #[derive(Debug, Clone, PartialEq, Eq)]
//     #[ink::scale_derive(Encode, Decode, TypeInfo)]
//     #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
//     pub struct VoteChoice {
//         pub option_index: u32,
//         pub option_text: String,
//     }

//     impl ExecutionDelay {
//         /// Convert to blocks (6-second block time)
//         pub fn to_blocks(&self) -> u32 {
//             match self {
//                 ExecutionDelay::Immediately => 0,
//                 ExecutionDelay::OneDay => 24 * 60 * 10,      // 14,400 blocks
//                 ExecutionDelay::TwoDays => 2 * 24 * 60 * 10, // 28,800 blocks
//                 ExecutionDelay::SevenDays => 7 * 24 * 60 * 10, // 100,800 blocks
//             }
//         }
//     }

//     #[derive(Debug, Clone, PartialEq, Eq)]
//     #[ink::scale_derive(Encode, Decode, TypeInfo)]
//     #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
//     pub struct Proposal {
//         pub id: u32,
//         pub title: String,
//         pub description: String,
//         pub proposal_type: ProposalType,
//         pub governance_params: GovernanceParameters,
//         pub voting_options: VotingOptions,
//         pub proposer: H160,
//         pub created_at: u32,
//         pub voting_end: u32,
//         pub execution_time: u32,
//         pub status: ProposalStatus,
//         pub vote_counts: Vec<u128>,
//         pub total_voters: u32,
//     }
//     #[derive(Debug, Clone, PartialEq, Eq)]
//     #[ink::scale_derive(Encode, Decode, TypeInfo)]
//     #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
//     pub struct Vote {
//         pub voter: H160,
//         pub choice: VoteChoice,
//         pub timestamp: u32,
//         pub weight: u128,
//     }

//     #[ink(storage)]
//     pub struct TreasuryGovernance {
//         /// Next proposal ID
//         next_proposal_id: u32,
//         /// All proposals
//         proposals: Mapping<u32, Proposal>,
//         /// User votes on proposals (proposal_id -> voter -> Vote)
//         votes: Mapping<(u32, H160), Vote>,
//         /// List of all proposal IDs
//         proposal_ids: Vec<u32>,
//         /// Total number of registered voters (for quorum calculation)
//         total_voters: u32,
//         /// Contract owner
//         owner: H160,
//     }
    
//     impl Default for TreasuryGovernance {
//         fn default() -> Self {
//             Self::new()
//         }
//     }

//     impl TreasuryGovernance {
//         #[ink(constructor)]
//         pub fn new() -> Self {
//             Self {
//                 next_proposal_id: 1,
//                 proposals: Mapping::default(),
//                 votes: Mapping::default(),
//                 proposal_ids: Vec::new(),
//                 total_voters: 0,
//                 owner: Self::env().caller(),
//             }
//         }
//     }

//     #[ink::event]
//     pub(crate) struct ProposalCreated {
//       #[ink(topic)]
//       proposal_id: u32,
//       proposer: H160,
//       title: String,
//     }
    
//     #[ink::event]
//     pub(crate) struct ProposalExecuted {
//       #[ink(topic)]
//       proposal_id: u32,
//       status: ProposalStatus,
//     }


//     #[derive(Debug, PartialEq, scale::Encode, scale::Decode)]
//     #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
//     pub enum Error{
//         InvalidProposal,
//         ProposalNotFound,
//         ProposalNotReadyForExecution
//     }

//     impl TreasuryGovernance {
//         #[ink(message)]
//         pub fn create_proposal(
//             &mut self,
//             title: String,
//             description: String,
//             proposal_type: ProposalType,
//             governance_params: GovernanceParameters,
//             voting_options: VotingOptions,
//         ) -> Result<u32, Error> {
//             // Validate voting options
//             if voting_options.options.is_empty() || voting_options.options.len() > 10 {
//                 return Err(Error::InvalidProposal);
//             }

//             let proposal_id = self.next_proposal_id;
//             let caller = self.env().caller();
//             let current_block = self.env().block_number();

//             // Calculate voting end time
//             let voting_blocks = governance_params.voting_period.to_blocks();
//                   let voting_end = current_block.saturating_add(voting_blocks);

//                   // Calculate execution time
//                   let execution_delay = governance_params.execution_delay.to_blocks();
//                   let execution_time = voting_end.saturating_add(execution_delay);

//                   // Initialize vote counts
//                   let vote_counts = Vec::new();

//                   let proposal = Proposal {
//                       id: proposal_id,
//                       title: title.clone(),
//                       description,
//                       proposal_type,
//                       governance_params,
//                       voting_options,
//                       proposer: caller,
//                       created_at: current_block,
//                       voting_end,
//                       execution_time,
//                       status: ProposalStatus::Active,
//                       vote_counts,
//                       total_voters: 0,
//                   };

//                   self.proposals.insert(proposal_id, &proposal);
//                   self.proposal_ids.push(proposal_id);
//                   self.next_proposal_id += 1;

//                   self.env().emit_event(ProposalCreated {
//                       proposal_id,
//                       proposer: caller,
//                       title,
//                   });

//                   Ok(proposal_id)
//               }

//         #[ink(message)]
//         pub fn update_proposal_status(&mut self, proposal_id: u32) -> Result<(), Error> {
//             let current_block = self.env().block_number();

//             let mut proposal = self.proposals.get(proposal_id)
//                 .ok_or(Error::ProposalNotFound)?;

//             // Only update if still active and voting period ended
//             if proposal.status != ProposalStatus::Active {
//                 return Ok(());
//             }

//             if current_block < proposal.voting_end {
//                 return Ok(());
//             }

//             // Calculate quorum requirement
//             let quorum_percentage = proposal.governance_params.quorum_threshold.to_percentage();
//             let required_votes = (self.total_voters as u128 * quorum_percentage as u128) / 100;

//             let total_votes: u128 = proposal.vote_counts.iter().sum();

//             // Check if quorum reached
//             if total_votes < required_votes {
//                 proposal.status = ProposalStatus::Rejected;
//                 self.proposals.insert(proposal_id, &proposal);
//                 return Ok(());
//             }

//             // Find winning option (highest vote count)
//             let max_votes = proposal.vote_counts.iter().max().copied().unwrap_or(0);
//             let winners: Vec<usize> = proposal.vote_counts
//                 .iter()
//                 .enumerate()
//                 .filter(|(_, count)| **count == max_votes)
//                 .map(|(idx, _)| idx)
//                 .collect();

//                 // Handle ties
//                 if winners.len() > 1 {
//                     proposal.status = ProposalStatus::Rejected;
//                 } else {
//                     proposal.status = ProposalStatus::Passed;
//             }

//             self.proposals.insert(proposal_id, &proposal);
//             Ok(())
//         }
        
//         #[ink(message)]
//                pub fn execute_proposal(&mut self, proposal_id: u32) -> Result<(), Error> {
//                    let current_block = self.env().block_number();
       
//                    let mut proposal = self.proposals.get(proposal_id)
//                        .ok_or(Error::ProposalNotFound)?;
       
//                    // Validate proposal is in Passed status
//                    if proposal.status != ProposalStatus::Passed {
//                        return Err(Error::ProposalNotReadyForExecution);
//                    }
       
//                    // Check execution delay has passed
//                    if current_block < proposal.execution_time {
//                        return Err(Error::ProposalNotReadyForExecution);
//                    }
       
//                    // Update status to Executed
//                    proposal.status = ProposalStatus::Executed;
//                    self.proposals.insert(proposal_id, &proposal);
       
//                    self.env().emit_event(ProposalExecuted {
//                        proposal_id,
//                        status: ProposalStatus::Executed,
//                    });
       
//                    Ok(())
//                }
               
//         #[ink(message)]
//         pub fn get_proposal(&self, proposal_id: u32) -> Option<Proposal> {
//             self.proposals.get(proposal_id)
//         }
        
//         #[ink(message)]
//         pub fn get_all_proposal_ids(&self) -> Vec<u32> {
//             self.proposal_ids.clone()
//         }
        
//         #[ink(message)]
//         pub fn get_user_vote(&self, proposal_id: u32, user: H160) -> Option<Vote> {
//             self.votes.get((proposal_id, user))
//         }
        
//         #[ink(message)]
//         pub fn get_stats(&self) -> (u32, u32, u32) {
//             let total = self.proposal_ids.len() as u32;
//             let mut active = 0u32;
//             let mut executed = 0u32;
        
//             for id in &self.proposal_ids {
//                 if let Some(proposal) = self.proposals.get(*id) {
//                     match proposal.status {
//                         ProposalStatus::Active => active += 1,
//                         ProposalStatus::Executed => executed += 1,
//                         _ => {}
//                     }
//                 }
//             }
        
//             (total, active, executed)
//         }
        
//         #[ink(message)]
//         pub fn get_total_voters(&self) -> u32 {
//             self.total_voters
//         }
        
//         #[ink(message)]
//         pub fn has_reached_quorum(&self, proposal_id: u32) -> bool {
//             if let Some(proposal) = self.proposals.get(proposal_id) {
//                 let quorum_percentage = proposal.governance_params.quorum_threshold.to_percentage();
//                 let required_votes = (self.total_voters as u128 * quorum_percentage as u128) / 100;
//                 let total_votes: u128 = proposal.vote_counts.iter().sum();
                
//                 total_votes >= required_votes
//             } else {
//                 false
//             }
//         }
        
//         #[ink(message)]
//         pub fn get_proposal_results(&self, proposal_id: u32) -> Option<(Vec<u128>, bool)> {
//             if let Some(proposal) = self.proposals.get(proposal_id) {
//                 let reached_quorum = self.has_reached_quorum(proposal_id);
//                 Some((proposal.vote_counts, reached_quorum))
//             } else {
//                 None
//             }
//         }
        
//         #[ink(message)]
//         pub fn get_voting_options(&self, proposal_id: u32) -> Option<Vec<String>> {
//             self.proposals.get(proposal_id)
//                 .map(|p| p.voting_options.options)
//         }
        
//         #[ink(message)]
//         pub fn get_detailed_results(&self, proposal_id: u32) -> Option<Vec<(String, u128)>> {
//             if let Some(proposal) = self.proposals.get(proposal_id) {
//                 let results: Vec<(String, u128)> = proposal.voting_options.options
//                            .iter()
//                            .zip(proposal.vote_counts.iter())
//                            .map(|(option, &count)| (option.clone(), count))
//                            .collect();
//                        Some(results)
//                 } else {
//                 None
//             }
//         }
        
//         #[ink(message)]
//         pub fn get_winning_option(&self, proposal_id: u32) -> Option<(String, u128)> {
//             if let Some(proposal) = self.proposals.get(proposal_id) {
//                 let max_votes = proposal.vote_counts.iter().max().copied()?;
//                 let winner_idx = proposal.vote_counts.iter().position(|&v| v == max_votes)?;
//                 let option_name = proposal.voting_options.options.get(winner_idx)?.clone();
//                 Some((option_name, max_votes))
//             } else {
//                 None
//             }
//         }
//     }
    
//     #[cfg(test)]
//     mod tests {
//         use super::*;
    
//         #[ink::test]
//         fn test_initialization() {
//             let contract = TreasuryGovernance::new();
//             assert_eq!(contract.next_proposal_id, 1);
//             assert_eq!(contract.total_voters, 0);
//             assert_eq!(contract.get_all_proposal_ids().len(), 0);
//         }
        
//         #[ink::test]
//         fn test_create_proposal() {
//             let mut contract = TreasuryGovernance::new();
            
//             let governance_params = GovernanceParameters {
//                 voting_period: VotingPeriod::SevenDays,
//                 quorum_threshold: QuorumThreshold::Ten,
//                 execution_delay: ExecutionDelay::OneDay,
//             };
    
//             let voting_options = VotingOptions {
//                 options: vec![
//                     String::from("Approve"),
//                     String::from("Reject"),
//                 ],
//             };
    
//             let result = contract.create_proposal(
//                 String::from("Test Proposal"),
//                 String::from("A test proposal"),
//                 ProposalType::Treasury,
//                 governance_params,
//                 voting_options,
//             );
        
//             assert!(result.is_ok());
//             assert_eq!(result.unwrap(), 1);
//             assert_eq!(contract.get_all_proposal_ids().len(), 1);
//         }
        
//         #[ink::test]
//         fn test_invalid_proposal_empty_options() {
//             let mut contract = TreasuryGovernance::new();
                    
//             let governance_params = GovernanceParameters {
//                 voting_period: VotingPeriod::ThreeDays,
//                 quorum_threshold: QuorumThreshold::Five,
//                 execution_delay: ExecutionDelay::Immediately,
//             };
        
//             let voting_options = VotingOptions {
//                 options: Vec::new(),
//             };
        
//             let result = contract.create_proposal(
//                 String::from("Invalid Proposal"),
//                 String::from("No options"),
//                 ProposalType::Other,
//                 governance_params,
//                 voting_options,
//             );
        
//             assert_eq!(result, Err(Error::InvalidProposal));
//         }
        
//         #[ink::test]
//         fn test_voting() {
//             let mut contract = TreasuryGovernance::new();
            
//             // Register voter
//             contract.register_voter();
    
//             // Create proposal
//             let governance_params = GovernanceParameters {
//                 voting_period: VotingPeriod::SevenDays,
//                 quorum_threshold: QuorumThreshold::Ten,
//                 execution_delay: ExecutionDelay::OneDay,
//             };
    
//             let voting_options = VotingOptions {
//                 options: vec![
//                     String::from("Yes"),
//                     String::from("No"),
//                 ],
//             };
       
//             let proposal_id = contract.create_proposal(
//                 String::from("Vote Test"),
//                 String::from("Testing voting"),
//                 ProposalType::Governance,
//                 governance_params,
//                 voting_options,
//             ).unwrap();
       
//             // Vote
//             let result = contract.vote(proposal_id, 0);
//             assert!(result.is_ok());
       
//             // Check vote was recorded
//             let proposal = contract.get_proposal(proposal_id).unwrap();
//             assert_eq!(proposal.vote_counts[0], 1);
//             assert_eq!(proposal.total_voters, 1);
//         }

//     }
// }
