#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 7 (Bonus): Use the Pop API to create a fungibles token for token-backed voting
//
// - **Difficulty:** Mid
// - **Submission Criteria:** DAO contract must
//   - Use the fungibles Pop API to create a new asset.
//   - Mint the asset for newly registered voter.
//   - Use the asset for token-backed voting by creating a new storage item to track the `Prevote` of each Superdao `Proposal`.
//   - Registered voter in the Dao will use the minted tokens to vote on the `Prevote`.
//   - If number of approvals in the `Prevote` is more than the disapprovals after the `deadline`, submit the vote to the proposal on Superdao.
// - **Submission Guidelines:** Verify with R0GUE DevRel, post on X with GitHub link
// - **Prize:** Pop ring candy

#[ink::contract]
mod dao {
    use ink::{
        contract_ref,
        prelude::{string::String, vec::Vec},
        storage::{Mapping, StorageVec},
        xcm::prelude::*,
    };
    use minidao_common::*;
    use pop_api::v0::fungibles::traits::Psp22;
    use superdao_traits::{Call, ChainCall, ContractCall, SuperDao, Vote};

    #[derive(Clone, Default)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Prevote {
        pub deadline: BlockNumber,
        pub aye_votes: Vec<(AccountId, Balance)>,
        pub nay_votes: Vec<(AccountId, Balance)>,
    }

    #[ink(storage)]
    pub struct Dao {
        name: String,
        prevotes: Mapping<u32, Prevote>,
        voters: StorageVec<AccountId>,
        token: AccountId,
        superdao: contract_ref!(SuperDao),
    }

    impl Dao {
        // Constructor that initializes the values for the contract.
        #[ink(constructor)]
        pub fn new(name: String, superdao: AccountId, token: AccountId) -> Self {
            // Register your Dao as a member of the Superdao.
            let mut instance = Self {
                name,
                token,
                superdao: superdao.into(),
                voters: StorageVec::new(),
                prevotes: Mapping::new(),
            };
            instance
        }

        #[ink(message)]
        pub fn name(&self) -> String {
            // - Returns the name of the Dao
            todo!()
        }

        #[ink(message)]
        pub fn register_voter(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterAlreadyRegistered` if the voter is registered
            // - Success: Register a new `voter` to the Dao
            Ok(())
        }

        #[ink(message)]
        pub fn deregister_voter(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Deregister a new `voter` from the Dao
            Ok(())
        }

        #[ink(message)]
        pub fn has_voter(&self, voter: AccountId) -> bool {
            todo!()
        }

        #[ink(message)]
        pub fn create_superdao_cross_chain_proposal(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Create a SuperDao proposal to execute a cross-chain message.
            Ok(())
        }

        #[ink(message)]
        pub fn create_contract_call_proposal(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Create a SuperDao proposal to call a contract method.
            Ok(())
        }

        #[ink(message)]
        pub fn submit_prevote(&mut self, proposal_id: u32, approved: bool) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Dao member prevote is recoreded with the current balance of the voter.
            Ok(())
        }

        #[ink(message)]
        pub fn vote_proposal(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Error: Throw error `DaoError::ProposalDoesNotExist` if the proposal does not found.
            Ok(())
        }
    }
}
