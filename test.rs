#[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut treasury_governance = TreasuryGovernance::new(false);
            assert_eq!(treasury_governance.get(), false);
            treasury_governance.flip();
            assert_eq!(treasury_governance.get(), true);
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = TreasuryGovernanceRef::new(false);
            let contract = client
                .instantiate("treasury_governance", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<TreasuryGovernance>();

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = call_builder.flip();
            let _flip_result = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
    
    
    use ink::storage::Mapping;
        use ink::prelude::vec::Vec;
        use ink::prelude::string::String;
    
        // ============================================================================
        // ENUMS AND DATA STRUCTURES
        // ============================================================================
    
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
            /// Convert voting period to blocks (6-second block time)
            pub fn to_blocks(&self) -> u32 {
                match self {
                    VotingPeriod::ThreeDays => 3 * 24 * 60 * 10,      // 43,200 blocks
                    VotingPeriod::SevenDays => 7 * 24 * 60 * 10,      // 100,800 blocks
                    VotingPeriod::FourteenDays => 14 * 24 * 60 * 10,  // 201,600 blocks
                    VotingPeriod::ThirtyDays => 30 * 24 * 60 * 10,    // 432,000 blocks
                }
            }
        }
        
        
        use ink::storage::Mapping;
            use ink::prelude::vec::Vec;
            use ink::prelude::string::String;
        
            // ============================================================================
            // ENUMS AND DATA STRUCTURES
            // ============================================================================
        
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
                /// Convert voting period to blocks (6-second block time)
                pub fn to_blocks(&self) -> u32 {
                    match self {
                        VotingPeriod::ThreeDays => 3 * 24 * 60 * 10,      // 43,200 blocks
                        VotingPeriod::SevenDays => 7 * 24 * 60 * 10,      // 100,800 blocks
                        VotingPeriod::FourteenDays => 14 * 24 * 60 * 10,  // 201,600 blocks
                        VotingPeriod::ThirtyDays => 30 * 24 * 60 * 10,    // 432,000 blocks
                    }
                }
            }