#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct UsernameRegistry;

#[contractimpl]
impl UsernameRegistry {
    /// Registers a unique username to a Stellar address.
    /// 
    /// # Arguments
    /// * `env` - The environment interface.
    /// * `user` - The Stellar address claiming the username.
    /// * `username` - The desired username (represented as a Symbol).
    pub fn register(env: Env, user: Address, username: Symbol) {
        // Ensure the transaction is authorized by the user's address
        user.require_auth();

        // Check if the username is already registered in persistent storage
        if env.storage().persistent().has(&username) {
            panic!("Error: Username is already taken.");
        }

        // Save the mapping of the username to the user's address
        env.storage().persistent().set(&username, &user);
    }

    /// Retrieves the Stellar address associated with a given username.
    /// 
    /// # Arguments
    /// * `env` - The environment interface.
    /// * `username` - The username to look up.
    /// 
    /// # Returns
    /// * `Option<Address>` - The associated address if found, otherwise None.
    pub fn get_address(env: Env, username: Symbol) -> Option<Address> {
        env.storage().persistent().get(&username)
    }
}