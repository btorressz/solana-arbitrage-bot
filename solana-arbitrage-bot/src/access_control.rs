use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

pub fn assert_owner(account: &AccountInfo, expected_owner: &Pubkey) -> Result<(), ProgramError> {
    if account.owner != expected_owner {
        Err(ProgramError::IllegalOwner)
    } else {
        Ok(())
    }
}
