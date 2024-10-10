use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, msg};

/// Validate that an account is owned by the expected program.
pub fn check_account_owner(account: &AccountInfo, expected_owner: &Pubkey) -> Result<(), ProgramError> {
    if account.owner != expected_owner {
        msg!("Account is not owned by the expected program");
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

/// Perform a safe addition operation.
pub fn safe_add(a: u64, b: u64) -> Result<u64, ProgramError> {
    a.checked_add(b).ok_or(ProgramError::Custom(100)) // Custom error code for overflow
}

/// Perform a safe subtraction operation.
pub fn safe_sub(a: u64, b: u64) -> Result<u64, ProgramError> {
    a.checked_sub(b).ok_or(ProgramError::Custom(101)) // Custom error code for underflow
}

/// Calculate slippage percentage based on the amount being traded and the pool size.
pub fn calculate_slippage(amount: u64, pool_size: u64) -> u64 {
    ((amount as f64 / pool_size as f64) * 100.0) as u64
}

/// Validate that an account is a signer.
pub fn check_signer(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_signer {
        msg!("The account is not a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

/// Validate that an account has the expected key.
pub fn check_account_key(account: &AccountInfo, expected_key: &Pubkey) -> Result<(), ProgramError> {
    if account.key != expected_key {
        msg!("Account key does not match the expected key");
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}
