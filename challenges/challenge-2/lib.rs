#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 2: Implement voter registration, proposal management, and voting in your Dao.
//
// - **Difficulty**: Mid
// - **Submission Criteria:** ink! contract must
//     - Use a storage-optimized data-structure `Mapping` or `StorageVec`
//     - Store registered members, member votes, and proposals to vote on.
//     - Implement methods to register and de-register members.
//     - Implement methods to create proposals and a method to vote on proposals.
//     - Unit tests for adding members, votes, and proposals.
// - **Submission Guidelines:**
//     - Verify with R0GUE DevRel, and post on X.
// - **Prize:** sub0 merch

#[ink::contract]
mod dao {
    use ink::{
        prelude::string::String,
        storage::{Mapping, StorageVec},
    };
    use minidao_common::*;

    #[derive(Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct BasicProposal {
        pub vote_count: u32,
    }

    #[ink(storage)]
    pub struct Dao {
        name: String,
        voters: Mapping<AccountId, u32>,
        proposals: StorageVec<BasicProposal>,
    }

    impl Dao {
        // Constructor that initializes the values for the contract.
        #[ink(constructor)]
        pub fn new(name: String) -> Self {
            Self { 
                name,
                voters: Mapping::new(),
                proposals: StorageVec::new(),
             }
        }

        // Constructor that initializes the default values for the contract.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        #[ink(message)]
        pub fn register_voter(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterAlreadyRegistered` if the voter is registered

            let caller = self.env().caller();
            // todo: false
            let voter = self.voters.get(&caller);

            if voter.is_some() {
                return Err(DaoError::VoterAlreadyRegistered);
            }

            // - Success: Register a new `voter` to the Dao

            self.voters.insert(caller, &0);
            
            Ok(())
        }

        #[ink(message)]
        pub fn deregister_voter(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Deregister a new `voter` from the Dao
            let caller = self.env().caller();
            self.voters.get(&caller).ok_or(DaoError::VoterNotRegistered)?;


            self.voters.remove(caller);
            Ok(())
        }

        #[ink(message)]
        pub fn has_voter(&self, voter: AccountId) -> bool {
            // - Success: Return if the voter is registered.
            self.voters.contains(&voter)
        }

        #[ink(message)]
        pub fn create_proposal(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Create a new proposal that stores `votes` from `voters`
            let caller = self.env().caller();
            self.voters.get(&caller).ok_or(DaoError::VoterNotRegistered)?;
            self.proposals.push(&BasicProposal { vote_count: 0 });
            
            Ok(())
        }

        #[ink(message)]
        pub fn remove_proposal(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Error: Throw error `DaoError::ProposalDoesNotExist` if the proposal is not created
            // - Success: Create a new proposal that stores `votes` from `voters`
            let caller = self.env().caller();
            self.voters.get(&caller).ok_or(DaoError::VoterNotRegistered)?;

            if self.proposals.len() <= proposal_id {
                return Err(DaoError::ProposalDoesNotExist);
            }

            self.proposals.get(
                proposal_id
            ).ok_or(DaoError::ProposalDoesNotExist)?;

            self.proposals.clear_at(proposal_id);



            Ok(())
        }

        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u32) -> Option<BasicProposal> {
            let proposal = self.proposals.get(
                proposal_id
            );
            proposal.clone()
        }

        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Error: Throw error `Error::ProposalDoesNotExist` if the proposal is not created
            // - Success: Vote on the proposal
            let caller = self.env().caller();
            let voter = self.voters.get(&caller).ok_or(DaoError::VoterNotRegistered)?;

            if self.proposals.len() <= proposal_id {
                return Err(DaoError::ProposalDoesNotExist);
            }

            let mut proposal = self.proposals.get(
                proposal_id
            ).ok_or(DaoError::ProposalDoesNotExist)?;


            proposal.vote_count += 1;
            self.proposals.set(proposal_id, &proposal);
            self.voters.insert(&caller, &(voter + 1));
            Ok(())
        }

        #[ink(message)]
        pub fn vote_count(&self, voter: AccountId) -> u32 {
            self.voters.get(&voter).unwrap_or(0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::dao::Dao;

        #[ink::test]
        fn test_voter_registration() {
            let mut dao = Dao::new(String::from("any name"));
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(dao.register_voter(), Ok(()));
        
            assert_eq!(dao.has_voter(accounts.alice), true);
        }

        #[ink::test]
        fn test_proposal_management() {
            let mut dao = Dao::new(String::from("any name"));
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(dao.register_voter(), Ok(()));
        
            assert_eq!(dao.has_voter(accounts.alice), true);

            assert_eq!(dao.create_proposal(), Ok(()));

            assert_eq!(dao.get_proposal(0).unwrap().vote_count, 0);
        }

        #[ink::test]
        fn test_vote() {
            let mut dao = Dao::new(String::from("any name"));
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(dao.register_voter(), Ok(()));
        
            assert_eq!(dao.has_voter(accounts.alice), true);

            assert_eq!(dao.create_proposal(), Ok(()));

            assert_eq!(dao.vote(0), Ok(()));

            assert_eq!(dao.vote_count(accounts.alice), 1);
            assert_eq!(dao.get_proposal(0).unwrap().vote_count, 1);
            
        }
    }
}
