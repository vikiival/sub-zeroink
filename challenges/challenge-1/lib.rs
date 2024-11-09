#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 1: Basics of ink! and setting up a DAO contract
//
// - **Difficulty**: Easy
// - **Submission Criteria:** ink! contract must
//     - Have a constructor accepting a name parameter.
//     - Have a storage field for the DAO name.
//     - Implement the provided methods.
//     - Unit test for constructor and setting DAO name.
//     - Be built and deployed on Pop Network testnet.
// - **Submission Guidelines:**
//     - Verify with R0GUE DevRel, and post on X.
// - **Prize:** sub0 merch

#[ink::contract]
mod dao {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Dao {
        name: String,
    }

    impl Dao {
        // Constructor that initializes the values for the contract.
        #[ink(constructor)]
        pub fn new(name: String) -> Self {
            Self { name }
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
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::dao::Dao;

        #[ink::test]
        fn test_name() {
            let dao = Dao::new(String::from("any name"));
            assert_eq!(dao.name, dao.get_name());
            assert_eq!(dao.get_name(), String::from("any name"));
        }
    }
}
