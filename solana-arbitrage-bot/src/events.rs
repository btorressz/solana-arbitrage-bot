use solana_program::msg;

/// Log a message indicating that a pool has been successfully initialized.
pub fn log_pool_initialized(pool_pubkey: &str, amount: u64) {
    msg!("Pool initialized: {}, Amount: {}", pool_pubkey, amount);
}

/// Log a message indicating that an arbitrage was successfully executed.
pub fn log_arbitrage_executed(source_pool: &str, dest_pool: &str, amount: u64) {
    msg!(
        "Arbitrage executed from {} to {} with amount: {}",
        source_pool,
        dest_pool,
        amount
    );
}
