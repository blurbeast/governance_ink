
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
    /// Convert to percentage value
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
    /// Convert to blocks (6-second block time)
    pub fn to_blocks(&self) -> u32 {
        match self {
            ExecutionDelay::Immediately => 0,
            ExecutionDelay::OneDay => 24 * 60 * 10,      // 14,400 blocks
            ExecutionDelay::TwoDays => 2 * 24 * 60 * 10, // 28,800 blocks
            ExecutionDelay::SevenDays => 7 * 24 * 60 * 10, // 100,800 blocks
        }
    }
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