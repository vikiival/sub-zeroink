#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 3: Connect your DAO to the Super DAO with registration and voting
//
// - **Difficulty**: Mid
// - **Submission Criteria:** ink! contract must
//     - Import the Super DAO trait>
//     - Store Super DAO contract address.
//     - Register contract as member of Super DAO - using trait-based contract calling.
//     - Vote on proposals in the Super DAO - using trait-based contract calling.
//     - Create proposals to call another contract - using trait-based contract calling.
//     - E2E test for cross-contract call.
// - **Submission Guidelines:**
//     - Verify with R0GUE DevRel, and post on X.
// - **Prize:** Sub0 Merch & ink! sports towel

#[ink::contract]
mod dao {
    use ink::{
        contract_ref,
        prelude::{string::String, vec},
        storage::StorageVec,
    };
    use minidao_common::*;
    use superdao_traits::{Call, ContractCall, SuperDao, Vote};

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
            todo!()
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

        #[ink::test]
        fn test_create_superdao_contract_call_proposal() {
            todo!("Challenge 3");
        }

        #[ink::test]
        fn test_vote_superdao_proposal() {
            todo!("Challenge 3");
        }
    }
}
