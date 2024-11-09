#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::env::{DefaultEnvironment, Environment};
use superdao_traits::Error as SuperdaoError;

pub type AccountId = <DefaultEnvironment as Environment>::AccountId;
pub type Balance = <DefaultEnvironment as Environment>::Balance;
pub type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum DaoError {
    // Voter is already registered in the Dao.
    VoterAlreadyRegistered,
    // Voter is not registered yet..
    VoterNotRegistered,
    // Voter already voted the proposal.
    VoterAlreadyVoted,
    // Proposal does not exist in the Dao.
    ProposalDoesNotExist,
    // Prevote period is not ended.
    PrevotePeriodIsNotEnded,
    // No contract address.
    NoContractAddress,
    // Error derived from Superdao contract.
    SuperdaoError(SuperdaoError),
}

impl From<SuperdaoError> for DaoError {
    fn from(error: SuperdaoError) -> Self {
        Self::SuperdaoError(error)
    }
}
