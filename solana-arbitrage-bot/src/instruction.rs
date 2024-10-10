use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ArbitrageInstruction {
    /// Initialize a liquidity pool
    /// Accounts expected:
    /// 0. `[signer]` The account initializing the pool
    /// 1. `[writable]` The liquidity pool account
    /// 2. `[]` The token mint account
    InitializePool { amount: u64 },

    /// Execute an arbitrage trade between two pools
    /// Accounts expected:
    /// 0. `[signer]` The trader
    /// 1. `[writable]` Source pool account
    /// 2. `[writable]` Destination pool account
    /// 3. `[]` Token program ID
    ExecuteArbitrage { amount: u64 },

    /// Rebalance liquidity pools
    /// Accounts expected:
    /// 0. `[signer]` The manager account
    /// 1. `[writable]` Source pool account
    /// 2. `[writable]` Destination pool account
    RebalancePools { source_pool: Pubkey, destination_pool: Pubkey, amount: u64 },
}

impl ArbitrageInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        ArbitrageInstruction::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }
}
