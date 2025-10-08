
#[cfg(test)]
mod tests {
    use ink::env::test;
    use crate::contract::treasury_governance::*;
    use ink::H160;
    use crate::enums::*;
    use crate::storage::*;
    use crate::errors::Error;

    #[ink::test]
    fn test_initialization() {
        let contract = TreasuryGovernance::new();
        assert_eq!(contract.next_proposal_id, 1);
        assert_eq!(contract.total_voters, 0);
        assert_eq!(contract.get_all_proposal_ids().len(), 0);
    }
    
    #[ink::test]
    fn test_create_proposal() {
        let mut contract = TreasuryGovernance::new();
        
        let governance_params = GovernanceParameters {
            voting_period: VotingPeriod::SevenDays,
            quorum_threshold: QuorumThreshold::Ten,
            execution_delay: ExecutionDelay::OneDay,
        };
    
        let voting_options = VotingOptions {
            options: vec![
                String::from("Approve"),
                String::from("Reject"),
            ],
        };
    
        let result = contract.create_proposal(
            String::from("Test Proposal"),
            String::from("A test proposal"),
            ProposalType::Treasury,
            governance_params,
            voting_options,
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        assert_eq!(contract.get_all_proposal_ids().len(), 1);
    }
        
    #[ink::test]
    fn test_invalid_proposal_empty_options() {
        let mut contract = TreasuryGovernance::new();
                    
        let governance_params = GovernanceParameters {
            voting_period: VotingPeriod::ThreeDays,
            quorum_threshold: QuorumThreshold::Five,
            execution_delay: ExecutionDelay::Immediately,
        };
        
        let voting_options = VotingOptions {
            options: Vec::new(),
        };
        
        let result = contract.create_proposal(
            String::from("Invalid Proposal"),
            String::from("No options"),
            ProposalType::Other,
            governance_params,
            voting_options,
        );
        
        assert_eq!(result, Err(Error::InvalidProposal));
    }
        
    fn set_caller(account: H160) {
        test::set_caller(account);
    }
       
    fn default_caller() -> H160 {
        H160::from_low_u64_be(1)
    }
    
    #[ink::test]
    fn test_voting() {
        let mut contract = TreasuryGovernance::new();
       
        // Register a voter (Alice)
        let alice = test::default_accounts().alice;
        set_caller(alice);
        contract.register_voter();
        let bob = test::default_accounts().bob;
        set_caller(bob);
        contract.register_voter();
               
        // Create proposal
        let governance_params = GovernanceParameters {
            voting_period: VotingPeriod::SevenDays,
            quorum_threshold: QuorumThreshold::Ten,
            execution_delay: ExecutionDelay::OneDay,
        };
       
        let voting_options = VotingOptions {
            options: vec![String::from("Yes"), String::from("No")],
        };
       
        let proposal_id = contract
            .create_proposal(
                String::from("Vote Test"),
                String::from("Testing voting"),
                ProposalType::Governance,
                governance_params,
                voting_options,
            )
            .unwrap();
       
            // Alice votes "No"
            set_caller(alice);
            let result = contract.vote(proposal_id, 1);
            assert!(result.is_ok());
       
            // Check proposal updates
            let proposal = contract.get_proposal(proposal_id).unwrap();
            assert_eq!(proposal.vote_counts[0], 0); // "Yes"
            assert_eq!(proposal.vote_counts[1], 1); // "No"
            assert_eq!(proposal.total_voters, 1);
       
            // Verify stored vote under Alice’s account
            let user_vote = contract.get_user_vote(proposal_id, alice).unwrap();
            assert_eq!(user_vote.choice.option_index, 1);
            assert_eq!(user_vote.choice.option_text, "No");
    }

        
    #[ink::test]
    fn test_double_voting_prevention() {
        let mut contract = TreasuryGovernance::new();
        contract.register_voter();

        let governance_params = GovernanceParameters {
            voting_period: VotingPeriod::ThreeDays,
            quorum_threshold: QuorumThreshold::Five,
            execution_delay: ExecutionDelay::Immediately,
        };

        let voting_options = VotingOptions {
            options: vec![String::from("Yes"), String::from("No")],
        };

        let proposal_id = contract.create_proposal(
            String::from("Double Vote Test"),
            String::from("Testing double voting"),
            ProposalType::Technical,
            governance_params,
            voting_options,
        ).unwrap();

        // First vote should succeed
        assert!(contract.vote(proposal_id, 0).is_ok());

        // Second vote should fail
        assert_eq!(contract.vote(proposal_id, 1), Err(Error::AlreadyVoted));
    }

    #[ink::test]
    fn test_voting_period_conversion() {
        assert_eq!(VotingPeriod::ThreeDays.to_blocks(), 43_200);
        assert_eq!(VotingPeriod::SevenDays.to_blocks(), 100_800);
        assert_eq!(VotingPeriod::FourteenDays.to_blocks(), 201_600);
        assert_eq!(VotingPeriod::ThirtyDays.to_blocks(), 432_000);
    }

    #[ink::test]
    fn test_quorum_percentage() {
        assert_eq!(QuorumThreshold::Five.to_percentage(), 5);
        assert_eq!(QuorumThreshold::Ten.to_percentage(), 10);
        assert_eq!(QuorumThreshold::Twenty.to_percentage(), 20);
        assert_eq!(QuorumThreshold::TwentyFive.to_percentage(), 25);
    }

    #[ink::test]
    fn test_execution_delay() {
        assert_eq!(ExecutionDelay::Immediately.to_blocks(), 0);
        assert_eq!(ExecutionDelay::OneDay.to_blocks(), 14_400);
        assert_eq!(ExecutionDelay::TwoDays.to_blocks(), 28_800);
        assert_eq!(ExecutionDelay::SevenDays.to_blocks(), 100_800);
    }
}
