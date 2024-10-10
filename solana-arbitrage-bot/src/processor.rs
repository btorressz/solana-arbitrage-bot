use borsh::BorshSerialize; // Add this line to import BorshSerialize
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::invoke,
    msg,
};

use crate::{
    instruction::ArbitrageInstruction,
    state::LiquidityPool,
    utils::{safe_add, calculate_slippage, check_account_owner},
    events::{log_arbitrage_executed, log_pool_initialized},
    access_control::assert_owner,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Unpack the instruction data into an `ArbitrageInstruction` enum
    let instruction = ArbitrageInstruction::unpack(instruction_data)?;

    match instruction {
        ArbitrageInstruction::InitializePool { amount } => {
            initialize_pool(accounts, amount, program_id)
        }
        ArbitrageInstruction::ExecuteArbitrage { amount } => {
            execute_arbitrage(accounts, amount, program_id)
        }
        ArbitrageInstruction::RebalancePools {
            source_pool,
            destination_pool,
            amount,
        } => rebalance_pools(accounts, source_pool, destination_pool, amount),
    }
}

fn initialize_pool(
    accounts: &[AccountInfo],
    amount: u64,
    program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let pool_account = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;

    // Validate that the initializer is a signer
    if !initializer.is_signer {
        msg!("Initializer must be a signer.");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check that the pool account is owned by the program
    check_account_owner(pool_account, program_id)?;

    // Create a new LiquidityPool state
    let pool_state = LiquidityPool::new(
        *mint_account.key,
        *pool_account.key,
        amount,
        *initializer.key,
    );

    // Save the pool state to the account
    pool_state.serialize(&mut *pool_account.data.borrow_mut())?;

    // Log event
    log_pool_initialized(pool_account.key.to_string().as_str(), amount);
    msg!("Pool initialized with {} tokens.", amount);

    Ok(())
}

fn execute_arbitrage(
    accounts: &[AccountInfo],
    amount: u64,
    program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let trader = next_account_info(account_info_iter)?;
    let source_pool_account = next_account_info(account_info_iter)?;
    let destination_pool_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    // Validate that the trader is a signer
    if !trader.is_signer {
        msg!("Trader must be a signer.");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate that the source and destination pool accounts are owned by the program
    check_account_owner(source_pool_account, program_id)?;
    check_account_owner(destination_pool_account, program_id)?;

    // Perform slippage check
    let slippage = calculate_slippage(amount, source_pool_account.data_len() as u64);
    if slippage > 5 {
        msg!("Slippage too high: {}%", slippage);
        return Err(ProgramError::InvalidArgument);
    }

    // Perform token transfer (real SPL token transfer)
    let transfer_instruction = spl_token::instruction::transfer(
        token_program.key,             // Token program ID
        source_pool_account.key,       // Source account
        destination_pool_account.key,  // Destination account
        trader.key,                    // Authority (trader)
        &[],                           // Signer seeds
        amount,                        // Amount
    )?;

    invoke(
        &transfer_instruction,
        &[
            source_pool_account.clone(),
            destination_pool_account.clone(),
            trader.clone(),
            token_program.clone(),
        ],
    )?;

    // Log the arbitrage execution
    log_arbitrage_executed(
        source_pool_account.key.to_string().as_str(),
        destination_pool_account.key.to_string().as_str(),
        amount,
    );

    msg!("Arbitrage executed successfully.");
    Ok(())
}

fn rebalance_pools(
    accounts: &[AccountInfo],
    source_pool: Pubkey,
    destination_pool: Pubkey,
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let manager = next_account_info(account_info_iter)?;
    let source_pool_account = next_account_info(account_info_iter)?;
    let destination_pool_account = next_account_info(account_info_iter)?;

    // Validate that the manager is a signer
    if !manager.is_signer {
        msg!("Manager must be a signer.");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate that the source pool account is owned by the expected program
    check_account_owner(source_pool_account, &source_pool)?;
    check_account_owner(destination_pool_account, &destination_pool)?;

    // Mock rebalance logic (example)
    msg!(
        "Rebalancing {} tokens from {} to {}",
        amount,
        source_pool_account.key,
        destination_pool_account.key
    );

    // Real implementation would involve token transfer or updating internal state

    msg!("Pools rebalanced successfully.");
    Ok(())
}
