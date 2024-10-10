use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// A struct representing a liquidity pool.
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Eq, Clone)]
pub struct LiquidityPool {
    /// The public key of the token mint for the liquidity pool.
    pub token_mint: Pubkey,
    /// The public key of the token account that holds the liquidity.
    pub token_account: Pubkey,
    /// The total amount of tokens in the pool.
    pub total_tokens: u64,
    /// The owner of the pool who can manage it.
    pub owner: Pubkey,
}

impl LiquidityPool {
    /// Initializes a new `LiquidityPool` instance.
    pub fn new(token_mint: Pubkey, token_account: Pubkey, total_tokens: u64, owner: Pubkey) -> Self {
        Self {
            token_mint,
            token_account,
            total_tokens,
            owner,
        }
    }

    /// Updates the total tokens in the pool.
    pub fn update_total_tokens(&mut self, new_total: u64) {
        self.total_tokens = new_total;
    }
}

/// A struct representing the global state of the program (optional).
/// This can be used to track the overall state of the arbitrage bot, such as the number of pools.
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Eq, Clone)]
pub struct ProgramState {
    /// Number of active liquidity pools
    pub pool_count: u32,
    /// The public keys of all the liquidity pools (optional, for easier management)
    pub pool_pubkeys: Vec<Pubkey>,
}

impl ProgramState {
    /// Creates a new program state.
    pub fn new() -> Self {
        Self {
            pool_count: 0,
            pool_pubkeys: vec![],
        }
    }

    /// Adds a new pool to the program state.
    pub fn add_pool(&mut self, pool_pubkey: Pubkey) {
        if !self.pool_pubkeys.contains(&pool_pubkey) {
            self.pool_pubkeys.push(pool_pubkey);
            self.pool_count += 1;
        }
    }

    /// Removes a pool from the program state by its public key.
    pub fn remove_pool(&mut self, pool_pubkey: Pubkey) {
        if let Some(index) = self.pool_pubkeys.iter().position(|&x| x == pool_pubkey) {
            self.pool_pubkeys.remove(index);
            self.pool_count -= 1;
        }
    }

    /// Checks if a pool exists in the program state.
    pub fn pool_exists(&self, pool_pubkey: &Pubkey) -> bool {
        self.pool_pubkeys.contains(pool_pubkey)
    }

    /// Finds a pool by its public key and returns its index in the vector, if it exists.
    pub fn find_pool_index(&self, pool_pubkey: &Pubkey) -> Option<usize> {
        self.pool_pubkeys.iter().position(|x| x == pool_pubkey)
    }
}
