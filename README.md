<div align="center">

# Sub0 `ink!` Challenges

ink! programming language challenges made for the Sub0 Reset Bangkok

</p>

<img height="70px" alt="Polkadot SDK Logo" src="https://github.com/user-attachments/assets/c60b6b92-a263-480c-b297-3535454ad3f6"/>

</div>

## Getting started

Before tackling the challenges, you need to setup your local environment for ink! contract development first.

- Install `pop-cli`: https://learn.onpop.io/cli/installing-pop-cli/

## About the challenges

To tackle the challenges, first fork and clone the repository

```
git clone https://github.com/r0gue-io/sub0-ink-challenges.git
```

To test your contract, run:

```
cargo test
```

<br/>

<div align="center">

# 🏆 CHALLENGES

| ✒️ Challenges | Description                                                          | Link to the challenge                       |
| ------------- | -------------------------------------------------------------------- | ------------------------------------------- |
| Challenge 1   | Basics of ink! and setting up a DAO contract                         | [Take challenge](./challenges/challenge-1/) |
| Challenge 2   | Membership and voting mechanism to the DAO.                          | [Take challenge](./challenges/challenge-2/) |
| Challenge 3   | Connect your DAO to the Super DAO with registration and voting       | [Take challenge](./challenges/challenge-3/) |
| Challenge 4   | Support creating cross-chain proposals to the Super DAO              | [Take challenge](./challenges/challenge-4/) |
| Challenge 5   | Build a UI for your DAO                                              | [Take challenge](./challenges/challenge-5/) |
| Challenge 6   | Ideate and design a multichain, permissionless solution for Polkadot | [Take challenge](./challenges/challenge-6/) |
| Challenge 7   | Use the Pop API to create a fungibles token for token-backed voting  | [Take challenge](./challenges/challenge-7/) |

Superdao Contract Address on Pop Network
```
12oasvoSoRKxQ7ABrjgCftm78g4wvfyLQ33prWK69htwNsc7
```

</div>

<br/>

<div align="center">
  
<img width="912" alt="Screenshot 2024-11-07 at 18 24 10" src="https://github.com/user-attachments/assets/bb40edbd-de7c-4acc-a88b-59c2013fd565">

</div>

<br/>

## 🍭 Challenge 1: Basics of ink! and setting up a DAO contract

- **Difficulty**: Easy
- **Submission Criteria:** ink! contract must
  - Have a constructor accepting a name parameter.
  - Have a storage field for the DAO name.
  - Implement the provided methods.
  - Unit test for constructor and setting DAO name.
  - Be built and deployed on Pop Network testnet.
- **Submission Guidelines:**
  - Verify with R0GUE DevRel, and post on X.
- **Prize:** sub0 merch

## 🍭 Challenge 2: Membership and voting mechanism to the DAO.

- **Difficulty**: Mid
- **Submission Criteria:** ink! contract must
  - Use a storage-optimized data-structure `Mapping` or `StorageVec`
  - Store registered members, member votes, and proposals to vote on.
  - Implement methods to register and de-register members.
  - Implement methods to create proposals and a method to vote on proposals.
  - Unit tests for adding members, votes, and proposals.
- **Submission Guidelines:**
  - Verify with R0GUE DevRel, and post on X.
- **Prize:** sub0 merch

## 🍭 Challenge 3: Connect your DAO to the Super DAO with registration and voting

- **Difficulty**: Mid
- **Submission Criteria:** ink! contract must
  - Import the Super DAO trait>
  - Store Super DAO contract address.
  - Register contract as member of Super DAO - using trait-based contract calling.
  - Vote on proposals in the Super DAO - using trait-based contract calling.
  - Create proposals to call another contract - using trait-based contract calling.
  - E2E test for cross-contract call.
- **Submission Guidelines:**
  - Verify with R0GUE DevRel, and post on X.
- **Prize:** Sub0 Merch & ink! sports towel

## 🍭 Challenge 4: Support creating cross-chain proposals to the Super DAO

- **Difficulty**: Advanced
- **Submission Criteria:** ink! contract must
  - Support creating cross-chain proposals to the Super DAO (XCM)
  - A deployed contract on Pop Network Testnet
  - Have a cross-chain proposal successfully executed
- **Submission Guidelines:**
  - Verify with R0GUE DevRel, and post on X.
- **Prize:** Sub0 merch

## 🍭 Challenge 5: Build a UI for your DAO

- **Difficulty:** Mid
- **Submission Criteria:** The UI must support
  - Registering & viewing members
  - Voting on and viewing proposals
  - Viewing overall proposal votes
- **Submission Guidelines:** Verify with R0GUE or Dedot DevRel, and post on X
- **Prize:** Sub0 merch & ink! sports towel

## 🍭 Challenge 6: Ideate and design a multichain, permissionless solution for Polkadot

- **Difficulty:** Easy
- **Submission Criteria:** A PDF containing
  - A write-up of their proposal
  - What their idea solves, how it works, and team members (if applicable)
- **Submission Guidelines:** Verify with R0GUE DevRel

## 🍭 Challenge 7 (Bonus): Use the Pop API to create a fungibles token for token-backed voting

- **Difficulty:** Mid
- **Submission Criteria:** DAO contract must
  - Store PSP22 token contract address.
  - Use the fungibles Pop API to create a new asset.
  - Mint the asset for newly registered voter.
  - Use the asset for token-backed voting by creating a new storage item to track the `Prevote` of each Superdao `Proposal`.
  - Registered voter in the Dao will use the minted tokens to vote on the `Prevote`.
  - If number of approvals in the `Prevote` is more than the disapprovals after the `deadline`, submit the vote to the proposal on Superdao.
- **Submission Guidelines:** Verify with R0GUE DevRel, post on X with GitHub link
- **Prize:** Pop ring candy

# Support

Be part of our passionate community of Web3 builders. [Join our Telegram](https://t.me/onpopio)!

Feel free to raise issues if anything is unclear, have ideas or want to contribute to Pop!

For any questions related to ink! you can also go to [Polkadot Stack Exchange](https://polkadot.stackexchange.com/) or
ask the [ink! community](https://t.me/inkathon/1).
