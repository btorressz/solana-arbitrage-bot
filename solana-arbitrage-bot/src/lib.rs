pub mod entrypoint;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;
pub mod events;
pub mod access_control;

pub use solana_program::{
    account_info::AccountInfo, 
    entrypoint::ProgramResult, 
    pubkey::Pubkey, 
    program_error::ProgramError, 
    msg,
};

pub use spl_token;

// Define a constant for the program ID. 
solana_program::declare_id!("BmNiubPYS5fmS1ViSa3teqByGcASdBGndYHz9qnxT8fK");

/// Custom error type for the program
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomError {
    /// Invalid pool state detected
    InvalidPoolState,
    /// Insufficient liquidity for trade
    InsufficientLiquidity,
    // Other errors can be added here
}

impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        match e {
            CustomError::InvalidPoolState => ProgramError::Custom(0), // Custom error code
            CustomError::InsufficientLiquidity => ProgramError::Custom(1),
        }
    }
}

#[cfg(test)]
mod tests {
    // Unit tests for the program logic
    use super::*;

    #[test]
    fn test_custom_error_conversion() {
        let error: ProgramError = CustomError::InvalidPoolState.into();
        assert_eq!(error, ProgramError::Custom(0));
    }
}
