use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Debug, Error)]
pub enum ArbitrageError {
    #[error("Insufficient liquidity in the pool")]
    InsufficientLiquidity,
    #[error("Invalid token mint")]
    InvalidTokenMint,
    #[error("Pool is not initialized")]
    PoolNotInitialized,
}

impl From<ArbitrageError> for ProgramError {
    fn from(e: ArbitrageError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
