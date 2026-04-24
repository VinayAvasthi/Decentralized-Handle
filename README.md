# Stellar Username Registry 

## Project Description
The Stellar Username Registry is a lightweight, decentralized smart contract built on the Stellar network using the Soroban smart contract platform. It serves as a foundational naming service, allowing users to map human-readable usernames to their standard Stellar public addresses. This simplifies transactions, identity verification, and social interactions within the Stellar ecosystem by replacing complex alphanumeric strings with memorable aliases.

## What it does
This smart contract provides a secure, immutable ledger for claiming and storing usernames. When a user submits a transaction to register a username, the contract verifies the cryptographic signature of the requesting Stellar address. It then checks the network's persistent storage to ensure the desired username is currently available. If the username is free, the contract permanently maps the username to the user's address. Anyone on the network can then query the contract with a specific username to instantly retrieve the associated Stellar address.

## Features
* **Human-Readable Identity:** Allows users to link their Stellar to a simple, readable .
* **Cryptographic Authorization:** Utilizes Soroban's native function to guarantee that only the true owner of a Stellar address can bind it to a username.
* **Collision Prevention:** Implements storage checks to ensure that once a username is registered, it cannot be overwritten or claimed by another user.
* **Persistent Storage:** Leverages Soroban's persistent state to securely store registry mappings for the long term.
* **Simple Lookup Method:** Provides a public read function to easily resolve usernames back to their respective Stellar addresses for routing payments or verifying identity.
wallet address: GD3MYM73QFDPMLJQULU25AT27USOT255RJTNNGXPYLJK6LDFBQVIXIS6

contract address: CANQPPYZJS7NH6TZG5TPMP3EENHC2CHZ6TBMRSXNRCI6MVPW4IZ2KSTC

https://stellar.expert/explorer/testnet/contract/CANQPPYZJS7NH6TZG5TPMP3EENHC2CHZ6TBMRSXNRCI6MVPW4IZ2KSTC

<img width="1918" height="1198" alt="image" src="https://github.com/user-attachments/assets/0e92a205-54fc-41c1-b4c9-046018b66e54" />
