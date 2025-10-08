#[derive(Debug, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error{
    InvalidProposal,
    ProposalNotFound,
    ProposalNotReadyForExecution,
    ProposalNotActive,
    VotingPeriodEnded,
    AlreadyVoted
}