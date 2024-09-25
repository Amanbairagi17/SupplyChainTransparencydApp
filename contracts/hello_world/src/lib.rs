#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// A structure to track each movement or transaction in the supply chain.
#[contracttype]
#[derive(Clone)]
pub struct Movement {
    pub transaction_id: u64,  // Unique transaction ID
    pub item: String,         // Item being moved
    pub from: String,         // Source of the item
    pub to: String,           // Destination of the item
    pub timestamp: u64,       // Time of the transaction
    pub verified: bool,       // Whether the transaction has been verified
}

// A structure to track overall statistics for the supply chain
#[contracttype]
#[derive(Clone)]
pub struct SupplyChainStats {
    pub total_transactions: u64,  // Total number of transactions
    pub verified_transactions: u64, // Total number of verified transactions
    pub unverified_transactions: u64, // Total number of unverified transactions
}

// Mapping unique transaction ID to its respective Movement struct
#[contracttype]
pub enum TransactionBook {
    Transaction(u64),
}

// For referencing the overall statistics
const STATS: Symbol = symbol_short!("STATS");

// Unique ID for tracking new transactions
const TRANSACTION_COUNT: Symbol = symbol_short!("T_COUNT");

#[contract]
pub struct SupplyChainContract;

#[contractimpl]
impl SupplyChainContract {
    // Function to create and record a new transaction in the supply chain
    pub fn record_movement(env: Env, item: String, from: String, to: String) -> u64 {
        let mut transaction_count: u64 = env.storage().instance().get(&TRANSACTION_COUNT).unwrap_or(0);
        transaction_count += 1;

        let timestamp = env.ledger().timestamp(); // Get current timestamp

        let mut stats = Self::view_supply_chain_stats(env.clone());

        // Create a new movement
        let movement = Movement {
            transaction_id: transaction_count,
            item,
            from,
            to,
            timestamp,
            verified: false, // Initially, the transaction is unverified
        };

        // Store the new movement
        env.storage().instance().set(&TransactionBook::Transaction(transaction_count), &movement);

        // Update the supply chain stats
        stats.total_transactions += 1;
        stats.unverified_transactions += 1;

        env.storage().instance().set(&STATS, &stats);
        env.storage().instance().set(&TRANSACTION_COUNT, &transaction_count);

        log!(&env, "Recorded new movement with Transaction ID: {}", transaction_count);

        transaction_count // Return the transaction ID
    }

    // Function for verifying a specific transaction
    pub fn verify_transaction(env: Env, transaction_id: u64) {
        let mut movement = Self::view_movement(env.clone(), transaction_id);

        if movement.verified == false {
            movement.verified = true; // Mark the transaction as verified

            let mut stats = Self::view_supply_chain_stats(env.clone());
            stats.verified_transactions += 1;
            stats.unverified_transactions -= 1;

            // Store updated transaction and stats
            env.storage().instance().set(&TransactionBook::Transaction(transaction_id), &movement);
            env.storage().instance().set(&STATS, &stats);

            log!(&env, "Transaction ID: {} has been verified", transaction_id);
        } else {
            log!(&env, "Transaction ID: {} is already verified", transaction_id);
        }
    }

    // Function to retrieve details of a specific movement by its transaction ID
    pub fn view_movement(env: Env, transaction_id: u64) -> Movement {
        env.storage().instance().get(&TransactionBook::Transaction(transaction_id)).unwrap_or(Movement {
            transaction_id: 0,
            item: String::from_str(&env, "Not Found"),
            from: String::from_str(&env, "Unknown"),
            to: String::from_str(&env, "Unknown"),
            timestamp: 0,
            verified: false,
        })
    }

    // Function to retrieve the overall supply chain statistics
    pub fn view_supply_chain_stats(env: Env) -> SupplyChainStats {
        env.storage().instance().get(&STATS).unwrap_or(SupplyChainStats {
            total_transactions: 0,
            verified_transactions: 0,
            unverified_transactions: 0,
        })
    }
}
