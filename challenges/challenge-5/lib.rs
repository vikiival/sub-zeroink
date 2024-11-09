#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 5: Build a UI for your DAO
//
// NOTE: Using this contract to combine the functionalities of challenge 4 contract and challenge 3 contract. Compile and deploy on Pop Network then building a UI for it.
//
// - **Difficulty:** Mid
// - **Submission Criteria:** The UI must support
//     - Registering & viewing members
//     - Voting on and viewing proposals
//     - Viewing overall proposal votes
// - **Submission Guidelines:** Verify with R0GUE or Dedot DevRel, and post on X
// - **Prize:** Sub0 merch & ink! sports towel

#[ink::contract]
mod dao {
    use ink::{
        contract_ref,
        prelude::{string::String, vec},
        storage::StorageVec,
        xcm::prelude::*,
    };
    use minidao_common::*;
    use superdao_traits::{Call, ChainCall, ContractCall, SuperDao, Vote};

    #[ink(storage)]
    pub struct Dao {
        superdao: contract_ref!(SuperDao),
        voters: StorageVec<AccountId>,
        name: String,
    }

    impl Dao {
        // Constructor that initializes the values for the contract.
        #[ink(constructor)]
        pub fn new(name: String, superdao: AccountId) -> Self {
            // Register your Dao as a member of the Superdao.
            let mut instance = Self {
                name,
                superdao: superdao.into(),
                voters: StorageVec::new(),
            };
            instance
        }

        #[ink(message)]
        pub fn get_name(&self) -> String {
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
            todo!();
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
        pub fn vote_proposal(&mut self, proposal_id: u32, vote: bool) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Vote a SuperDao proposal.
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::dao::Dao;

        #[ink::test]
        fn test_vote_superdao_cross_chain_proposal() {
            todo!("Challenge 4");
        }
    }
}
